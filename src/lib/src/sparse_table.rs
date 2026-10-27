use crate::range_bounds::半開;
use cargo_snippet::snippet;

#[snippet("スパース_テーブル", include = "半開_区間")]
#[snippet("SparseTable")]
#[snippet("区間_min")]
#[snippet("区間_max")]
/// スパーステーブル。静的（更新なし）な配列に対し、区間 min / max / gcd など
/// 「冪等な二項演算」の区間クエリを前処理 O(N log N)・各クエリ O(1) で答える。
///
/// `merge` は結合的かつ冪等（`merge(x, x) == x`）な演算であること（min, max, gcd, &, | など）。
/// 和のような非冪等な演算には使えない（その用途は累積和やセグメント木を使う）。
struct スパーステーブル<T, F> {
    table: Vec<Vec<T>>,
    merge: F,
}

#[snippet("スパース_テーブル")]
#[snippet("SparseTable")]
#[snippet("区間_min")]
#[snippet("区間_max")]
impl<T: Copy, F: Fn(T, T) -> T> スパーステーブル<T, F> {
    /// 配列と冪等な二項演算 `merge` からスパーステーブルを構築する。
    ///
    /// # 例
    /// ```ignore
    /// let st = スパーステーブル::new(&a, |x, y| x.min(y)); // 区間 min
    /// ```
    fn new(配列: &[T], merge: F) -> Self {
        let n = 配列.len();
        let mut table = vec![配列.to_vec()];
        let mut 幅 = 1;
        while 幅 * 2 <= n {
            let prev = table.last().unwrap();
            let 行: Vec<T> = (0..=n - 幅 * 2).map(|i| merge(prev[i], prev[i + 幅])).collect();
            table.push(行);
            幅 *= 2;
        }
        Self { table, merge }
    }

    /// 区間（`a..b` / `a..=b` / `..` など）に対する `merge` の畳み込み結果を O(1) で返す。
    /// 空区間は問い合わせ不可。
    fn 区間(&self, 範囲: impl std::ops::RangeBounds<usize>) -> T {
        let (l, r) = 半開(範囲, self.table[0].len());
        debug_assert!(l < r, "スパーステーブル: 空区間は問い合わせ不可");
        let j = (usize::BITS - 1 - (r - l).leading_zeros()) as usize; // floor(log2(r-l))
        (self.merge)(self.table[j][l], self.table[j][r - (1 << j)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 区間最小と最大() {
        let a = [3, 1, 4, 1, 5, 9, 2, 6];
        let st_min = スパーステーブル::new(&a, std::cmp::min);
        assert_eq!(st_min.区間(0..8), 1);
        assert_eq!(st_min.区間(..), 1); // 全体
        assert_eq!(st_min.区間(4..8), 2);
        assert_eq!(st_min.区間(2..3), 4);
        assert_eq!(st_min.区間(2..=2), 4); // 閉区間でも可

        let st_max = スパーステーブル::new(&a, std::cmp::max);
        assert_eq!(st_max.区間(0..4), 4);
        assert_eq!(st_max.区間(3..8), 9);
    }
}
