use cargo_snippet::snippet;

#[snippet("ダブリング")]
#[snippet("doubling")]
#[snippet("k_回_移動")]
/// ダブリング。「各頂点から1回で進む先」を与えると、任意の頂点から
/// `k` 回進んだ先を O(log k) で求められるよう前処理する。
///
/// 各頂点の出次数が 1 のグラフ（ファンクショナルグラフ）での「k 個先」、
/// 「同じ写像を k 回適用した結果」などに使う。
struct ダブリング {
    /// `table[j][v]` = `v` から `2^j` 回進んだ先
    table: Vec<Vec<usize>>,
}

#[snippet("ダブリング")]
#[snippet("doubling")]
#[snippet("k_回_移動")]
impl ダブリング {
    /// `next[v]` = `v` から1回で進む先（`0..next.len()`）。
    /// `最大回数` までのジャンプに対応するテーブルを構築する。O(N log(最大回数))。
    fn new(next: &[usize], 最大回数: usize) -> Self {
        let n = next.len();
        let log = (usize::BITS - 最大回数.leading_zeros()).max(1) as usize;
        let mut table = vec![vec![0usize; n]; log];
        table[0].copy_from_slice(next);
        for j in 1..log {
            for v in 0..n {
                table[j][v] = table[j - 1][table[j - 1][v]];
            }
        }
        Self { table }
    }

    /// `始点` から `回数` 回進んだ先の頂点を返す。`回数 <= 最大回数` であること。O(log 回数)。
    fn 進む(&self, 始点: usize, mut 回数: usize) -> usize {
        let mut v = 始点;
        let mut j = 0;
        while 回数 > 0 {
            if 回数 & 1 == 1 {
                v = self.table[j][v];
            }
            回数 >>= 1;
            j += 1;
        }
        v
    }

    /// `始点` から進み続け、到達頂点が `条件` を満たす限り進めるだけ進んだ回数を返す。O(log)。
    ///
    /// `条件` は経路上で単調（一度偽になったら以降ずっと偽）であることが前提。
    /// 添字が単調増加する遷移（次がより大きい添字を指す）で「上限以下に留まれる最大回数」などに使う。
    /// 例: `d.進める回数(始点, |v| v <= 上限)`。返り値はそのまま [`進む`](Self::進む) に渡せる。
    fn 進める回数(&self, 始点: usize, mut 条件: impl FnMut(usize) -> bool) -> usize {
        let mut 現在 = 始点;
        let mut 回数 = 0;
        for j in (0..self.table.len()).rev() {
            let 行先 = self.table[j][現在];
            if 条件(行先) {
                回数 += 1 << j;
                現在 = 行先;
            }
        }
        回数
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 環状の遷移() {
        // 0->1->2->3->4->0 のサイクル
        let next = vec![1, 2, 3, 4, 0];
        let d = ダブリング::new(&next, 1_000_000_000);
        assert_eq!(d.進む(0, 0), 0);
        assert_eq!(d.進む(0, 1), 1);
        assert_eq!(d.進む(0, 5), 0); // 一周
        assert_eq!(d.進む(2, 6), 3); // 2 -> ... 6回
        assert_eq!(d.進む(0, 1_000_000_000), 0); // 1e9 % 5 == 0
    }

    #[test]
    fn 条件を満たす限り進める回数() {
        // 単調増加チェーン 0->1->2->3->4->5（5 は自己ループの番兵）
        let next = vec![1, 2, 3, 4, 5, 5];
        let d = ダブリング::new(&next, next.len());
        // 到達添字が 3 以下に留まる最大回数: 0->1->2->3 で 3 回。
        assert_eq!(d.進める回数(0, |v| v <= 3), 3);
        // 返り値はそのまま 進む に渡せて到達頂点に一致する。
        assert_eq!(d.進む(0, d.進める回数(0, |v| v <= 3)), 3);
        // 4 以下なら 4 に到達して 4 回。
        assert_eq!(d.進める回数(0, |v| v <= 4), 4);
        // 一歩も進めない条件は 0 回。
        assert_eq!(d.進める回数(2, |_| false), 0);
        // 途中の頂点から: 2->3 で 1 回。
        assert_eq!(d.進める回数(2, |v| v <= 3), 1);
    }
}
