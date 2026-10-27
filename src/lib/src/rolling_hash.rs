use crate::range_bounds::半開;
use cargo_snippet::snippet;

#[snippet("ローリング_ハッシュ", include = "半開_区間")]
#[snippet("ロリハ")]
#[snippet("文字列_ハッシュ")]
/// ローリングハッシュ（mod 2^61-1）。構築 O(N) 後、任意の半開区間 `[l, r)` のハッシュを
/// O(1) で取得でき、部分文字列の一致判定を高速に行える。
///
/// 法と基数は固定の定数なので、同じ型から作った2つのインスタンス同士のハッシュは比較可能。
/// （AtCoder では衝突を狙う anti-hash ケースはまず無いが、心配なら基数を書き換える。）
struct ローリングハッシュ {
    /// 前方ハッシュ。`前方[i]` = `s[0..i]` のハッシュ
    前方: Vec<u64>,
    /// `基数^i mod 法`
    べき: Vec<u64>,
}

#[snippet("ローリング_ハッシュ")]
#[snippet("ロリハ")]
#[snippet("文字列_ハッシュ")]
impl ローリングハッシュ {
    /// バイト列（`s.as_bytes()` や `Vec<u8>`）からローリングハッシュを構築する。
    fn new(s: &[u8]) -> Self {
        const 法: u64 = (1 << 61) - 1;
        const 基数: u64 = 1_000_000_007;
        let n = s.len();
        let mut 前方 = vec![0u64; n + 1];
        let mut べき = vec![1u64; n + 1];
        for i in 0..n {
            前方[i + 1] = Self::積(前方[i], 基数, 法).wrapping_add(s[i] as u64) % 法;
            べき[i + 1] = Self::積(べき[i], 基数, 法);
        }
        Self { 前方, べき }
    }

    /// `a * b mod (2^61-1)` を u128 経由で計算する内部関数。
    fn 積(a: u64, b: u64, 法: u64) -> u64 {
        (a as u128 * b as u128 % 法 as u128) as u64
    }

    /// 区間（`a..b` / `a..=b` / `..` など）の部分文字列のハッシュ値を返す。O(1)。
    fn 区間ハッシュ(&self, 範囲: impl std::ops::RangeBounds<usize>) -> u64 {
        const 法: u64 = (1 << 61) - 1;
        let (l, r) = 半開(範囲, self.前方.len() - 1);
        let 全体 = self.前方[r];
        let prev = Self::積(self.前方[l], self.べき[r - l], 法);
        (全体 + 法 - prev) % 法
    }

    /// 2 つの区間（`a..b` / `a..=b` / `..` など）が表す部分文字列が等しいかを O(1) で判定する。
    fn 等しい(
        &self,
        区間1: impl std::ops::RangeBounds<usize>,
        区間2: impl std::ops::RangeBounds<usize>,
    ) -> bool {
        let n = self.前方.len() - 1;
        let (l1, r1) = 半開(区間1, n);
        let (l2, r2) = 半開(区間2, n);
        (r1 - l1) == (r2 - l2) && self.区間ハッシュ(l1..r1) == self.区間ハッシュ(l2..r2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 部分文字列の一致判定() {
        let s = b"abcabc";
        let rh = ローリングハッシュ::new(s);
        // "abc"(0..3) と "abc"(3..6) は等しい
        assert!(rh.等しい(0..3, 3..6));
        // 閉区間・全体指定でも書ける
        assert!(rh.等しい(0..=2, 3..=5));
        // "ab"(0..2) と "bc"(1..3) は異なる
        assert!(!rh.等しい(0..2, 1..3));
        // 長さが違えば即不一致
        assert!(!rh.等しい(0..2, 0..3));
        // 空文字列同士
        assert!(rh.等しい(0..0, 3..3));
    }

    #[test]
    fn 全文字列のハッシュ一致() {
        let rh1 = ローリングハッシュ::new(b"hello");
        let rh2 = ローリングハッシュ::new(b"xhello");
        // 別インスタンスでも基数・法が同じなので比較できる
        assert_eq!(rh1.区間ハッシュ(0..5), rh2.区間ハッシュ(1..6));
        // 全体（`..`）でも取れる
        assert_eq!(rh1.区間ハッシュ(..), rh2.区間ハッシュ(1..6));
    }
}
