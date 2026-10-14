use cargo_snippet::snippet;

#[snippet("橋")]
#[snippet("関節点")]
#[snippet("lowlink")]
#[snippet("LowLink")]
#[snippet("橋と_関節点")]
/// 無向グラフの橋（取り除くと連結成分が増える辺）と関節点（取り除くと連結成分が増える頂点）を
/// lowlink で求める。構築 O(V + E)。非連結でも全成分を処理する。
///
/// 多重辺がある場合、同じ 2 頂点を結ぶ辺が 2 本以上あればその辺は橋にならない（辺番号で区別）。
///
/// - [`Self::橋一覧`] : 橋を `(u, v)`（u < v）の昇順で返す。
/// - [`Self::関節点一覧`] : 関節点を昇順で返す。
struct ローリンク {
    n: usize,
    グラフ: Vec<Vec<(usize, usize)>>, // (相手, 辺番号)
    辺数: usize,
}

#[snippet("橋")]
#[snippet("関節点")]
#[snippet("lowlink")]
#[snippet("LowLink")]
#[snippet("橋と_関節点")]
/// DFS 中に使い回す作業領域。
struct 作業領域 {
    カウンタ: usize,
    ord: Vec<i64>,
    low: Vec<usize>,
    橋: Vec<(usize, usize)>,
    関節: Vec<bool>,
}

#[snippet("橋")]
#[snippet("関節点")]
#[snippet("lowlink")]
#[snippet("LowLink")]
#[snippet("橋と_関節点")]
impl ローリンク {
    /// 頂点数 `n` の空グラフを作る。
    fn new(n: usize) -> Self {
        Self {
            n,
            グラフ: vec![Vec::new(); n],
            辺数: 0,
        }
    }

    /// 無向辺 `u - v` を追加する。多重辺・自己ループも許容する。
    fn 辺追加(&mut self, u: usize, v: usize) {
        let id = self.辺数;
        self.グラフ[u].push((v, id));
        self.グラフ[v].push((u, id));
        self.辺数 += 1;
    }

    /// DFS で `ord`（行きがけ順）と `low` を計算し、橋・関節点を収集する。
    fn 探索(&self, now: usize, 親辺: usize, 根か: bool, w: &mut 作業領域) {
        w.ord[now] = w.カウンタ as i64;
        w.low[now] = w.カウンタ;
        w.カウンタ += 1;
        let mut 子数 = 0usize;
        let mut 関節点フラグ = false;
        for &(next, 辺) in &self.グラフ[now] {
            if 辺 == 親辺 {
                continue; // 親へ戻る辺（この 1 本だけ）はスキップ
            }
            if w.ord[next] == -1 {
                子数 += 1;
                self.探索(next, 辺, false, w);
                w.low[now] = w.low[now].min(w.low[next]);
                // 橋判定: 子の low が now より深い順序なら now-next は橋
                if w.low[next] > w.ord[now] as usize {
                    let (a, b) = if now < next { (now, next) } else { (next, now) };
                    w.橋.push((a, b));
                }
                // 関節点判定（非根）
                if !根か && w.low[next] >= w.ord[now] as usize {
                    関節点フラグ = true;
                }
            } else {
                w.low[now] = w.low[now].min(w.ord[next] as usize);
            }
        }
        if 根か {
            // 根は子が 2 つ以上で関節点
            if 子数 >= 2 {
                w.関節[now] = true;
            }
        } else if 関節点フラグ {
            w.関節[now] = true;
        }
    }

    fn 計算(&self) -> (Vec<(usize, usize)>, Vec<bool>) {
        let mut w = 作業領域 {
            カウンタ: 0,
            ord: vec![-1i64; self.n],
            low: vec![0usize; self.n],
            橋: Vec::new(),
            関節: vec![false; self.n],
        };
        for s in 0..self.n {
            if w.ord[s] == -1 {
                self.探索(s, !0, true, &mut w);
            }
        }
        (w.橋, w.関節)
    }

    /// 橋を `(u, v)`（u < v）の昇順で返す。
    fn 橋一覧(&self) -> Vec<(usize, usize)> {
        let (mut 橋, _) = self.計算();
        橋.sort_unstable();
        橋
    }

    /// 関節点を昇順で返す。
    fn 関節点一覧(&self) -> Vec<usize> {
        let (_, 関節) = self.計算();
        (0..self.n).filter(|&v| 関節[v]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 一本道は全辺が橋で中間が関節点() {
        // 0 - 1 - 2 - 3
        let mut g = ローリンク::new(4);
        for (u, v) in [(0, 1), (1, 2), (2, 3)] {
            g.辺追加(u, v);
        }
        assert_eq!(g.橋一覧(), vec![(0, 1), (1, 2), (2, 3)]);
        assert_eq!(g.関節点一覧(), vec![1, 2]);
    }

    #[test]
    fn 閉路には橋も関節点も無い() {
        // 三角形 0-1-2-0
        let mut g = ローリンク::new(3);
        for (u, v) in [(0, 1), (1, 2), (2, 0)] {
            g.辺追加(u, v);
        }
        assert!(g.橋一覧().is_empty());
        assert!(g.関節点一覧().is_empty());
    }

    #[test]
    fn 多重辺は橋にならない() {
        // 0 と 1 を 2 本で結ぶ → どちらの辺も橋でない、関節点も無い
        let mut g = ローリンク::new(2);
        g.辺追加(0, 1);
        g.辺追加(0, 1);
        assert!(g.橋一覧().is_empty());
        assert!(g.関節点一覧().is_empty());
    }

    #[test]
    fn 二つの三角形を一辺で繋ぐ() {
        // 三角形 {0,1,2} と {3,4,5} を辺 2-3 で連結
        let mut g = ローリンク::new(6);
        for (u, v) in [(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3), (2, 3)] {
            g.辺追加(u, v);
        }
        assert_eq!(g.橋一覧(), vec![(2, 3)]);
        // 2 と 3 が関節点
        assert_eq!(g.関節点一覧(), vec![2, 3]);
    }

    #[test]
    fn 非連結でも各成分を処理() {
        // 成分1: 0-1（橋）, 成分2: 2-3-4-2（三角形, 橋なし）
        let mut g = ローリンク::new(5);
        for (u, v) in [(0, 1), (2, 3), (3, 4), (4, 2)] {
            g.辺追加(u, v);
        }
        assert_eq!(g.橋一覧(), vec![(0, 1)]);
        assert!(g.関節点一覧().is_empty());
    }
}
