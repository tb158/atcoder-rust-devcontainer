use crate::mod_pow::累乗_mod;
use cargo_snippet::snippet;

#[snippet("原始根", include = "累乗_mod")]
#[snippet("PrimitiveRoot")]
#[snippet("位数")]
#[snippet("MultiplicativeOrder")]
/// `n` の相異なる素因数を昇順で返す（試し割り）。`n` の最大素因数が ~10^7 程度までなら高速。
fn 素因数集合(mut n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            v.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        v.push(n);
    }
    v
}

#[snippet("原始根")]
#[snippet("PrimitiveRoot")]
#[snippet("位数")]
#[snippet("MultiplicativeOrder")]
/// 素数 `p` の最小の原始根を返す。`p == 2` なら 1。
///
/// 原始根 `g` は位数が `p-1`、すなわち `g^1, g^2, ..., g^{p-1}` が `1..p` を一巡する数。NTT の回転因子や
/// 離散対数の底に使う。
/// `p` は素数であること（`p-1` の素因数分解に試し割りを使うので `p-1` の最大素因数が大きすぎないこと）。
fn 原始根(p: impl TryInto<u64>) -> usize {
    let p: u64 = p
        .try_into()
        .ok()
        .expect("原始根: 引数が u64 に収まりません");
    if p == 2 {
        return 1;
    }
    let 素因数 = 素因数集合(p - 1);
    let mut g = 2u64;
    loop {
        // すべての素因数 q について g^{(p-1)/q} != 1 なら g は原始根
        if 素因数.iter().all(|&q| 累乗_mod(g, (p - 1) / q, p) != 1) {
            return g as usize;
        }
        g += 1;
    }
}

#[snippet("原始根")]
#[snippet("PrimitiveRoot")]
#[snippet("位数")]
#[snippet("MultiplicativeOrder")]
/// 素数 `p` を法とする `a`（`a % p != 0`）の乗法的位数（`a^k ≡ 1 mod p` となる最小の正の `k`）。
/// 位数は `p-1` の約数。`p` は素数であること。
fn 位数(a: impl TryInto<u64>, p: impl TryInto<u64>) -> usize {
    let a: u64 = a.try_into().ok().expect("位数: 引数が u64 に収まりません");
    let p: u64 = p.try_into().ok().expect("位数: 引数が u64 に収まりません");
    assert!(a % p != 0, "a must not be a multiple of p");
    let mut k = p - 1;
    for q in 素因数集合(p - 1) {
        while k % q == 0 && 累乗_mod(a, k / q, p) == 1 {
            k /= q;
        }
    }
    k as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    // 総当たり: a の位数を 1 から順に確かめる。
    fn 位数素朴(a: u64, p: u64) -> u64 {
        let mut x = a % p;
        let mut k = 1;
        while x != 1 {
            x = x * a % p;
            k += 1;
        }
        k
    }

    #[test]
    fn 原始根の位数はp1() {
        for &p in &[2u64, 3, 5, 7, 11, 13, 17, 97, 998_244_353, 1_000_000_007] {
            let g = 原始根(p);
            if p == 2 {
                assert_eq!(g, 1);
                continue;
            }
            assert_eq!(位数(g, p), (p - 1) as usize, "p={p} g={g}");
        }
    }

    #[test]
    fn 位数が総当たりと一致() {
        for &p in &[3u64, 5, 7, 11, 13, 17, 19, 23] {
            for a in 1..p {
                assert_eq!(位数(a, p), 位数素朴(a, p) as usize, "p={p} a={a}");
            }
        }
    }

    #[test]
    fn 既知の原始根() {
        // 3 は 998244353 と 1e9+7 の原始根
        assert_eq!(原始根(998_244_353), 3);
        assert_eq!(原始根(1_000_000_007), 5);
    }
}
