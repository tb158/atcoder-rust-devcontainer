use crate::mod_pow::累乗_mod;
use cargo_snippet::snippet;

#[snippet("mod_平方根", include = "累乗_mod")]
#[snippet("平方_剰余")]
#[snippet("TonelliShanks")]
#[snippet("トネリ_シャンクス")]
#[snippet("mod_の_平方根")]
/// 素数 `p` を法とする平方根：`x^2 ≡ a (mod p)` を満たす `x` を 1 つ返す（無ければ `None`）。
/// Tonelli-Shanks 法。期待 O(log^2 p)。
///
/// `p` は**素数**であること。`a` は `mod p` 前で渡してよい。返り値 `x` に対し `p - x` も解。
/// `a ≡ 0` のときは `Some(0)`。平方非剰余なら `None`。`p < 2^31` 程度を想定。
fn modの平方根(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if p == 2 {
        return Some(a & 1);
    }
    // オイラーの規準：a が平方剰余か判定
    if 累乗_mod(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    // p ≡ 3 (mod 4) なら閉じた式で求まる
    if p % 4 == 3 {
        return Some(累乗_mod(a, (p + 1) / 4, p));
    }
    // p - 1 = q · 2^s（q は奇数）
    let mut q = p - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    // 平方非剰余 z を 1 つ探す
    let mut z = 2u64;
    while 累乗_mod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = 累乗_mod(z, q, p);
    let mut t = 累乗_mod(a, q, p);
    let mut r = 累乗_mod(a, q.div_ceil(2), p);
    loop {
        if t == 1 {
            return Some(r);
        }
        // t^(2^i) = 1 となる最小の i (0 < i < m) を探す
        let mut i = 0u32;
        let mut 暫定 = t;
        while 暫定 != 1 {
            暫定 = 暫定 * 暫定 % p;
            i += 1;
        }
        let b = 累乗_mod(c, 1u64 << (m - i - 1), p);
        m = i;
        c = b * b % p;
        t = t * c % p;
        r = r * b % p;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 総当たりで検証() {
        // 素数 p について全 a で確認
        for &p in &[2u64, 3, 5, 7, 11, 13, 17, 97, 998_244_353] {
            let 上限 = if p > 1000 { 1000 } else { p };
            for a in 0..上限 {
                match modの平方根(a, p) {
                    Some(x) => assert_eq!(x * x % p, a % p, "p={p}, a={a}"),
                    None => {
                        // 解が無いことの確認（p が小さいときのみ全探索）
                        if p <= 1000 {
                            assert!(
                                (0..p).all(|x| x * x % p != a % p),
                                "p={p}, a={a} は実際には解がある"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn 既知の例() {
        // 2 は mod 7 の平方剰余: 3^2=9≡2, 4^2=16≡2
        let x = modの平方根(2, 7).unwrap();
        assert!(x == 3 || x == 4);
        // 3 は mod 7 の平方非剰余
        assert_eq!(modの平方根(3, 7), None);
        assert_eq!(modの平方根(0, 13), Some(0));
    }
}
