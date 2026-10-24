use cargo_snippet::snippet;

#[snippet("最小_平均_閉路")]
#[snippet("最小_平均_サイクル")]
#[snippet("MinMeanCycle")]
#[snippet("Karp")]
/// 有向重み付きグラフの**最小平均閉路**（辺の重み和 ÷ 辺数を最小化する閉路の、その比）を返す。
/// 閉路が存在しなければ `None`。重みは負でもよい。`O(V·E)`。
///
/// Karp の定理を用いる。最小費用流の最適性判定や周期最適化などで使う。
fn 最小平均閉路(頂点数: usize, 辺: &[(usize, usize, isize)]) -> Option<f64> {
    if 頂点数 == 0 {
        return None;
    }
    const INF: isize = 1 << 60;
    let v = 頂点数;
    let n = v + 1; // 仮想始点を加えた頂点数
    let 始点 = v;
    // 全辺（元の辺 ＋ 仮想始点からの重み 0 辺）。
    let mut 全辺 = 辺.to_vec();
    for w in 0..v {
        全辺.push((始点, w, 0));
    }
    // d[k][v]: 始点からちょうど k 辺で到達する最小重み。
    let mut d = vec![vec![INF; n]; n + 1];
    d[0][始点] = 0;
    for k in 1..=n {
        // 借用のため前段を読みつつ当段を更新。
        let (前, 当) = d.split_at_mut(k);
        let 前 = &前[k - 1];
        let 当 = &mut 当[0];
        for &(a, b, wt) in &全辺 {
            if 前[a] < INF {
                let nd = 前[a] + wt;
                if nd < 当[b] {
                    当[b] = nd;
                }
            }
        }
    }
    let mut 最小 = f64::INFINITY;
    for t in 0..n {
        if d[n][t] >= INF {
            continue;
        }
        let mut 最大 = f64::NEG_INFINITY;
        for k in 0..n {
            if d[k][t] >= INF {
                continue;
            }
            let 比 = (d[n][t] - d[k][t]) as f64 / (n - k) as f64;
            if 比 > 最大 {
                最大 = 比;
            }
        }
        if 最大 < 最小 {
            最小 = 最大;
        }
    }
    if 最小.is_infinite() {
        None
    } else {
        Some(最小)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 全単純閉路を総当たりして最小平均を求める独立オラクル。
    fn 総当たり(頂点数: usize, 辺: &[(usize, usize, isize)]) -> Option<f64> {
        // 隣接（最小重み）。自己ループも 1 辺の閉路として扱う。
        let mut 重み = vec![vec![None::<isize>; 頂点数]; 頂点数];
        for &(a, b, w) in 辺 {
            重み[a][b] = Some(重み[a][b].map_or(w, |x: isize| x.min(w)));
        }
        let mut 最小 = f64::INFINITY;
        // 各始点 s から、s より小さい頂点を経由しない単純パスで s に戻る閉路を数える。
        for s in 0..頂点数 {
            // 自己ループ。
            if let Some(w) = 重み[s][s] {
                最小 = 最小.min(w as f64);
            }
            // DFS: 現在地, 累積重み, 辺数, 訪問集合。
            let mut スタック = vec![(s, 0isize, 0usize, 1u32 << s)];
            while let Some((u, 和, 数, 訪問)) = スタック.pop() {
                for w in 0..頂点数 {
                    if let Some(wt) = 重み[u][w] {
                        if w == s && 数 + 1 >= 2 {
                            // s に戻る（長さ 2 以上の閉路）。
                            最小 = 最小.min((和 + wt) as f64 / (数 + 1) as f64);
                        } else if w > s && 訪問 & (1 << w) == 0 {
                            スタック.push((w, 和 + wt, 数 + 1, 訪問 | (1 << w)));
                        }
                    }
                }
            }
        }
        if 最小.is_infinite() {
            None
        } else {
            Some(最小)
        }
    }

    #[test]
    fn 既知例() {
        // 0->1->2->0 の重み 1,2,3 → 平均 2。
        let 辺 = vec![(0, 1, 1), (1, 2, 2), (2, 0, 3)];
        assert!((最小平均閉路(3, &辺).unwrap() - 2.0).abs() < 1e-9);
        // 閉路なし（DAG）→ None。
        assert!(最小平均閉路(3, &[(0, 1, 5), (1, 2, 5)]).is_none());
        // 自己ループは長さ 1 の閉路。
        assert!((最小平均閉路(2, &[(0, 0, -4), (0, 1, 1)]).unwrap() + 4.0).abs() < 1e-9);
        // 2 つの閉路のうち平均が小さい方。
        let 辺 = vec![(0, 1, 1), (1, 0, 1), (1, 2, 10), (2, 1, -2)];
        // 0-1: 平均 1、1-2: 平均 4 → 最小 1。
        assert!((最小平均閉路(3, &辺).unwrap() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x77a2_4cd0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..3000 {
            let n = (乱数() as usize % 5) + 2; // 2..=6
            let mut 辺 = Vec::new();
            for a in 0..n {
                for b in 0..n {
                    // 各順序対に確率 1/2 で辺。自己ループも含む。
                    if 乱数() % 2 == 0 {
                        let w = (乱数() % 21) as isize - 10;
                        辺.push((a, b, w));
                    }
                }
            }
            let 速 = 最小平均閉路(n, &辺);
            let 遅 = 総当たり(n, &辺);
            match (速, 遅) {
                (None, None) => {}
                (Some(x), Some(y)) => {
                    assert!((x - y).abs() < 1e-6, "速={x} 遅={y} n={n} 辺={辺:?}");
                }
                _ => panic!("閉路有無が不一致 速={速:?} 遅={遅:?} n={n} 辺={辺:?}"),
            }
        }
    }
}
