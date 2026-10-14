use crate::graph_build::無向グラフ;
use cargo_snippet::snippet;

#[snippet("木_LCA", include = "無向_グラフ")]
/// 木の LCA（ダブリング / 二分上昇）。前処理 `O(N log N)`、クエリ `O(log N)`。
///
/// 入力は AtCoder の辺入力をそのまま 0-indexed にした `辺: &[(usize, usize)]`。連結な木を前提とする。
///
/// - [`Self::lca`] : 最近共通祖先。
/// - [`Self::距離`] : 2 頂点間の距離（辺の本数）。
/// - [`Self::パス頂点列`] : `u` から `v` へ向かうパス上の頂点列。
/// - [`Self::祖先`] : `k` 個上の祖先（level ancestor）。
///
/// 構築は反復 DFS なので、`N = 2 * 10^5` の一直線の木でもスタックが溢れない。
struct 木Lca {
    log: usize,
    /// `上[k][v]`: `v` の `2^k` 個上の祖先。根より上は根自身（自己ループ）で埋める。
    上: Vec<Vec<usize>>,
    深さ: Vec<usize>,
    根: usize,
}

#[snippet("木_LCA")]
impl 木Lca {
    /// 頂点数 `n`・辺リスト `辺` の木を、頂点 0 を根として前処理する。
    fn new(n: usize, 辺: &[(usize, usize)]) -> Self {
        Self::根付き(n, 辺, 0)
    }

    /// 根を明示して前処理する。
    fn 根付き(n: usize, 辺: &[(usize, usize)], 根: usize) -> Self {
        let log = ((usize::BITS - n.leading_zeros()) as usize).max(1);
        let mut 上 = vec![vec![根; n]; log];
        let mut 深さ = vec![0usize; n];
        if n == 0 {
            return Self {
                log, 上, 深さ, 根
            };
        }
        // 反復 DFS で親と深さを確定（再帰だと深い木でスタックが溢れる）。
        // 根の親は根自身にしておくと、上への移動が範囲外にならない。
        let グラフ = 無向グラフ(n, 辺);
        let mut 探索 = vec![(根, 根)];
        while let Some((now, prev)) = 探索.pop() {
            上[0][now] = prev;
            for &next in &グラフ[now] {
                if next != prev {
                    深さ[next] = 深さ[now] + 1;
                    探索.push((next, now));
                }
            }
        }
        for k in 1..log {
            let (下位, 上位) = 上.split_at_mut(k);
            let 一段下 = &下位[k - 1];
            for now in 0..n {
                上位[0][now] = 一段下[一段下[now]];
            }
        }
        Self {
            log, 上, 深さ, 根
        }
    }

    /// 根。
    fn 根(&self) -> usize {
        self.根
    }

    /// 根から `v` までの辺数。
    fn 深さ(&self, v: usize) -> usize {
        self.深さ[v]
    }

    /// `v` の親。根の親は根自身。
    fn 親(&self, v: usize) -> usize {
        self.上[0][v]
    }

    /// `v` の `k` 個上の祖先。根を通り越す場合（`k > 深さ(v)`）は `None`。
    fn 祖先(&self, mut v: usize, k: usize) -> Option<usize> {
        if k > self.深さ[v] {
            return None;
        }
        for i in 0..self.log {
            if k >> i & 1 == 1 {
                v = self.上[i][v];
            }
        }
        Some(v)
    }

    /// 最近共通祖先。
    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.深さ[u] < self.深さ[v] {
            std::mem::swap(&mut u, &mut v);
        }
        // 深い方を同じ深さまで持ち上げる。
        let 差 = self.深さ[u] - self.深さ[v];
        for k in 0..self.log {
            if 差 >> k & 1 == 1 {
                u = self.上[k][u];
            }
        }
        if u == v {
            return u;
        }
        // 祖先が一致しない限り上がると、LCA の 1 つ下で止まる。
        for k in (0..self.log).rev() {
            if self.上[k][u] != self.上[k][v] {
                u = self.上[k][u];
                v = self.上[k][v];
            }
        }
        self.上[0][u]
    }

    /// `u`-`v` 間の距離（辺の本数）。
    fn 距離(&self, u: usize, v: usize) -> usize {
        self.深さ[u] + self.深さ[v] - 2 * self.深さ[self.lca(u, v)]
    }

    /// `u` から `v` へ向かうパス上の頂点列（`u` が先頭、`v` が末尾、両端を含む）。
    /// 長さは `距離(u, v) + 1`。パスの長さに比例した時間がかかる。
    fn パス頂点列(&self, u: usize, v: usize) -> Vec<usize> {
        let w = self.lca(u, v);
        let mut 列 = Vec::new();
        let mut x = u;
        while x != w {
            列.push(x);
            x = self.上[0][x];
        }
        列.push(w);
        // v 側は v から登るので、LCA 側が先に来るよう反転してから繋ぐ。
        let mut 後半 = Vec::new();
        let mut y = v;
        while y != w {
            後半.push(y);
            y = self.上[0][y];
        }
        後半.reverse();
        列.extend(後半);
        列
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 0-1-2-3-...-(n-1) の一直線。辺 i は (i, i+1)。
    fn 直線(n: usize) -> Vec<(usize, usize)> {
        (0..n - 1).map(|i| (i, i + 1)).collect()
    }

    // 親を辿るだけの素朴な LCA。
    fn 素朴lca(親: &[usize], 深さ: &[usize], mut u: usize, mut v: usize) -> usize {
        while u != v {
            if 深さ[u] < 深さ[v] {
                std::mem::swap(&mut u, &mut v);
            }
            u = 親[u];
        }
        u
    }

    #[test]
    fn 直線のlcaと距離() {
        let 辺 = 直線(5);
        let lca = 木Lca::new(5, &辺);
        assert_eq!(lca.lca(0, 4), 0);
        assert_eq!(lca.lca(2, 4), 2);
        assert_eq!(lca.距離(1, 3), 2);
        assert_eq!(lca.距離(0, 4), 4);
    }

    #[test]
    fn パス頂点列はuからvの順() {
        let 辺 = 直線(5);
        let lca = 木Lca::new(5, &辺);
        assert_eq!(lca.パス頂点列(1, 3), vec![1, 2, 3]);
        assert_eq!(lca.パス頂点列(3, 1), vec![3, 2, 1]);
        assert_eq!(lca.パス頂点列(0, 4), vec![0, 1, 2, 3, 4]);
        assert_eq!(lca.パス頂点列(2, 2), vec![2]);
    }

    #[test]
    fn パス頂点列はlcaで折り返す() {
        //        0
        //       / \
        //      1   2
        //     / \   \
        //    3   4   5
        let 辺 = [(0, 1), (0, 2), (1, 3), (1, 4), (2, 5)];
        let lca = 木Lca::new(6, &辺);
        assert_eq!(lca.パス頂点列(3, 5), vec![3, 1, 0, 2, 5]);
        assert_eq!(lca.パス頂点列(3, 4), vec![3, 1, 4]);
        assert_eq!(lca.パス頂点列(4, 0), vec![4, 1, 0]);
    }

    #[test]
    fn 祖先はk個上を返す() {
        let 辺 = 直線(5);
        let lca = 木Lca::new(5, &辺);
        assert_eq!(lca.祖先(4, 0), Some(4));
        assert_eq!(lca.祖先(4, 1), Some(3));
        assert_eq!(lca.祖先(4, 4), Some(0));
        assert_eq!(lca.祖先(4, 5), None);
        assert_eq!(lca.祖先(0, 1), None);
    }

    #[test]
    fn 根を指定できる() {
        let 辺 = 直線(5);
        let lca = 木Lca::根付き(5, &辺, 4);
        assert_eq!(lca.根(), 4);
        assert_eq!(lca.深さ(0), 4);
        assert_eq!(lca.親(0), 1);
        assert_eq!(lca.lca(0, 2), 2); // 根が 4 なので 2 が祖先
        assert_eq!(lca.距離(0, 2), 2);
    }

    #[test]
    fn 乱数木で素朴と一致() {
        let mut seed = 0x51a9_3c07u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..300 {
            let n = (乱数() as usize % 30) + 1;
            // 頂点 i (>= 1) を 0..i のランダムな親に繋ぐ。
            let mut 辺 = Vec::new();
            let mut 親 = vec![0usize; n];
            let mut 深さ = vec![0usize; n];
            for i in 1..n {
                let p = 乱数() as usize % i;
                親[i] = p;
                深さ[i] = 深さ[p] + 1;
                辺.push((p, i));
            }
            let lca = 木Lca::new(n, &辺);
            for u in 0..n {
                assert_eq!(lca.深さ(u), 深さ[u], "深さ n={n} u={u}");
                assert_eq!(lca.親(u), 親[u], "親 n={n} u={u}");
                for v in 0..n {
                    let 期待 = 素朴lca(&親, &深さ, u, v);
                    assert_eq!(lca.lca(u, v), 期待, "lca n={n} u={u} v={v}");
                    assert_eq!(
                        lca.距離(u, v),
                        深さ[u] + 深さ[v] - 2 * 深さ[期待],
                        "距離 n={n} u={u} v={v}"
                    );
                    // パス頂点列: 端点・長さ・隣接関係を検証。
                    let 列 = lca.パス頂点列(u, v);
                    assert_eq!(列[0], u);
                    assert_eq!(列[列.len() - 1], v);
                    assert_eq!(列.len(), lca.距離(u, v) + 1);
                    for w in 列.windows(2) {
                        assert!(親[w[0]] == w[1] || 親[w[1]] == w[0], "隣接していない {w:?}");
                    }
                }
            }
        }
    }
}
