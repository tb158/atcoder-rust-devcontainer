use cargo_snippet::snippet;

#[snippet("最大_クリーク")]
#[snippet("MaxClique")]
#[snippet("最大_独立_集合")]
#[snippet("MaximumIndependentSet")]
#[snippet("BronKerbosch")]
/// 最大クリーク（完全部分グラフ）を Bron–Kerbosch 法（ピボット付き）で求める。頂点数 `n <= 64`。
/// ビットマスクで近傍を持ち、実用的には数十頂点まで高速（最悪 O(3^{n/3})）。
///
/// [`Self::最大独立集合`] は補グラフの最大クリークとして同じ実装で求まる。
struct 最大クリーク探索 {
    n: usize,
    グラフ: Vec<u64>,
}

#[snippet("最大_クリーク")]
#[snippet("MaxClique")]
#[snippet("最大_独立_集合")]
#[snippet("MaximumIndependentSet")]
#[snippet("BronKerbosch")]
impl 最大クリーク探索 {
    /// 頂点数 `n`（`<= 64`）の空グラフを作る。
    fn new(n: usize) -> Self {
        assert!(n <= 64, "n must be <= 64");
        Self {
            n,
            グラフ: vec![0u64; n],
        }
    }

    /// 無向辺 `u - v` を追加する。
    fn 辺追加(&mut self, u: usize, v: usize) {
        if u != v {
            self.グラフ[u] |= 1 << v;
            self.グラフ[v] |= 1 << u;
        }
    }

    /// 最大クリークの頂点集合を返す。
    fn 最大クリーク(&self) -> Vec<usize> {
        let 全 = if self.n == 64 {
            u64::MAX
        } else {
            (1u64 << self.n) - 1
        };
        let mut 最良 = 0u64;
        self.探索(0, 全, 0, &mut 最良);
        ビット列(最良)
    }

    /// 最大独立集合の頂点集合を返す（補グラフの最大クリーク）。
    fn 最大独立集合(&self) -> Vec<usize> {
        let 全 = if self.n == 64 {
            u64::MAX
        } else {
            (1u64 << self.n) - 1
        };
        let mut 補 = 最大クリーク探索::new(self.n);
        for v in 0..self.n {
            // 補グラフ: 自分以外で隣接していない頂点と結ぶ
            補.グラフ[v] = !self.グラフ[v] & 全 & !(1u64 << v);
        }
        let mut 最良 = 0u64;
        補.探索(0, 全, 0, &mut 最良);
        ビット列(最良)
    }

    /// Bron–Kerbosch（ピボット付き）。`r` 現在のクリーク、`p` 候補、`x` 除外済み。
    fn 探索(&self, r: u64, p: u64, x: u64, 最良: &mut u64) {
        if p == 0 && x == 0 {
            if r.count_ones() > 最良.count_ones() {
                *最良 = r;
            }
            return;
        }
        // 枝刈り: 今のクリーク + 残り候補で最良を超えられないなら打ち切る
        if r.count_ones() + p.count_ones() <= 最良.count_ones() {
            return;
        }
        // ピボット: P|X の中で「P と最も多く隣接する」頂点を選ぶ
        let mut ピボット候補 = p | x;
        let mut ピボット = 0usize;
        let mut 最多 = -1isize;
        while ピボット候補 != 0 {
            let v = ピボット候補.trailing_zeros() as usize;
            ピボット候補 &= ピボット候補 - 1;
            let c = (p & self.グラフ[v]).count_ones() as isize;
            if c > 最多 {
                最多 = c;
                ピボット = v;
            }
        }
        let mut 候補 = p & !self.グラフ[ピボット];
        let mut p = p;
        let mut x = x;
        while 候補 != 0 {
            let v = 候補.trailing_zeros() as usize;
            let ビット = 1u64 << v;
            候補 &= 候補 - 1;
            self.探索(r | ビット, p & self.グラフ[v], x & self.グラフ[v], 最良);
            p &= !ビット;
            x |= ビット;
        }
    }
}

#[snippet("最大_クリーク")]
#[snippet("MaxClique")]
#[snippet("最大_独立_集合")]
#[snippet("MaximumIndependentSet")]
#[snippet("BronKerbosch")]
/// ビットマスクを立っている添字の昇順 `Vec` に変換する。
fn ビット列(mut m: u64) -> Vec<usize> {
    let mut v = Vec::new();
    while m != 0 {
        v.push(m.trailing_zeros() as usize);
        m &= m - 1;
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    // 総当たり: 全頂点部分集合からクリークの最大サイズを求める。
    fn 総当たりクリーク(n: usize, 隣接行列: &[Vec<bool>]) -> usize {
        let mut 最良 = 0;
        for s in 0u32..(1 << n) {
            let 集合: Vec<usize> = (0..n).filter(|&i| (s >> i) & 1 == 1).collect();
            let mut クリーク = true;
            for &a in &集合 {
                for &b in &集合 {
                    if a != b && !隣接行列[a][b] {
                        クリーク = false;
                    }
                }
            }
            if クリーク {
                最良 = 最良.max(集合.len());
            }
        }
        最良
    }

    fn 総当たり独立集合(n: usize, 隣接行列: &[Vec<bool>]) -> usize {
        let mut 最良 = 0;
        for s in 0u32..(1 << n) {
            let 集合: Vec<usize> = (0..n).filter(|&i| (s >> i) & 1 == 1).collect();
            let mut 独立 = true;
            for &a in &集合 {
                for &b in &集合 {
                    if a != b && 隣接行列[a][b] {
                        独立 = false;
                    }
                }
            }
            if 独立 {
                最良 = 最良.max(集合.len());
            }
        }
        最良
    }

    #[test]
    fn 完全グラフ() {
        let mut g = 最大クリーク探索::new(5);
        for u in 0..5 {
            for v in u + 1..5 {
                g.辺追加(u, v);
            }
        }
        assert_eq!(g.最大クリーク().len(), 5);
        assert_eq!(g.最大独立集合().len(), 1);
    }

    #[test]
    fn 辺なしグラフ() {
        let g = 最大クリーク探索::new(6);
        assert_eq!(g.最大クリーク().len(), 1);
        assert_eq!(g.最大独立集合().len(), 6);
    }

    #[test]
    fn クリークが正当() {
        let mut g = 最大クリーク探索::new(6);
        for (u, v) in [(0, 1), (1, 2), (0, 2), (2, 3), (3, 4), (4, 5)] {
            g.辺追加(u, v);
        }
        let c = g.最大クリーク();
        // クリークであることを確認
        for i in 0..c.len() {
            for j in i + 1..c.len() {
                assert!((g.グラフ[c[i]] >> c[j]) & 1 == 1);
            }
        }
        assert_eq!(c.len(), 3); // 三角形 {0,1,2}
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0xabcd_1234u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..120 {
            let n = (乱数() as usize % 9) + 1; // 1..=9
            let mut 行列 = vec![vec![false; n]; n];
            let mut g = 最大クリーク探索::new(n);
            for u in 0..n {
                for v in u + 1..n {
                    if 乱数() % 2 == 0 {
                        行列[u][v] = true;
                        行列[v][u] = true;
                        g.辺追加(u, v);
                    }
                }
            }
            assert_eq!(g.最大クリーク().len(), 総当たりクリーク(n, &行列), "n={n}");
            assert_eq!(g.最大独立集合().len(), 総当たり独立集合(n, &行列), "n={n}");
        }
    }
}
