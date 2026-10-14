use cargo_snippet::snippet;

#[snippet("支配木", include = "有向_グラフ")]
#[snippet("DominatorTree")]
#[snippet("支配点")]
#[snippet("idom")]
/// 支配木（dominator tree）を求める。`根` から各頂点への全経路が必ず通る頂点のうち
/// 最も近いもの（直接支配点 idom）を返す。`辺` は AtCoder の入力そのまま
/// （0-indexed の有向辺 `(u, v)` = `u → v`）。
///
/// 返り値 `idom`：`idom[根] = 根`、到達可能な `v` は `idom[v]` がその直接支配点、到達不能な頂点は
/// `!0`。制御フロー解析や「ある頂点を経由しないと到達できない頂点」の判定に使う。
///
/// 最悪計算量は Lengauer–Tarjan の `O((V+E) log V)` に劣るが、実グラフでは十分高速。
fn 支配木(n: usize, 辺: &[(usize, usize)], 根: usize) -> Vec<usize> {
    let mut g = petgraph::graph::DiGraph::<(), ()>::new();
    let 節: Vec<petgraph::graph::NodeIndex> = (0..n).map(|_| g.add_node(())).collect();
    for &(v, u) in 辺 {
        g.add_edge(節[v], 節[u], ());
    }
    let dom = petgraph::algo::dominators::simple_fast(&g, 節[根]);
    let mut 結果 = vec![!0; n];
    結果[根] = 根;
    for (v, &節v) in 節.iter().enumerate() {
        if v != 根
            && let Some(d) = dom.immediate_dominator(節v)
        {
            結果[v] = d.index();
        }
    }
    結果
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_build::有向グラフ;

    const 無: usize = usize::MAX;

    // 根から v へ到達できるか（x を除いたグラフで）。x == 無 なら除去なし。
    fn 到達(n: usize, 隣接: &[Vec<usize>], 根: usize, 除外: usize) -> Vec<bool> {
        let mut 可 = vec![false; n];
        if 除外 == 根 {
            return 可;
        }
        可[根] = true;
        let mut スタック = vec![根];
        while let Some(v) = スタック.pop() {
            for &u in &隣接[v] {
                if u != 除外 && !可[u] {
                    可[u] = true;
                    スタック.push(u);
                }
            }
        }
        可
    }

    // 素朴: x を除くと到達不能になる頂点の集合から idom を求める。
    fn 素朴支配木(n: usize, 隣接: &[Vec<usize>], 根: usize) -> Vec<usize> {
        let 到達元 = 到達(n, 隣接, 根, 無);
        // Dom[w] = w を支配する頂点集合（根・w 自身を含む）。
        let mut dom = vec![std::collections::BTreeSet::new(); n];
        for w in 0..n {
            if 到達元[w] {
                dom[w].insert(根);
                dom[w].insert(w);
            }
        }
        for x in 0..n {
            if x == 根 || !到達元[x] {
                continue;
            }
            let 除外到達 = 到達(n, 隣接, 根, x);
            for w in 0..n {
                if 到達元[w] && w != x && !除外到達[w] {
                    dom[w].insert(x); // x は w を支配
                }
            }
        }
        // idom[w] = Dom[w]\{w} のうち Dom が最大（= w に最も近い支配点）。
        let mut idom = vec![無; n];
        idom[根] = 根;
        for w in 0..n {
            if !到達元[w] || w == 根 {
                continue;
            }
            let mut 最良 = 無;
            let mut 最大 = 0;
            for &d in &dom[w] {
                if d != w && dom[d].len() >= 最大 {
                    最大 = dom[d].len();
                    最良 = d;
                }
            }
            idom[w] = 最良;
        }
        idom
    }

    #[test]
    fn 小さな例() {
        // 0->1, 0->2, 1->3, 2->3, 3->4
        let idom = 支配木(5, &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4)], 0);
        assert_eq!(idom, vec![0, 0, 0, 0, 3]);
    }

    #[test]
    fn 乱数で素朴と一致() {
        let mut seed = 0xd0d1_7a70u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..400 {
            let n = (乱数() as usize % 8) + 1;
            let 本数 = 乱数() as usize % (n * 2 + 1);
            let mut 辺 = Vec::new();
            for _ in 0..本数 {
                let u = 乱数() as usize % n;
                let v = 乱数() as usize % n;
                辺.push((u, v));
            }
            let g = 有向グラフ(n, &辺);
            let 根 = 乱数() as usize % n;
            let 得 = 支配木(n, &辺, 根);
            let 期待 = 素朴支配木(n, &g, 根);
            assert_eq!(得, 期待, "n={n} 根={根} 辺={辺:?}");
        }
    }
}
