use cargo_snippet::snippet;

#[snippet("オイラーの_φ_関数")]
#[snippet("オイラーの_トーシェント")]
#[snippet("トーシェント_関数")]
#[snippet("totient")]
#[snippet("互いに素な_個数")]
/// オイラーのφ関数 φ(n)：`1..=n` のうち `n` と互いに素な整数の個数。O(√n)。
///
/// 多くの `n` について一括で欲しいときは [`オイラーのファイ表`] を使う。
fn オイラーのファイ(mut n: u64) -> u64 {
    let mut 結果 = n;
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            結果 -= 結果 / p;
        }
        p += 1;
    }
    if n > 1 {
        結果 -= 結果 / n;
    }
    結果
}

#[snippet("オイラーの_φ_関数")]
#[snippet("オイラーの_トーシェント")]
#[snippet("トーシェント_関数")]
#[snippet("totient")]
#[snippet("互いに素な_個数")]
/// `0..=n` の各 `i` について φ(i) を入れた配列を篩で前計算する。O(n log log n)。
///
/// `表[0] = 0`、`表[1] = 1`。
fn オイラーのファイ表(n: usize) -> Vec<u64> {
    let mut 表: Vec<u64> = (0..=n as u64).collect();
    for i in 2..=n {
        if 表[i] == i as u64 {
            // i は素数
            let mut j = i;
            while j <= n {
                表[j] -= 表[j] / i as u64;
                j += i;
            }
        }
    }
    表
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 素朴ファイ(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        (1..=n).filter(|&k| num::integer::gcd(k, n) == 1).count() as u64
    }

    #[test]
    fn 単体ファイが素朴解と一致() {
        for n in 1..=200u64 {
            assert_eq!(オイラーのファイ(n), 素朴ファイ(n), "n={n}");
        }
    }

    #[test]
    fn 既知値() {
        assert_eq!(オイラーのファイ(1), 1);
        assert_eq!(オイラーのファイ(6), 2); // 1,5
        assert_eq!(オイラーのファイ(10), 4); // 1,3,7,9
        assert_eq!(オイラーのファイ(12), 4); // 1,5,7,11
        assert_eq!(オイラーのファイ(97), 96); // 素数
    }

    #[test]
    fn 表が単体と一致() {
        let n = 300;
        let 表 = オイラーのファイ表(n);
        assert_eq!(表[0], 0);
        for (i, &v) in 表.iter().enumerate().skip(1) {
            assert_eq!(v, オイラーのファイ(i as u64), "i={i}");
        }
    }
}
