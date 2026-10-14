use cargo_snippet::snippet;

#[snippet("一般_マッチング")]
#[snippet("Blossom")]
#[snippet("花_アルゴリズム")]
#[snippet("一般_グラフ_最大_マッチング")]
#[snippet("EdmondsBlossom")]
/// 一般グラフ（二部とは限らない、奇閉路を含んでよい）の最大マッチング。O(V·E)。
///
/// 二部グラフに限るなら [`crate::hopcroft_karp`] の方が高速。
///
/// - [`Self::最大マッチング`] : マッチした本数を返す（内部に相手も保持）。
/// - [`Self::相手一覧`] : 各頂点の相手（未マッチは -1）。
struct 一般マッチング {
    n: usize,
    辺: Vec<(usize, usize)>,
    相手: Vec<i64>,
}

#[snippet("一般_マッチング")]
#[snippet("Blossom")]
#[snippet("花_アルゴリズム")]
#[snippet("一般_グラフ_最大_マッチング")]
#[snippet("EdmondsBlossom")]
impl 一般マッチング {
    /// 頂点数 `n` の空グラフを作る。
    fn new(n: usize) -> Self {
        Self {
            n,
            辺: Vec::new(),
            相手: vec![-1; n],
        }
    }

    /// 無向辺 `u - v` を追加する。
    fn 辺追加(&mut self, u: usize, v: usize) {
        self.辺.push((u, v));
    }

    /// 最大マッチングの本数を求める。
    fn 最大マッチング(&mut self) -> usize {
        let mut g = petgraph::graph::UnGraph::<(), ()>::new_undirected();
        let 節: Vec<petgraph::graph::NodeIndex> = (0..self.n).map(|_| g.add_node(())).collect();
        for &(u, v) in &self.辺 {
            g.add_edge(節[u], 節[v], ());
        }
        let マッチング = petgraph::algo::maximum_matching(&g);
        self.相手 = vec![-1; self.n];
        let mut 本数 = 0;
        for (a, b) in マッチング.edges() {
            self.相手[a.index()] = b.index() as i64;
            self.相手[b.index()] = a.index() as i64;
            本数 += 1;
        }
        本数
    }

    /// 各頂点の相手（未マッチは -1）。先に [`Self::最大マッチング`] を呼ぶこと。
    fn 相手一覧(&self) -> &[i64] {
        &self.相手
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 総当たり：一般グラフの最大マッチング（再帰バックトラック）
    fn 総当たり(n: usize, 隣接行列: &[Vec<bool>]) -> usize {
        fn 再帰(n: usize, 隣接行列: &[Vec<bool>], 使用: &mut Vec<bool>) -> usize {
            let 空き = (0..n).find(|&i| !使用[i]);
            match 空き {
                None => 0,
                Some(v) => {
                    使用[v] = true;
                    let mut 最良 = 再帰(n, 隣接行列, 使用); // v を未マッチのまま
                    for u in v + 1..n {
                        if !使用[u] && 隣接行列[v][u] {
                            使用[u] = true;
                            最良 = 最良.max(1 + 再帰(n, 隣接行列, 使用));
                            使用[u] = false;
                        }
                    }
                    使用[v] = false;
                    最良
                }
            }
        }
        let mut 使用 = vec![false; n];
        再帰(n, 隣接行列, &mut 使用)
    }

    #[test]
    fn 奇閉路は最大2() {
        // C5: 0-1-2-3-4-0 の最大マッチングは 2
        let mut m = 一般マッチング::new(5);
        for (u, v) in [(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)] {
            m.辺追加(u, v);
        }
        assert_eq!(m.最大マッチング(), 2);
    }

    #[test]
    fn 三角形は1() {
        let mut m = 一般マッチング::new(3);
        for (u, v) in [(0, 1), (1, 2), (2, 0)] {
            m.辺追加(u, v);
        }
        assert_eq!(m.最大マッチング(), 1);
    }

    #[test]
    fn マッチングの整合性() {
        let mut m = 一般マッチング::new(6);
        for (u, v) in [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0), (0, 3)] {
            m.辺追加(u, v);
        }
        let 本数 = m.最大マッチング();
        let 相手 = m.相手一覧();
        // 相手関係が対称
        let mut 数 = 0;
        for v in 0..6 {
            if 相手[v] != -1 {
                assert_eq!(相手[相手[v] as usize], v as i64);
                数 += 1;
            }
        }
        assert_eq!(数 / 2, 本数);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x13579bdfu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..150 {
            let n = (乱数() as usize % 8) + 1;
            let mut 行列 = vec![vec![false; n]; n];
            let mut m = 一般マッチング::new(n);
            for u in 0..n {
                for v in u + 1..n {
                    if 乱数() % 2 == 0 {
                        行列[u][v] = true;
                        行列[v][u] = true;
                        m.辺追加(u, v);
                    }
                }
            }
            assert_eq!(m.最大マッチング(), 総当たり(n, &行列), "n={n}");
        }
    }
}
