use cargo_snippet::snippet;

#[snippet("約数の_個数_表")]
#[snippet("約数の_和_表")]
#[snippet("約数_テーブル")]
#[snippet("約数の_個数")]
#[snippet("約数の_和")]
/// `0..=n` の各 `i` について約数の個数 d(i) を篩で前計算する。O(n log n)。
///
/// `d[0] = 0`、`d[1] = 1`。各 `i` をその倍数全てに足し込む調和級数オーダー。
/// 単一の `n` の約数列挙は [`crate::divisors`] を、まとめて欲しいときはこちらを使う。
fn 約数の個数表(n: usize) -> Vec<u64> {
    let mut d = vec![0u64; n + 1];
    for i in 1..=n {
        let mut j = i;
        while j <= n {
            d[j] += 1;
            j += i;
        }
    }
    d
}

#[snippet("約数の_個数_表")]
#[snippet("約数の_和_表")]
#[snippet("約数_テーブル")]
#[snippet("約数の_個数")]
#[snippet("約数の_和")]
/// `0..=n` の各 `i` について約数の総和 σ(i) を篩で前計算する。O(n log n)。
///
/// `s[0] = 0`、`s[1] = 1`。`i` をその倍数全てに足し込む。`σ(6) = 1+2+3+6 = 12` など。
fn 約数の和表(n: usize) -> Vec<u64> {
    let mut s = vec![0u64; n + 1];
    for i in 1..=n {
        let mut j = i;
        while j <= n {
            s[j] += i as u64;
            j += i;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 素朴約数の個数(n: u64) -> u64 {
        (1..=n).filter(|&d| n % d == 0).count() as u64
    }

    fn 素朴約数の和(n: u64) -> u64 {
        (1..=n).filter(|&d| n % d == 0).sum()
    }

    #[test]
    fn 個数表が素朴解と一致() {
        let n = 500;
        let d = 約数の個数表(n);
        assert_eq!(d[0], 0);
        for (i, &v) in d.iter().enumerate().skip(1) {
            assert_eq!(v, 素朴約数の個数(i as u64), "i={i}");
        }
    }

    #[test]
    fn 和表が素朴解と一致() {
        let n = 500;
        let s = 約数の和表(n);
        assert_eq!(s[0], 0);
        for (i, &v) in s.iter().enumerate().skip(1) {
            assert_eq!(v, 素朴約数の和(i as u64), "i={i}");
        }
    }

    #[test]
    fn 既知値() {
        let d = 約数の個数表(12);
        assert_eq!(d[12], 6); // 1,2,3,4,6,12
        let s = 約数の和表(12);
        assert_eq!(s[6], 12); // 1+2+3+6
        assert_eq!(s[1], 1);
        assert_eq!(s[12], 28); // 1+2+3+4+6+12
    }
}
