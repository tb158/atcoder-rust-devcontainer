#[snippet("GomoryHu")]
#[snippet("Gusfield")]
#[snippet("ゴモリー_フー_木")]
#[snippet("全点対_最小_カット")]
#[snippet("最小_カット_木")]
use ac_library::MfGraph;
use cargo_snippet::snippet;

#[snippet("全点対_最小_カット")]
#[snippet("ゴモリー_フー_木")]
#[snippet("GomoryHu")]
#[snippet("Gusfield")]
#[snippet("最小_カット_木")]
/// 全点対最小カット木（Gomory–Hu 木）。無向重み付きグラフの「任意の 2 頂点間の最小カット」を、
/// `n-1` 本の辺からなる 1 本の木で表す。木上の `u`–`v` パスの最小辺重みが `u`–`v` の最小カットに等しい。
///
/// `prev[i]`（`i >= 1`）はその親、`重み[i]` は辺 `(i, prev[i])` の容量（＝そのカット値）。
struct 全点対最小カット木 {
    /// `prev[i]` は木における頂点 `i` の親（`prev[0] = 0`）。
    prev: Vec<usize>,
    /// `重み[i]` は辺 `(i, prev[i])` の最小カット値（`重み[0] = 0`）。
    重み: Vec<isize>,
    グラフ: Vec<Vec<(usize, isize)>>,
}

#[snippet("全点対_最小_カット")]
#[snippet("ゴモリー_フー_木")]
#[snippet("GomoryHu")]
#[snippet("Gusfield")]
#[snippet("最小_カット_木")]
impl 全点対最小カット木 {
    /// 木上のパスの最小辺重み = 頂点 `u`, `v` 間の最小カット値を `O(n)` で返す。
    fn 最小カット(&self, u: usize, v: usize) -> isize {
        if u == v {
            return 0;
        }
        let n = self.グラフ.len();
        let mut 最小 = vec![isize::MAX; n];
        let mut 到達済 = vec![false; n];
        到達済[u] = true;
        let mut 探索 = vec![u];
        while let Some(now) = 探索.pop() {
            for &(next, w) in &self.グラフ[now] {
                if !到達済[next] {
                    到達済[next] = true;
                    最小[next] = 最小[now].min(w);
                    探索.push(next);
                }
            }
        }
        最小[v]
    }
}

#[snippet("全点対_最小_カット")]
#[snippet("ゴモリー_フー_木")]
#[snippet("GomoryHu")]
#[snippet("Gusfield")]
#[snippet("最小_カット_木")]
/// Gusfield の簡易アルゴリズムで Gomory–Hu 木（等価フロー木）を構築する。`ac_library::MfGraph` を
/// `n-1` 回呼ぶ。`辺` は `(u, v, 容量)` の無向辺（容量非負、多重辺は別々に追加、自己ループは無視）。
///
/// 計算量は最大流 `n-1` 回ぶん。非連結なグラフでは成分間のカットは `0` になる。
fn 全点対最小カット木構築(
    n: usize,
    辺: &[(usize, usize, isize)],
) -> 全点対最小カット木 {
    let mut prev = vec![0usize; n];
    let mut 重み = vec![0isize; n];
    for i in 1..n {
        let t = prev[i];
        let mut g = MfGraph::<isize>::new(n);
        for &(u, v, w) in 辺 {
            if u != v {
                // 無向辺は両方向に容量 w を張る。
                g.add_edge(u, v, w);
                g.add_edge(v, u, w);
            }
        }
        let f = g.flow(i, t);
        重み[i] = f;
        let 側 = g.min_cut(i); // 側[v] = true なら v は i 側
        for j in (i + 1)..n {
            if 側[j] && prev[j] == t {
                prev[j] = i;
            }
        }
    }
    let mut グラフ = vec![Vec::new(); n];
    for i in 1..n {
        グラフ[i].push((prev[i], 重み[i]));
        グラフ[prev[i]].push((i, 重み[i]));
    }
    全点対最小カット木 {
        prev, 重み, グラフ
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ac_library の最大流で直接 s–t 最小カットを求める（独立オラクル）。
    fn 直接最小カット(n: usize, 辺: &[(usize, usize, isize)], s: usize, t: usize) -> isize {
        let mut g = MfGraph::<isize>::new(n);
        for &(u, v, w) in 辺 {
            if u != v {
                g.add_edge(u, v, w);
                g.add_edge(v, u, w);
            }
        }
        g.flow(s, t)
    }

    #[test]
    fn 乱数で全点対が一致() {
        let mut seed = 0xc0ff_ee11u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..150 {
            let n = (乱数() as usize % 6) + 2; // 2..=7
            let 本数 = 乱数() as usize % (n * 2 + 1);
            let 辺: Vec<(usize, usize, isize)> = (0..本数)
                .filter_map(|_| {
                    let u = 乱数() as usize % n;
                    let v = 乱数() as usize % n;
                    if u == v {
                        None
                    } else {
                        Some((u, v, (乱数() % 9) as isize + 1))
                    }
                })
                .collect();
            let 木 = 全点対最小カット木構築(n, &辺);
            for u in 0..n {
                for v in (u + 1)..n {
                    assert_eq!(
                        木.最小カット(u, v),
                        直接最小カット(n, &辺, u, v),
                        "u={u} v={v} n={n} 辺={辺:?}"
                    );
                }
            }
        }
    }
}
