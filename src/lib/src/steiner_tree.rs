use crate::to_int::to_i64;
use cargo_snippet::snippet;
#[snippet("SteinerTree")]
#[snippet("シュタイナー_木")]
#[snippet("最小_シュタイナー_木")]
#[snippet("端子を_結ぶ_最小木")]
use std::cmp::Reverse;
#[snippet("SteinerTree")]
#[snippet("シュタイナー_木")]
#[snippet("最小_シュタイナー_木")]
#[snippet("端子を_結ぶ_最小木")]
use std::collections::BinaryHeap;

#[snippet("シュタイナー_木")]
#[snippet("SteinerTree")]
#[snippet("最小_シュタイナー_木")]
#[snippet("端子を_結ぶ_最小木")]
const 無限大シュタイナー: i64 = 1 << 60;

#[snippet("シュタイナー_木")]
#[snippet("SteinerTree")]
#[snippet("最小_シュタイナー_木")]
#[snippet("端子を_結ぶ_最小木")]
/// 最小シュタイナー木。重み付き無向グラフで、指定した「端子（terminal）」をすべて連結する最小重み部分木の
/// 重みを求める。端子数 `k` に対し O(3^k V + 2^k (V + E) log V) の集合 DP。辺重みは非負（内部で Dijkstra を使う）。
///
/// 全域木（最小全域木）とは異なり、端子以外の頂点（Steiner 点）を経由してよい点が特徴。
/// 端子が 0/1 個なら 0、連結できないときは [`Self::到達不可`] を返す。
///
/// - [`Self::解く`] : 端子集合を連結する最小重み。
struct 最小シュタイナー木 {
    n: usize,
    グラフ: Vec<Vec<(usize, i64)>>,
}

#[snippet("シュタイナー_木", include = "to_i64")]
#[snippet("SteinerTree")]
#[snippet("最小_シュタイナー_木")]
#[snippet("端子を_結ぶ_最小木")]
impl 最小シュタイナー木 {
    /// 頂点数 `n` の空グラフを作る。
    fn new(n: usize) -> Self {
        Self {
            n,
            グラフ: vec![Vec::new(); n],
        }
    }

    /// 重み `w`（非負）の無向辺 `u - v` を追加する。
    fn 辺追加(&mut self, u: usize, v: usize, w: impl TryInto<i64>) {
        let w = to_i64(w);
        self.グラフ[u].push((v, w));
        self.グラフ[v].push((u, w));
    }

    /// 連結できないことを表す番兵値。
    fn 到達不可() -> i64 {
        無限大シュタイナー
    }

    /// 端子集合をすべて連結する最小重み部分木の総重みを返す。
    fn 解く(&self, 端子: &[usize]) -> i64 {
        let k = 端子.len();
        if k <= 1 {
            return 0;
        }
        let 全 = 1usize << k;
        let mut dp = vec![vec![無限大シュタイナー; self.n]; 全];
        for (i, &t) in 端子.iter().enumerate() {
            dp[1 << i][t] = 0;
        }
        for mask in 1..全 {
            // 部分集合のマージ: dp[mask][v] = min(dp[sub][v] + dp[mask^sub][v])
            let mut sub = (mask - 1) & mask;
            while sub > 0 {
                let 他 = mask ^ sub;
                for v in 0..self.n {
                    let 候補 = dp[sub][v].saturating_add(dp[他][v]);
                    if 候補 < dp[mask][v] {
                        dp[mask][v] = 候補;
                    }
                }
                sub = (sub - 1) & mask;
            }
            // 同じ S を保ったまま辺で緩和する Dijkstra
            let mut ヒープ = BinaryHeap::new();
            for (v, &d) in dp[mask].iter().enumerate() {
                if d < 無限大シュタイナー {
                    ヒープ.push((Reverse(d), v));
                }
            }
            while let Some((Reverse(d), now)) = ヒープ.pop() {
                if d > dp[mask][now] {
                    continue;
                }
                for &(next, w) in &self.グラフ[now] {
                    if dp[mask][now] + w < dp[mask][next] {
                        dp[mask][next] = dp[mask][now] + w;
                        ヒープ.push((Reverse(dp[mask][next]), next));
                    }
                }
            }
        }
        dp[全 - 1].iter().copied().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 総当たり: Steiner 点の部分集合を全列挙し、(端子 ∪ Steiner 点) の距離閉包上の MST を取る。
    // 最小シュタイナー木 = min_S MST(端子 ∪ S, 全点対最短距離) が成り立つ。
    fn 総当たり(n: usize, 辺: &[(usize, usize, i64)], 端子: &[usize]) -> i64 {
        if 端子.len() <= 1 {
            return 0;
        }
        // Floyd-Warshall
        let mut d = vec![vec![無限大シュタイナー; n]; n];
        for (i, row) in d.iter_mut().enumerate() {
            row[i] = 0;
        }
        for &(u, v, w) in 辺 {
            d[u][v] = d[u][v].min(w);
            d[v][u] = d[v][u].min(w);
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if d[i][k] < 無限大シュタイナー && d[k][j] < 無限大シュタイナー
                    {
                        let 経由 = d[i][k] + d[k][j];
                        if 経由 < d[i][j] {
                            d[i][j] = 経由;
                        }
                    }
                }
            }
        }
        let 端子集合: std::collections::HashSet<usize> = 端子.iter().copied().collect();
        let 余り: Vec<usize> = (0..n).filter(|v| !端子集合.contains(v)).collect();
        let mut 最良 = 無限大シュタイナー;
        for ビット in 0..(1usize << 余り.len()) {
            let mut 頂点 = 端子.to_vec();
            for (i, &v) in 余り.iter().enumerate() {
                if (ビット >> i) & 1 == 1 {
                    頂点.push(v);
                }
            }
            最良 = 最良.min(mst距離閉包(&頂点, &d));
        }
        最良
    }

    // 与えた頂点集合について距離行列上の完全グラフの MST（Prim）。
    fn mst距離閉包(頂点: &[usize], d: &[Vec<i64>]) -> i64 {
        let m = 頂点.len();
        if m <= 1 {
            return 0;
        }
        let mut 連結 = vec![false; m];
        let mut 距離 = vec![無限大シュタイナー; m];
        距離[0] = 0;
        let mut 合計 = 0;
        for _ in 0..m {
            let mut 選 = None;
            for i in 0..m {
                if !連結[i] && (選.is_none() || 距離[i] < 距離[選.unwrap()]) {
                    選 = Some(i);
                }
            }
            let 選 = 選.unwrap();
            if 距離[選] >= 無限大シュタイナー {
                return 無限大シュタイナー; // 非連結
            }
            連結[選] = true;
            合計 += 距離[選];
            for i in 0..m {
                if !連結[i] {
                    距離[i] = 距離[i].min(d[頂点[選]][頂点[i]]);
                }
            }
        }
        合計
    }

    #[test]
    fn 三角形で経由点を使う() {
        // 0,1,2 が端子。直接辺は重いが中心 3 経由が安い。
        let mut g = 最小シュタイナー木::new(4);
        g.辺追加(0, 1, 10);
        g.辺追加(1, 2, 10);
        g.辺追加(2, 0, 10);
        g.辺追加(0, 3, 1);
        g.辺追加(1, 3, 1);
        g.辺追加(2, 3, 1);
        assert_eq!(g.解く(&[0, 1, 2]), 3);
    }

    #[test]
    fn 端子が0または1個は0() {
        let mut g = 最小シュタイナー木::new(3);
        g.辺追加(0, 1, 5);
        assert_eq!(g.解く(&[]), 0);
        assert_eq!(g.解く(&[2]), 0);
    }

    #[test]
    fn 一本道() {
        // 0-1-2-3-4 を結ぶ最小木は両端端子なら全部
        let mut g = 最小シュタイナー木::new(5);
        for i in 0..4 {
            g.辺追加(i, i + 1, (i as i64) + 1);
        }
        assert_eq!(g.解く(&[0, 4]), 1 + 2 + 3 + 4);
        assert_eq!(g.解く(&[1, 3]), 2 + 3);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x9e37_79b9u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..120 {
            let n = (乱数() as usize % 6) + 2; // 2..=7
            let mut 辺 = Vec::new();
            let mut g = 最小シュタイナー木::new(n);
            // 連結性確保のため木を 1 本張る
            for v in 1..n {
                let p = (乱数() as usize) % v;
                let w = (乱数() as i64 % 9) + 1;
                辺.push((p, v, w));
                g.辺追加(p, v, w);
            }
            // 追加辺
            let 追加数 = 乱数() as usize % (n + 1);
            for _ in 0..追加数 {
                let u = 乱数() as usize % n;
                let v = 乱数() as usize % n;
                if u != v {
                    let w = (乱数() as i64 % 9) + 1;
                    辺.push((u, v, w));
                    g.辺追加(u, v, w);
                }
            }
            // 端子をランダムに選ぶ（1..=min(4,n)）
            let k = (乱数() as usize % n.min(4)) + 1;
            let mut 候補: Vec<usize> = (0..n).collect();
            let mut 端子 = Vec::new();
            for _ in 0..k {
                let idx = 乱数() as usize % 候補.len();
                端子.push(候補.swap_remove(idx));
            }
            assert_eq!(
                g.解く(&端子),
                総当たり(n, &辺, &端子),
                "n={n} 端子={端子:?} 辺={辺:?}"
            );
        }
    }
}
