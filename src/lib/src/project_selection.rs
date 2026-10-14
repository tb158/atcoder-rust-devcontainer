use cargo_snippet::snippet;

#[snippet("最大_重み_閉包")]
#[snippet("燃やす_埋める")]
#[snippet("ProjectSelection")]
#[snippet("MaxWeightClosure")]
/// 最大重み閉包問題（Project Selection／いわゆる「燃やす埋める」）。
///
/// 各要素に整数重み `w_i`（負も可）があり、「要素 `i` を選ぶなら `j` も必ず選ぶ」という依存
/// （有向辺 `i→j`）が課されている。依存を満たす部分集合 `S`（`i∈S ⇒ j∈S`）のうち重み総和
/// `Σ_{i∈S} w_i` を最大にするものを求める。空集合も選択可能なので答えは常に `0` 以上。
///
/// 内部で最小カット（[`ac_library::MfGraph`] の Dinic）へ帰着する。
struct 最大重み閉包 {
    重み: Vec<i64>,
    依存: Vec<(usize, usize)>, // i を選ぶなら j も選ぶ
}

#[snippet("最大_重み_閉包")]
#[snippet("燃やす_埋める")]
#[snippet("ProjectSelection")]
#[snippet("MaxWeightClosure")]
impl 最大重み閉包 {
    /// 各要素の重み（負も可）を与えて初期化する。
    fn new(重み: Vec<i64>) -> Self {
        最大重み閉包 {
            重み,
            依存: Vec::new(),
        }
    }

    /// 依存「要素 `i` を選ぶなら `j` も選ぶ」を追加する。
    fn 依存追加(&mut self, i: usize, j: usize) {
        assert!(i < self.重み.len() && j < self.重み.len());
        self.依存.push((i, j));
    }

    /// 最大重みと、それを達成する選択集合 `選択[i]` を返す。`O(最大流)`。
    fn 解く(&self) -> (i64, Vec<bool>) {
        use ac_library::MfGraph;
        const INF: i64 = 1 << 60;
        let n = self.重み.len();
        let (源, 匯) = (n, n + 1);
        let mut g = MfGraph::<i64>::new(n + 2);
        let mut 正の和 = 0i64;
        for (i, &w) in self.重み.iter().enumerate() {
            if w > 0 {
                g.add_edge(源, i, w);
                正の和 += w;
            } else if w < 0 {
                g.add_edge(i, 匯, -w);
            }
        }
        for &(i, j) in &self.依存 {
            g.add_edge(i, j, INF);
        }
        let カット = g.flow(源, 匯);
        let 到達 = g.min_cut(源); // 源から残余で到達できる頂点 = 源側
        let 選択 = (0..n).map(|i| 到達[i]).collect();
        (正の和 - カット, 選択)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 全部分集合を総当たりして、依存を満たす最大重みを求める独立オラクル。
    fn 総当たり(重み: &[i64], 依存: &[(usize, usize)]) -> i64 {
        let n = 重み.len();
        let mut 最良 = 0i64; // 空集合
        for マスク in 0u32..(1 << n) {
            let 入る = |k: usize| マスク >> k & 1 == 1;
            // 依存充足: i が入るなら j も入る。
            if 依存.iter().all(|&(i, j)| !入る(i) || 入る(j)) {
                let w: i64 = (0..n).filter(|&k| 入る(k)).map(|k| 重み[k]).sum();
                最良 = 最良.max(w);
            }
        }
        最良
    }

    #[test]
    fn 既知例() {
        // 利益 10 の仕事には費用 4 の前提が必要。両方取ると 6 > 0。
        let mut p = 最大重み閉包::new(vec![10, -4]);
        p.依存追加(0, 1);
        let (値, 選) = p.解く();
        assert_eq!(値, 6);
        assert_eq!(選, vec![true, true]);

        // 費用が利益を上回るなら何も選ばない。
        let mut p = 最大重み閉包::new(vec![3, -10]);
        p.依存追加(0, 1);
        let (値, 選) = p.解く();
        assert_eq!(値, 0);
        assert_eq!(選, vec![false, false]);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x51d0_3a40u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..4000 {
            let n = (乱数() as usize % 7) + 1; // 1..=7
            let 重み: Vec<i64> = (0..n).map(|_| (乱数() % 21) as i64 - 10).collect();
            let e = 乱数() as usize % (n * 2 + 1);
            let 依存: Vec<(usize, usize)> = (0..e)
                .map(|_| {
                    let i = 乱数() as usize % n;
                    let j = 乱数() as usize % n;
                    (i, j)
                })
                .filter(|&(i, j)| i != j)
                .collect();
            let mut p = 最大重み閉包::new(重み.clone());
            for &(i, j) in &依存 {
                p.依存追加(i, j);
            }
            let (値, 選) = p.解く();
            let 期待 = 総当たり(&重み, &依存);
            assert_eq!(値, 期待, "重み={重み:?} 依存={依存:?}");
            // 返した選択集合が依存を満たし、重みが値に一致することも確認。
            assert!(
                依存.iter().all(|&(i, j)| !選[i] || 選[j]),
                "選択 {選:?} が依存を破る 依存={依存:?}"
            );
            let 実重み: i64 = (0..n).filter(|&k| 選[k]).map(|k| 重み[k]).sum();
            assert_eq!(実重み, 値, "選択集合の重みが値と不一致");
        }
    }
}
