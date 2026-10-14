use cargo_snippet::snippet;

#[snippet("半開_区間")]
#[snippet("RangeBounds_変換")]
#[snippet("範囲を_半開に")]
pub fn 半開(範囲: impl std::ops::RangeBounds<usize>, n: usize) -> (usize, usize) {
    use std::ops::Bound::{Excluded, Included, Unbounded};
    let l = match 範囲.start_bound() {
        Included(&x) => x,
        Excluded(&x) => x + 1,
        Unbounded => 0,
    };
    let r = match 範囲.end_bound() {
        Included(&x) => x + 1,
        Excluded(&x) => x,
        Unbounded => n,
    };
    (l, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 各種の範囲記法を解決できる() {
        assert_eq!(半開(2..5, 10), (2, 5));
        assert_eq!(半開(2..=5, 10), (2, 6));
        assert_eq!(半開(..5, 10), (0, 5));
        assert_eq!(半開(..=5, 10), (0, 6));
        assert_eq!(半開(2.., 10), (2, 10));
        assert_eq!(半開(.., 10), (0, 10));
    }

    #[test]
    fn 空区間もそのまま返す() {
        assert_eq!(半開(3..3, 10), (3, 3));
        assert_eq!(半開(0..0, 0), (0, 0));
        assert_eq!(半開(.., 0), (0, 0));
    }

    #[test]
    fn 除外始点は1つ進む() {
        use std::ops::Bound::{Excluded, Unbounded};
        assert_eq!(半開((Excluded(2), Unbounded), 10), (3, 10));
    }
}
