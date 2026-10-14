use crate::graph_build::無向グラフ;
use cargo_snippet::snippet;

#[snippet("HL_分解", include = "無向_グラフ")]
#[snippet("重軽_分解")]
#[snippet("HeavyLightDecomposition")]
#[snippet("HLD")]
#[snippet("木_パス_クエリ")]
/// HL 分解（Heavy-Light Decomposition）。根付き木のパス/部分木を `pos` 上の連続区間に
/// 落とし込み、`ac_library::Segtree` 等の区間データ構造と組み合わせて使う。構築 O(N)。
///
/// - [`Self::パス区間`] : `u`-`v` パス（頂点モデル、LCA を含む）を半開区間 `[l, r)` の列に分解。
/// - [`Self::部分木区間`] : `v` の部分木を 1 つの半開区間に。
/// - [`Self::lca`] : 最近共通祖先 O(log N)。
///
/// `pos[v]` は heavy path が連続するよう振った Euler 順位置。区間データ構造には
/// `pos` 順で値を載せる。辺に値を持たせたい場合は「子頂点に辺の値を載せ、パス分解時に
/// LCA を 1 つ除く」運用にする。
struct 重軽分解 {
    prev: Vec<usize>,
    深さ: Vec<usize>,
    sz: Vec<usize>,
    /// `head[v]`: `v` を含む heavy path の先頭（最も浅い）頂点
    head: Vec<usize>,
    /// `pos[v]`: 区間データ構造上での `v` の位置
    pos: Vec<usize>,
}

#[snippet("HL_分解")]
#[snippet("重軽_分解")]
#[snippet("HeavyLightDecomposition")]
#[snippet("HLD")]
#[snippet("木_パス_クエリ")]
impl 重軽分解 {
    /// 頂点数 `n`・辺リスト `辺`（AtCoder の入力そのまま）と根から構築する。O(N)。
    fn new(n: usize, 辺: &[(usize, usize)], 根: usize) -> Self {
        let グラフ = 無向グラフ(n, 辺);
        let mut prev = vec![!0; n];
        let mut 深さ = vec![0usize; n];
        let mut sz = vec![1usize; n];
        let mut heavy = vec![!0; n];
        let mut 順 = Vec::with_capacity(n);
        // 反復 DFS で prev・深さ・行きがけ順を確定
        let mut 探索 = vec![根];
        while let Some(now) = 探索.pop() {
            順.push(now);
            for &next in &グラフ[now] {
                if next != prev[now] {
                    prev[next] = now;
                    深さ[next] = 深さ[now] + 1;
                    探索.push(next);
                }
            }
        }
        // 帰りがけ順（行きがけの逆）で部分木サイズと heavy child を求める
        for &now in 順.iter().rev() {
            let mut 最大 = 0;
            for &next in &グラフ[now] {
                if next != prev[now] {
                    sz[now] += sz[next];
                    if sz[next] > 最大 {
                        最大 = sz[next];
                        heavy[now] = next;
                    }
                }
            }
        }
        // head と pos を割り当てる。light child を先に積み heavy child を最後に積むことで、
        // heavy child が先に pop され heavy path が連続した pos を持つ。
        let mut head = vec![0usize; n];
        let mut pos = vec![0usize; n];
        let mut 次位置 = 0usize;
        let mut 探索 = vec![(根, 根)];
        while let Some((now, h)) = 探索.pop() {
            head[now] = h;
            pos[now] = 次位置;
            次位置 += 1;
            for &next in &グラフ[now] {
                if next != prev[now] && next != heavy[now] {
                    探索.push((next, next));
                }
            }
            if heavy[now] != !0 {
                探索.push((heavy[now], h));
            }
        }
        Self {
            prev,
            深さ,
            sz,
            head,
            pos,
        }
    }

    /// 区間データ構造上の位置 `pos[v]`。
    fn 位置(&self, v: usize) -> usize {
        self.pos[v]
    }

    /// 最近共通祖先。O(log N)。
    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        while self.head[u] != self.head[v] {
            if self.深さ[self.head[u]] < self.深さ[self.head[v]] {
                std::mem::swap(&mut u, &mut v);
            }
            u = self.prev[self.head[u]];
        }
        if self.深さ[u] < self.深さ[v] {
            u
        } else {
            v
        }
    }

    /// `u`-`v` パス上の頂点集合を、区間データ構造上の半開区間 `[l, r)` の列で返す（順不同）。
    /// LCA を含む頂点モデル。区間は O(log N) 本。
    fn パス区間(&self, mut u: usize, mut v: usize) -> Vec<(usize, usize)> {
        let mut 区間 = Vec::new();
        while self.head[u] != self.head[v] {
            if self.深さ[self.head[u]] < self.深さ[self.head[v]] {
                std::mem::swap(&mut u, &mut v);
            }
            let h = self.head[u];
            区間.push((self.pos[h], self.pos[u] + 1));
            u = self.prev[h];
        }
        let (lo, hi) = if self.pos[u] <= self.pos[v] {
            (self.pos[u], self.pos[v])
        } else {
            (self.pos[v], self.pos[u])
        };
        区間.push((lo, hi + 1));
        区間
    }

    /// `v` を根とする部分木に対応する半開区間 `[l, r)`。O(1)。
    fn 部分木区間(&self, v: usize) -> (usize, usize) {
        (self.pos[v], self.pos[v] + self.sz[v])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // pos -> 頂点 の逆引き
    fn 逆引き(h: &重軽分解, n: usize) -> Vec<usize> {
        let mut inv = vec![0usize; n];
        for v in 0..n {
            inv[h.位置(v)] = v;
        }
        inv
    }

    // 素朴な親・深さを行きがけ DFS で構築
    fn 素朴親深さ(g: &[Vec<usize>], 根: usize) -> (Vec<usize>, Vec<usize>) {
        let n = g.len();
        let mut 親 = vec![usize::MAX; n];
        let mut 深さ = vec![0usize; n];
        let mut stk = vec![(根, usize::MAX)];
        while let Some((u, p)) = stk.pop() {
            親[u] = p;
            for &x in &g[u] {
                if x != p {
                    深さ[x] = 深さ[u] + 1;
                    stk.push((x, u));
                }
            }
        }
        (親, 深さ)
    }

    fn 素朴パス頂点(
        親: &[usize], 深さ: &[usize], mut u: usize, mut v: usize
    ) -> Vec<usize> {
        let mut 集合 = Vec::new();
        while u != v {
            if 深さ[u] < 深さ[v] {
                std::mem::swap(&mut u, &mut v);
            }
            集合.push(u);
            u = 親[u];
        }
        集合.push(u);
        集合.sort_unstable();
        集合
    }

    fn 素朴部分木(親: &[usize], 根: usize, v: usize) -> Vec<usize> {
        let n = 親.len();
        // v の子孫集合（v 自身を含む）
        let mut 子孫 = vec![false; n];
        子孫[v] = true;
        // 親方向に v へ到達できるかで判定
        #[allow(clippy::needless_range_loop)]
        for x in 0..n {
            let mut cur = x;
            loop {
                if cur == v {
                    子孫[x] = true;
                    break;
                }
                if cur == 根 || 親[cur] == usize::MAX {
                    break;
                }
                cur = 親[cur];
            }
        }
        (0..n).filter(|&x| 子孫[x]).collect()
    }

    #[test]
    fn パス分解が素朴解と一致() {
        let n = 9;
        let 辺 = [
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (3, 5),
            (2, 6),
            (6, 7),
            (6, 8),
        ];
        let g = 無向グラフ(n, &辺);
        let h = 重軽分解::new(n, &辺, 0);
        let inv = 逆引き(&h, n);
        let (親, 深さ) = 素朴親深さ(&g, 0);

        for u in 0..n {
            for v in 0..n {
                let mut 集合: Vec<usize> = h
                    .パス区間(u, v)
                    .into_iter()
                    .flat_map(|(l, r)| (l..r).map(|p| inv[p]))
                    .collect();
                集合.sort_unstable();
                let 期待 = 素朴パス頂点(&親, &深さ, u, v);
                assert_eq!(集合, 期待, "u={u}, v={v}");
            }
        }
    }

    #[test]
    fn 部分木区間が素朴解と一致() {
        let n = 9;
        let 辺 = [
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (3, 5),
            (2, 6),
            (6, 7),
            (6, 8),
        ];
        let g = 無向グラフ(n, &辺);
        let h = 重軽分解::new(n, &辺, 0);
        let inv = 逆引き(&h, n);
        let (親, _) = 素朴親深さ(&g, 0);

        for v in 0..n {
            let (l, r) = h.部分木区間(v);
            let mut 集合: Vec<usize> = (l..r).map(|p| inv[p]).collect();
            集合.sort_unstable();
            assert_eq!(集合, 素朴部分木(&親, 0, v), "v={v}");
        }
    }

    #[test]
    fn lcaが素朴解と一致() {
        let n = 7;
        let 辺 = [(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)];
        let g = 無向グラフ(n, &辺);
        let h = 重軽分解::new(n, &辺, 0);
        let (親, 深さ) = 素朴親深さ(&g, 0);
        let 素朴lca = |mut u: usize, mut v: usize| {
            while u != v {
                if 深さ[u] < 深さ[v] {
                    std::mem::swap(&mut u, &mut v);
                }
                u = 親[u];
            }
            u
        };
        for u in 0..n {
            for v in 0..n {
                assert_eq!(h.lca(u, v), 素朴lca(u, v), "u={u}, v={v}");
            }
        }
    }
}
