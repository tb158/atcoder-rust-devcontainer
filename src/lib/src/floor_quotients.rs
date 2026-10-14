use cargo_snippet::snippet;

#[snippet("商の_列挙")]
#[snippet("床関数の_列挙")]
#[snippet("調和_級数の_列挙")]
#[snippet("n_割る_i_の_列挙")]
#[snippet("約数_ブロック")]
/// `⌊n/i⌋`（`i = 1..=n`）の値が一定になる `i` の区間をまとめて列挙する。O(√n)。
///
/// 各要素は `(商, 左, 右)`：`i ∈ [左, 右]` の範囲で `⌊n/i⌋ == 商`。区間は `i` の昇順、
/// `商` は降順に並ぶ。`Σ_{i=1}^{n} ⌊n/i⌋` や約数関数の和などを O(√n) で計算するのに使う。
fn 商の列挙(n: u64) -> Vec<(u64, u64, u64)> {
    let mut res = Vec::new();
    let mut i = 1;
    while i <= n {
        let 商 = n / i;
        let 右 = n / 商; // この商を保てる最大の i
        res.push((商, i, 右));
        i = 右 + 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 区間が網羅的かつ正しい() {
        let n = 10;
        let v = 商の列挙(n);
        assert_eq!(
            v,
            vec![(10, 1, 1), (5, 2, 2), (3, 3, 3), (2, 4, 5), (1, 6, 10)]
        );
        // 区間で素朴な ⌊n/i⌋ を再現できる
        for &(商, 左, 右) in &v {
            for i in 左..=右 {
                assert_eq!(n / i, 商);
            }
        }
    }

    #[test]
    fn 総和をブロックで計算() {
        let n = 100;
        let 素朴: u64 = (1..=n).map(|i| n / i).sum();
        let ブロック: u64 = 商の列挙(n)
            .iter()
            .map(|&(商, 左, 右)| 商 * (右 - 左 + 1))
            .sum();
        assert_eq!(素朴, ブロック);
    }
}
