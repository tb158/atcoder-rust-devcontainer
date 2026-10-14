use crate::graph_build::無向グラフ;
use cargo_snippet::snippet;

#[snippet("仮想木", include = "無向_グラフ")]
#[snippet("AuxiliaryTree")]
#[snippet("VirtualTree")]
#[snippet("縮約木")]
#[snippet("補助木")]
/// 仮想木（Auxiliary Tree）。根付き木と頂点集合 `S` が与えられたとき、`S` と「`S` 内 2 頂点の LCA」だけを
/// 残して縮約した木を作る。頂点数 `|S|` に対し O(|S| log |S|) で構築でき、クエリごとの木 DP を高速化する。
///
/// 二分上昇 LCA を内包する自己完結版。
///
/// - [`Self::lca`] : 最近共通祖先。
/// - [`Self::構築`] : 頂点集合から仮想木の `(頂点リスト, 辺(親, 子))` を返す。
struct 仮想木 {
    上: Vec<Vec<i64>>, // 上[k][v] = v の 2^k 個上の祖先（無ければ -1）
    深さ: Vec<usize>,
    入時刻: Vec<usize>,
    出時刻: Vec<usize>,
    log: usize,
}

#[snippet("仮想木")]
#[snippet("AuxiliaryTree")]
#[snippet("VirtualTree")]
#[snippet("縮約木")]
#[snippet("補助木")]
impl 仮想木 {
    /// 頂点数 `n`・辺リスト `辺`（AtCoder の入力そのまま）と根から前計算する。
    fn new(n: usize, 辺: &[(usize, usize)], 根: usize) -> Self {
        let グラフ = 無向グラフ(n, 辺);
        let mut log = 1;
        while (1usize << log) < n.max(2) {
            log += 1;
        }
        let mut 上 = vec![vec![-1i64; n]; log];
        let 深さ = vec![0usize; n];
        let 入時刻 = vec![0usize; n];
        let 出時刻 = vec![0usize; n];
        let mut d = 構築データ {
            prev: std::mem::take(&mut 上[0]),
            深さ,
            入時刻,
            出時刻,
            時刻: 0,
        };
        探索(根, -1, &グラフ, &mut d);
        上[0] = d.prev;
        let 深さ = d.深さ;
        let 入時刻 = d.入時刻;
        let 出時刻 = d.出時刻;
        for k in 1..log {
            for v in 0..n {
                let 中 = 上[k - 1][v];
                上[k][v] = if 中 < 0 {
                    -1
                } else {
                    上[k - 1][中 as usize]
                };
            }
        }
        Self {
            上,
            深さ,
            入時刻,
            出時刻,
            log,
        }
    }

    /// `a` が `b` の祖先か（自身も祖先とみなす）。
    fn 祖先か(&self, a: usize, b: usize) -> bool {
        self.入時刻[a] <= self.入時刻[b] && self.出時刻[b] <= self.出時刻[a]
    }

    /// 最近共通祖先。
    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.深さ[u] < self.深さ[v] {
            std::mem::swap(&mut u, &mut v);
        }
        // u を v と同じ深さまで持ち上げる
        let 差 = self.深さ[u] - self.深さ[v];
        for k in 0..self.log {
            if (差 >> k) & 1 == 1 {
                u = self.上[k][u] as usize;
            }
        }
        if u == v {
            return u;
        }
        for k in (0..self.log).rev() {
            if self.上[k][u] != self.上[k][v] {
                u = self.上[k][u] as usize;
                v = self.上[k][v] as usize;
            }
        }
        self.上[0][u] as usize
    }

    /// 頂点集合から仮想木を構築し、`(頂点リスト, 辺(親, 子))` を返す。
    /// 頂点リストは入時刻順。辺は親が子の祖先になる向き。
    fn 構築(&self, 頂点集合: &[usize]) -> (Vec<usize>, Vec<(usize, usize)>) {
        if 頂点集合.is_empty() {
            return (Vec::new(), Vec::new());
        }
        let mut s = 頂点集合.to_vec();
        s.sort_by_key(|&v| self.入時刻[v]);
        s.dedup();
        // 連続する 2 頂点の LCA を追加
        let mut 追加 = Vec::new();
        for w in s.windows(2) {
            追加.push(self.lca(w[0], w[1]));
        }
        s.extend(追加);
        s.sort_by_key(|&v| self.入時刻[v]);
        s.dedup();
        // スタックで辺を構築
        let mut 辺 = Vec::new();
        let mut スタック: Vec<usize> = Vec::new();
        for &v in &s {
            while let Some(&天辺) = スタック.last() {
                if self.祖先か(天辺, v) {
                    break;
                }
                スタック.pop();
            }
            if let Some(&天辺) = スタック.last() {
                辺.push((天辺, v));
            }
            スタック.push(v);
        }
        (s, 辺)
    }
}

#[snippet("仮想木")]
#[snippet("AuxiliaryTree")]
#[snippet("VirtualTree")]
#[snippet("縮約木")]
#[snippet("補助木")]
/// DFS 中に埋める配列をまとめた作業領域（引数を 1 つに減らすため）。
struct 構築データ {
    prev: Vec<i64>,
    深さ: Vec<usize>,
    入時刻: Vec<usize>,
    出時刻: Vec<usize>,
    時刻: usize,
}

#[snippet("仮想木")]
#[snippet("AuxiliaryTree")]
#[snippet("VirtualTree")]
#[snippet("縮約木")]
#[snippet("補助木")]
/// 入時刻・出時刻・深さ・直接の親を埋める DFS。
fn 探索(now: usize, prev: i64, グラフ: &[Vec<usize>], d: &mut 構築データ) {
    d.prev[now] = prev;
    d.入時刻[now] = d.時刻;
    d.時刻 += 1;
    for &next in &グラフ[now] {
        if next as i64 != prev {
            d.深さ[next] = d.深さ[now] + 1;
            探索(next, now as i64, グラフ, d);
        }
    }
    d.出時刻[now] = d.時刻;
    d.時刻 += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    // 素朴な祖先判定（親を根までたどる）
    fn 親配列(隣接: &[Vec<usize>], 根: usize) -> Vec<i64> {
        let n = 隣接.len();
        let mut 親 = vec![-1i64; n];
        let mut スタック = vec![(根, -1i64)];
        while let Some((v, p)) = スタック.pop() {
            親[v] = p;
            for &u in &隣接[v] {
                if u as i64 != p {
                    スタック.push((u, v as i64));
                }
            }
        }
        親
    }

    fn 祖先か素朴(親: &[i64], a: usize, b: usize) -> bool {
        let mut c = b as i64;
        while c != -1 {
            if c as usize == a {
                return true;
            }
            c = 親[c as usize];
        }
        false
    }

    #[test]
    fn lca検証() {
        // 0-1, 0-2, 1-3, 1-4, 2-5
        let vt = 仮想木::new(6, &[(0, 1), (0, 2), (1, 3), (1, 4), (2, 5)], 0);
        assert_eq!(vt.lca(3, 4), 1);
        assert_eq!(vt.lca(3, 5), 0);
        assert_eq!(vt.lca(3, 1), 1);
        assert_eq!(vt.lca(5, 5), 5);
    }

    #[test]
    fn 仮想木が素朴と一致() {
        // やや大きめの木
        let 辺 = [
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (3, 6),
            (3, 7),
            (2, 5),
            (5, 8),
            (5, 9),
        ];
        let vt = 仮想木::new(10, &辺, 0);
        let 親 = 親配列(&無向グラフ(10, &辺), 0);
        for 集合 in [
            vec![6, 7, 9],
            vec![6, 8, 9],
            vec![4, 7, 8],
            vec![6, 7, 8, 9],
            vec![3],
        ] {
            let (頂点, 辺) = vt.構築(&集合);
            // 頂点は「集合 ∪ ペア LCA の閉包」
            let mut 期待頂点: std::collections::BTreeSet<usize> = 集合.iter().copied().collect();
            for &a in &集合 {
                for &b in &集合 {
                    期待頂点.insert(vt.lca(a, b));
                }
            }
            let 実頂点: std::collections::BTreeSet<usize> = 頂点.iter().copied().collect();
            assert_eq!(実頂点, 期待頂点, "集合={集合:?}");
            // 各辺 (親, 子): 親は子の祖先で、間に他の頂点集合メンバーがいない（最深の祖先）
            for &(p, c) in &辺 {
                assert!(祖先か素朴(&親, p, c), "{p} は {c} の祖先でない");
                // p と c の間（c の真の祖先で p の真の子孫）に頂点集合の要素がない
                let mut x = 親[c];
                while x != -1 && x as usize != p {
                    assert!(!実頂点.contains(&(x as usize)), "間に頂点がある");
                    x = 親[x as usize];
                }
            }
            // 辺数 = 頂点数 - 1（連結）
            assert_eq!(辺.len(), 頂点.len() - 1);
        }
    }
}
