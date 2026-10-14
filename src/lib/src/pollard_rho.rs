use crate::mod_pow::累乗_mod;
use cargo_snippet::snippet;

#[snippet("ポラード_ロー", include = "累乗_mod")]
#[snippet("高速_素因数_分解")]
#[snippet("ミラー_ラビン")]
#[snippet("素数_判定")]
#[snippet("PollardRho")]
/// `a * b mod m` を u128 経由でオーバーフローさせずに計算する。
fn 乗算_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

#[snippet("ポラード_ロー")]
#[snippet("高速_素因数_分解")]
#[snippet("ミラー_ラビン")]
#[snippet("素数_判定")]
#[snippet("PollardRho")]
/// ミラーラビン素数判定。底 `{2,3,5,7,11,13,17,19,23,29,31,37}` を使うと u64 全域で決定的。
/// 1 回あたり O(log^2 n)。`primal` の篩は上限までしか扱えないが、こちらは単一の巨大数（〜10^18）を直接判定できる。
fn ミラーラビン素数判定(n: impl TryInto<u64>) -> bool {
    let n: u64 = n
        .try_into()
        .ok()
        .expect("ミラーラビン素数判定: 引数が u64 に収まりません");
    const 底集合: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    if n < 2 {
        return false;
    }
    for &p in &底集合 {
        if n % p == 0 {
            return n == p;
        }
    }
    // n - 1 = d * 2^r （d は奇数）
    let mut d = n - 1;
    let mut r = 0u32;
    while d & 1 == 0 {
        d >>= 1;
        r += 1;
    }
    'witness: for &a in &底集合 {
        let mut x = 累乗_mod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..r - 1 {
            x = 乗算_mod(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

#[snippet("ポラード_ロー")]
#[snippet("高速_素因数_分解")]
#[snippet("ミラー_ラビン")]
#[snippet("素数_判定")]
#[snippet("PollardRho")]
/// 合成数 `n` の自明でない約数を 1 つ返す（Pollard の rho 法）。
fn 単一因数(n: u64) -> u64 {
    if n & 1 == 0 {
        return 2;
    }
    let mut c = 1u64;
    loop {
        let f = |x: u64| (乗算_mod(x, x, n) + c) % n;
        let mut x = 2u64;
        let mut y = 2u64;
        let mut d = 1u64;
        while d == 1 {
            x = f(x);
            y = f(f(y));
            let 差 = x.abs_diff(y);
            d = num::integer::gcd(差, n);
        }
        if d != n {
            return d;
        }
        c += 1;
    }
}

#[snippet("ポラード_ロー")]
#[snippet("高速_素因数_分解")]
#[snippet("ミラー_ラビン")]
#[snippet("素数_判定")]
#[snippet("PollardRho")]
/// `n`（>= 1）の素因数を重複込みで昇順に返す。`n == 1` のときは空。
/// 期待計算量 O(n^{1/4}) 程度で、`prime_number` の線形篩（上限 5×10^7）が扱えない巨大数も分解できる。
fn 素因数分解(n: impl TryInto<u64>) -> Vec<usize> {
    let n: u64 = n
        .try_into()
        .ok()
        .expect("素因数分解: 引数が u64 に収まりません");
    fn 再帰(n: u64, 結果: &mut Vec<u64>) {
        if n == 1 {
            return;
        }
        if ミラーラビン素数判定(n) {
            結果.push(n);
            return;
        }
        let d = 単一因数(n);
        再帰(d, 結果);
        再帰(n / d, 結果);
    }
    let mut 結果 = Vec::new();
    再帰(n, &mut 結果);
    結果.sort_unstable();
    結果.into_iter().map(|p| p as usize).collect()
}

#[snippet("ポラード_ロー")]
#[snippet("高速_素因数_分解")]
#[snippet("ミラー_ラビン")]
#[snippet("素数_判定")]
#[snippet("PollardRho")]
/// `n` の素因数と指数の昇順リスト `(素因数, 指数)`。
fn 素因数分解指数付き(n: impl TryInto<u64>) -> Vec<(usize, usize)> {
    let mut 結果: Vec<(usize, usize)> = Vec::new();
    for p in 素因数分解(n) {
        if let Some(末尾) = 結果.last_mut()
            && 末尾.0 == p
        {
            末尾.1 += 1;
            continue;
        }
        結果.push((p, 1));
    }
    結果
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 既知の素数と合成数を判定() {
        for &p in &[
            2u64,
            3,
            97,
            1_000_000_007,
            998_244_353,
            1_000_000_000_000_000_003,
        ] {
            assert!(ミラーラビン素数判定(p), "{p} は素数のはず");
        }
        for &c in &[1u64, 4, 100, 1_000_000_006, 1_000_000_007 * 2] {
            assert!(!ミラーラビン素数判定(c), "{c} は合成数のはず");
        }
    }

    #[test]
    fn 小さな数を分解() {
        assert_eq!(素因数分解(1), Vec::<usize>::new());
        assert_eq!(素因数分解(12), vec![2, 2, 3]);
        assert_eq!(素因数分解(97), vec![97]);
        assert_eq!(素因数分解指数付き(360), vec![(2, 3), (3, 2), (5, 1)]);
    }

    #[test]
    fn 巨大な半素数を分解() {
        // 大きな素数の積
        let a = 1_000_000_007u64;
        let b = 998_244_353u64;
        let n = a * b;
        assert_eq!(素因数分解(n), vec![b as usize, a as usize]); // 昇順
    }

    #[test]
    fn 乱数で積が元に戻り全因数が素数() {
        let mut seed = 0x1234_5678u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 1
        };
        for _ in 0..200 {
            let n = (乱数() % 1_000_000_000_000u64) + 2;
            let 因数 = 素因数分解(n);
            let 積: u64 = 因数.iter().map(|&p| p as u64).product();
            assert_eq!(積, n, "n={n}");
            for &p in &因数 {
                assert!(ミラーラビン素数判定(p), "n={n} の因数 {p} が素数でない");
            }
            // 昇順であること
            assert!(因数.windows(2).all(|w| w[0] <= w[1]));
        }
    }
}
