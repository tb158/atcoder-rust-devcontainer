use cargo_snippet::snippet;

#[snippet("二重_辺_連結_成分")]
#[snippet("TwoEdgeConnected")]
#[snippet("橋_森")]
#[snippet("BridgeTree")]
#[snippet("二辺_連結_成分_分解")]
/// 二重辺連結成分分解の結果。`成分[v]` は頂点 `v` の属する二重辺連結成分の番号、`成分数` はその個数。
/// `橋` は橋（取り除くと連結成分が増える辺）の一覧、`木` は成分を頂点とする**橋森**（橋でつないだ森）。
struct 二重辺連結成分 {
    /// 頂点 → 成分番号。
    成分: Vec<usize>,
    /// 成分の個数。
    成分数: usize,
    /// 橋の一覧（`(u, v)`、`u` は DFS 木で親側）。
    橋: Vec<(usize, usize)>,
    /// 橋森。`木[c]` は成分 `c` と橋で隣接する成分の一覧。
    木: Vec<Vec<usize>>,
}

#[snippet("二重_辺_連結_成分")]
#[snippet("TwoEdgeConnected")]
#[snippet("橋_森")]
#[snippet("BridgeTree")]
#[snippet("二辺_連結_成分_分解")]
/// 無向グラフを二重辺連結成分（どの 1 辺を取り除いても連結が保たれる極大頂点集合）に分解し、
/// 橋を縮約した**橋森**を返す。`lowlink`（橋・関節点の列挙）とは別で、成分分解と縮約木まで構成する。
///
/// `辺` は多重辺・自己ループを含んでよい（並行辺は橋にならない／自己ループは無視）。計算量 `O(n + 辺数)`。
/// 反復 DFS なので深いグラフでもスタック安全。
fn 二重辺連結成分分解(n: usize, 辺: &[(usize, usize)]) -> 二重辺連結成分 {
    let mut グラフ: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n]; // (相手, 辺番号)
    for (i, &(u, v)) in 辺.iter().enumerate() {
        if u != v {
            グラフ[u].push((v, i));
            グラフ[v].push((u, i));
        }
    }
    let mut 番号 = vec![-1isize; n]; // 発見順
    let mut low = vec![0isize; n];
    let mut 橋辺 = vec![false; 辺.len()];
    let mut 橋 = Vec::new();
    let mut 時刻 = 0isize;
    for s in 0..n {
        if 番号[s] != -1 {
            continue;
        }
        番号[s] = 時刻;
        low[s] = 時刻;
        時刻 += 1;
        // (頂点, 親辺番号, 隣接走査位置)
        let mut 探索: Vec<(usize, isize, usize)> = vec![(s, -1, 0)];
        while let Some(&(now, 親辺, 位置)) = 探索.last() {
            if 位置 < グラフ[now].len() {
                探索.last_mut().unwrap().2 += 1;
                let (next, eid) = グラフ[now][位置];
                if eid as isize == 親辺 {
                    continue; // 来た辺だけは 1 回スキップ（並行辺はスキップしない）
                }
                if 番号[next] == -1 {
                    番号[next] = 時刻;
                    low[next] = 時刻;
                    時刻 += 1;
                    探索.push((next, eid as isize, 0));
                } else {
                    low[now] = low[now].min(番号[next]);
                }
            } else {
                探索.pop();
                if let Some(&(prev, _, _)) = 探索.last() {
                    low[prev] = low[prev].min(low[now]);
                    if low[now] > 番号[prev] {
                        橋辺[親辺 as usize] = true;
                        橋.push((prev, now));
                    }
                }
            }
        }
    }
    // 橋でない辺だけで連結成分（＝二重辺連結成分）を割り当てる。
    let mut 成分 = vec![!0; n];
    let mut 成分数 = 0;
    for s in 0..n {
        if 成分[s] != !0 {
            continue;
        }
        成分[s] = 成分数;
        let mut 探索 = vec![s];
        while let Some(now) = 探索.pop() {
            for &(next, eid) in &グラフ[now] {
                if !橋辺[eid] && 成分[next] == !0 {
                    成分[next] = 成分数;
                    探索.push(next);
                }
            }
        }
        成分数 += 1;
    }
    // 橋森を構成する。
    let mut 木 = vec![Vec::new(); 成分数];
    for &(a, b) in &橋 {
        let (x, y) = (成分[a], 成分[b]);
        木[x].push(y);
        木[y].push(x);
    }
    二重辺連結成分 {
        成分,
        成分数,
        橋,
        木,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ユニオンファインドで連結成分を求める素朴版。
    fn 連結成分(n: usize, 辺: &[(usize, usize)]) -> Vec<usize> {
        let mut 親: Vec<usize> = (0..n).collect();
        fn 根(親: &mut [usize], x: usize) -> usize {
            if 親[x] == x {
                x
            } else {
                let r = 根(親, 親[x]);
                親[x] = r;
                r
            }
        }
        for &(u, v) in 辺 {
            let (ru, rv) = (根(&mut 親, u), 根(&mut 親, v));
            親[ru] = rv;
        }
        (0..n).map(|x| 根(&mut 親, x)).collect()
    }

    // 辺 i を取り除くと端点が非連結になるか（＝橋か）を素朴に判定。
    fn 橋集合(n: usize, 辺: &[(usize, usize)]) -> Vec<bool> {
        (0..辺.len())
            .map(|i| {
                let (u, v) = 辺[i];
                if u == v {
                    return false;
                }
                let 残り: Vec<(usize, usize)> = 辺
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, &e)| e)
                    .collect();
                let c = 連結成分(n, &残り);
                c[u] != c[v]
            })
            .collect()
    }

    #[test]
    fn 乱数で素朴と一致() {
        let mut seed = 0xb71d_9e3au64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..400 {
            let n = (乱数() as usize % 9) + 1;
            let 本数 = 乱数() as usize % (n * 2 + 1);
            let 辺: Vec<(usize, usize)> = (0..本数)
                .map(|_| (乱数() as usize % n, 乱数() as usize % n))
                .collect();
            let r = 二重辺連結成分分解(n, &辺);

            // 1) 橋本数が素朴と一致。
            let 橋判定 = 橋集合(n, &辺);
            let 素朴橋数 = 橋判定.iter().filter(|&&b| b).count();
            assert_eq!(r.橋.len(), 素朴橋数, "橋数 n={n} 辺={辺:?}");

            // 2) 二重辺連結成分の分割が、橋を全部取り除いた連結成分と一致。
            let 橋なし辺: Vec<(usize, usize)> = 辺
                .iter()
                .enumerate()
                .filter(|&(j, _)| !橋判定[j])
                .map(|(_, &e)| e)
                .collect();
            let オラクル = 連結成分(n, &橋なし辺);
            for i in 0..n {
                for j in 0..n {
                    assert_eq!(
                        r.成分[i] == r.成分[j],
                        オラクル[i] == オラクル[j],
                        "分割 n={n} 辺={辺:?} i={i} j={j}"
                    );
                }
            }

            // 3) 橋森は森（辺数 = 成分数 - 連結なグラフ全体の連結成分数）。
            let 木辺数: usize = r.木.iter().map(|a| a.len()).sum::<usize>() / 2;
            assert_eq!(木辺数, r.橋.len());
            let 全体連結 = 連結成分(n, &辺);
            let 連結数 = {
                let mut s: Vec<usize> = 全体連結.clone();
                s.sort_unstable();
                s.dedup();
                s.len()
            };
            assert_eq!(r.成分数, 木辺数 + 連結数, "森性 n={n} 辺={辺:?}");
        }
    }
}
