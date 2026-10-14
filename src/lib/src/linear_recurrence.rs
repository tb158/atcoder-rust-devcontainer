use crate::mod_pow::累乗_mod;
use cargo_snippet::snippet;

#[snippet("線形_漸化式", include = "累乗_mod")]
#[snippet("ベルレカンプ_マッシー")]
#[snippet("Kitamasa")]
#[snippet("きたまさ_法")]
#[snippet("数列の_第_n_項")]
/// Berlekamp-Massey 法。数列 `数列`（素数 `法` 上）を生成する最短の線形漸化式の係数を返す。
/// O(N·L)（L は漸化式の次数）。
///
/// 返り値 `c`（長さ L）は `S[i] = Σ_{j=0}^{L-1} c[j]·S[i-1-j]`（`i ≥ L`）を満たす。
/// 数列は少なくとも `2L` 項あると安全。第 N 項を求めるには [`線形漸化式の第n項`] に渡す。
fn ベルレカンプマッシー(数列: &[u64], 法: u64) -> Vec<u64> {
    let p = 法 as i128;
    let n = 数列.len();
    let s: Vec<i128> = 数列.iter().map(|&x| (x as i128) % p).collect();
    let mut c = vec![0i128; n + 1];
    let mut b = vec![0i128; n + 1];
    c[0] = 1 % p;
    b[0] = 1 % p;
    let mut l = 0usize;
    let mut m = 0usize;
    let mut bb = 1i128 % p;
    for i in 0..n {
        m += 1;
        let mut d = s[i] % p;
        for j in 1..=l {
            d = (d + c[j] * s[i - j]) % p;
        }
        if d == 0 {
            continue;
        }
        let t = c.clone();
        let coef = d * 累乗_mod((bb % p) as u64, 法 - 2, 法) as i128 % p;
        for j in m..=n {
            c[j] = ((c[j] - coef * b[j - m]) % p + p) % p;
        }
        if 2 * l > i {
            continue;
        }
        l = i + 1 - l;
        b = t;
        bb = d;
        m = 0;
    }
    c.truncate(l + 1);
    c.drain(0..1);
    c.iter().map(|&x| (((p - x) % p + p) % p) as u64).collect()
}

#[snippet("線形_漸化式")]
#[snippet("ベルレカンプ_マッシー")]
#[snippet("Kitamasa")]
#[snippet("きたまさ_法")]
#[snippet("数列の_第_n_項")]
/// 多項式の積を `x^k = Σ 係数[j]·x^{k-1-j}` で次数 `k` 未満に縮約する内部関数。
fn 多項式乗算縮約(a: &[u64], b: &[u64], 係数: &[u64], 法: u64) -> Vec<u64> {
    let k = 係数.len();
    let m = 法 as u128;
    let mut res = vec![0u128; (a.len() + b.len()).saturating_sub(1).max(k)];
    for (i, &ai) in a.iter().enumerate() {
        if ai == 0 {
            continue;
        }
        for (j, &bj) in b.iter().enumerate() {
            res[i + j] = (res[i + j] + ai as u128 * bj as u128) % m;
        }
    }
    for d in (k..res.len()).rev() {
        let v = res[d];
        if v != 0 {
            res[d] = 0;
            for (j, &係) in 係数.iter().enumerate() {
                let deg = d - 1 - j;
                res[deg] = (res[deg] + v * 係 as u128) % m;
            }
        }
    }
    res.truncate(k);
    res.resize(k, 0);
    res.iter().map(|&x| x as u64).collect()
}

#[snippet("線形_漸化式")]
#[snippet("ベルレカンプ_マッシー")]
#[snippet("Kitamasa")]
#[snippet("きたまさ_法")]
#[snippet("数列の_第_n_項")]
/// きたまさ法。線形漸化式 `S[i] = Σ_{j} 係数[j]·S[i-1-j]` と初項 `初項`（長さ ≥ 次数）から、
/// 第 `n` 項を素数 `法` で O(k^2 log n) で求める（k は次数）。
fn 線形漸化式の第n項(初項: &[u64], 係数: &[u64], n: usize, 法: u64) -> u64 {
    let k = 係数.len();
    if k == 0 {
        return 0;
    }
    if n < k {
        return 初項[n] % 法;
    }
    // x^n mod f を二分累乗で求める。多項式は長さ k の係数ベクトル。
    let mut 結果 = vec![0u64; k];
    結果[0] = 1 % 法; // 多項式 1
    let mut 底 = if k >= 2 {
        let mut v = vec![0u64; k];
        v[1] = 1 % 法; // 多項式 x
        v
    } else {
        vec![係数[0] % 法] // k==1: x ≡ 係数[0]
    };
    let mut e = n;
    while e > 0 {
        if e & 1 == 1 {
            結果 = 多項式乗算縮約(&結果, &底, 係数, 法);
        }
        底 = 多項式乗算縮約(&底, &底, 係数, 法);
        e >>= 1;
    }
    let m = 法 as u128;
    let mut ans = 0u128;
    for (i, &係) in 結果.iter().enumerate() {
        ans = (ans + 係 as u128 * (初項[i] % 法) as u128) % m;
    }
    ans as u64
}

#[snippet("線形_漸化式")]
#[snippet("ベルレカンプ_マッシー")]
#[snippet("Kitamasa")]
#[snippet("きたまさ_法")]
#[snippet("数列の_第_n_項")]
/// 数列の先頭を渡すと、Berlekamp-Massey で漸化式を推定し、きたまさ法で第 `n` 項を返す。
/// 数列は最短漸化式の次数の 2 倍以上の項を含むこと（足りないと誤った外挿になる）。
fn 数列の第n項(数列: &[u64], n: usize, 法: u64) -> u64 {
    let 係数 = ベルレカンプマッシー(数列, 法);
    線形漸化式の第n項(数列, &係数, n, 法)
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: u64 = 998_244_353;

    fn フィボナッチ列(n: usize) -> Vec<u64> {
        let mut f = vec![0u64; n];
        if n > 1 {
            f[1] = 1;
        }
        for i in 2..n {
            f[i] = (f[i - 1] + f[i - 2]) % P;
        }
        f
    }

    #[test]
    fn ベルレカンプマッシーでフィボナッチを復元() {
        let f = フィボナッチ列(12);
        assert_eq!(ベルレカンプマッシー(&f, P), vec![1, 1]);
    }

    #[test]
    fn フィボナッチ第n項() {
        let f = フィボナッチ列(50);
        for n in 0..50usize {
            assert_eq!(数列の第n項(&f, n, P), f[n], "n={n}");
        }
        // 大きい n も直接計算と一致
        let mut a = 0u64;
        let mut b = 1u64;
        for _ in 0..1000 {
            let c = (a + b) % P;
            a = b;
            b = c;
        }
        assert_eq!(数列の第n項(&f, 1000, P), a);
    }

    #[test]
    fn 三項間漸化式() {
        // S[i] = 2 S[i-1] + 3 S[i-2] - S[i-3], 初項 1,2,3
        let 係数 = vec![2u64, 3, P - 1];
        let mut s = vec![1u64, 2, 3];
        for i in 3..40 {
            let v = (2 * s[i - 1] % P + 3 * s[i - 2] % P + (P - 1) * s[i - 3] % P) % P;
            s.push(v);
        }
        for n in 0..40usize {
            assert_eq!(線形漸化式の第n項(&s, &係数, n, P), s[n], "n={n}");
        }
        // BM 経由でも一致
        assert_eq!(数列の第n項(&s, 39, P), s[39]);
    }

    #[test]
    fn 等比数列() {
        // 3^n: S[i] = 3 S[i-1]
        let s: Vec<u64> = (0..20).map(|i| 累乗_mod(3, i, P)).collect();
        assert_eq!(数列の第n項(&s, 100, P), 累乗_mod(3, 100, P));
    }
}
