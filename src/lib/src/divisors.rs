use cargo_snippet::snippet;

#[snippet("約数_列挙")]
#[snippet("約数")]
#[snippet("約数_全列挙")]
#[snippet("divisors")]
/// `n` の正の約数を昇順で全列挙する。O(√n)。
///
/// 素因数分解（`primal::Sieve::factor` 等）ではなく「約数そのものの一覧」が欲しいとき用。
/// 例: `12 → [1, 2, 3, 4, 6, 12]`。
fn 約数列挙(n: usize) -> Vec<usize> {
    let mut 小さい側 = Vec::new();
    let mut 大きい側 = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            小さい側.push(i);
            if i != n / i {
                大きい側.push(n / i);
            }
        }
        i += 1;
    }
    大きい側.reverse();
    小さい側.extend(大きい側);
    小さい側
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 約数を昇順で得る() {
        assert_eq!(約数列挙(12), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(約数列挙(1), vec![1]);
        assert_eq!(約数列挙(16), vec![1, 2, 4, 8, 16]); // 平方数は中央が重複しない
        assert_eq!(約数列挙(13), vec![1, 13]); // 素数
    }
}
