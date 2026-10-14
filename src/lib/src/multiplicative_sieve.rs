use cargo_snippet::snippet;

#[snippet("乗法的_関数_線形_篩")]
#[snippet("乗法的_関数_表")]
#[snippet("MultiplicativeSieve")]
/// 任意の**乗法的関数** `f`（`gcd(a,b)=1 ⇒ f(ab)=f(a)f(b)`）の `f(1..=n)` を線形篩 `O(N)` で作る。
///
/// 利用者は単位元 `単位 = f(1)`、値の乗算 `乗算`、素数冪での値 `素数冪値(p, c) = f(pᶜ)` を渡す。
/// [`メビウス関数表`](crate::mobius::メビウス関数表)・[`オイラー関数`](crate::totient) は μ・φ 専用、
/// [`divisor_sieve`](crate::divisor_sieve) は約数個数・約数和の `O(N log N)` 調和篩。本関数は
/// 約数個数・約数和・μ・φ・`p`進付値・積性なら何でも同じ `O(N)` 篩で作れる汎用版。
///
/// 戻り値は長さ `n+1`。`f[0]` は未定義（`単位` を置く）、`f[1] = 単位`。
fn 乗法的関数表<T, M, P>(n: usize, 単位: T, 乗算: M, 素数冪値: P) -> Vec<T>
where
    T: Clone,
    M: Fn(&T, &T) -> T,
    P: Fn(u64, u32) -> T,
{
    let mut f = vec![単位.clone(); n + 1];
    if n == 0 {
        return f;
    }
    let mut 合成 = vec![false; n + 1];
    let mut 最小素数冪 = vec![0usize; n + 1]; // i の最小素数のべき部分 pᶜ
    let mut 指数 = vec![0u32; n + 1]; // i の最小素数の指数 c
    let mut 素数: Vec<usize> = Vec::new();
    // f[1] = 単位 は初期化済み。
    for i in 2..=n {
        if !合成[i] {
            素数.push(i);
            最小素数冪[i] = i;
            指数[i] = 1;
            f[i] = 素数冪値(i as u64, 1);
        }
        for &p in &素数 {
            let ip = i * p;
            if ip > n {
                break;
            }
            合成[ip] = true;
            if i % p == 0 {
                // p は i の最小素数。べきを 1 つ伸ばす。
                最小素数冪[ip] = 最小素数冪[i] * p;
                指数[ip] = 指数[i] + 1;
                let 残り = i / 最小素数冪[i]; // ip / 最小素数冪[ip]、p と互いに素
                f[ip] = 乗算(&f[残り], &素数冪値(p as u64, 指数[ip]));
                break; // 線形性のためここで打ち切る
            } else {
                // p と i は互いに素。
                最小素数冪[ip] = p;
                指数[ip] = 1;
                f[ip] = 乗算(&f[i], &f[p]);
            }
        }
    }
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    // n を素因数分解し、素数冪値を掛け合わせる独立オラクル。
    fn 素朴<P: Fn(u64, u32) -> i64>(n: u64, 単位: i64, 素数冪値: &P) -> i64 {
        let mut 値 = 単位;
        let mut m = n;
        let mut p = 2u64;
        while p * p <= m {
            if m % p == 0 {
                let mut c = 0u32;
                while m % p == 0 {
                    m /= p;
                    c += 1;
                }
                値 *= 素数冪値(p, c);
            }
            p += 1;
        }
        if m > 1 {
            値 *= 素数冪値(m, 1);
        }
        値
    }

    #[test]
    fn 約数個数_オイラー_メビウスが素朴と一致() {
        let n = 3000usize;
        // 約数個数 d(pᶜ) = c + 1。
        let 約数規則 = |_p: u64, c: u32| (c + 1) as i64;
        // オイラー φ(pᶜ) = pᶜ − pᶜ⁻¹。
        let 互約規則 = |p: u64, c: u32| (p.pow(c) - p.pow(c - 1)) as i64;
        // メビウス μ(p)=−1, μ(pᶜ≥2)=0。
        let 符号規則 = |_p: u64, c: u32| if c == 1 { -1 } else { 0 };

        let 約数表 = 乗法的関数表(n, 1i64, |a, b| a * b, 約数規則);
        let 互約表 = 乗法的関数表(n, 1i64, |a, b| a * b, 互約規則);
        let 符号表 = 乗法的関数表(n, 1i64, |a, b| a * b, 符号規則);
        assert_eq!(約数表[1], 1);
        assert_eq!(互約表[1], 1);
        assert_eq!(符号表[1], 1);
        for k in 2..=n {
            assert_eq!(約数表[k], 素朴(k as u64, 1, &約数規則), "d({k})");
            assert_eq!(互約表[k], 素朴(k as u64, 1, &互約規則), "φ({k})");
            assert_eq!(符号表[k], 素朴(k as u64, 1, &符号規則), "μ({k})");
        }
    }

    #[test]
    fn 約数和も一致() {
        let n = 2000usize;
        // σ(pᶜ) = (p^(c+1) − 1)/(p − 1)。
        let 約数和規則 = |p: u64, c: u32| ((p.pow(c + 1) - 1) / (p - 1)) as i64;
        let 約数和表 = 乗法的関数表(n, 1i64, |a, b| a * b, 約数和規則);
        for (k, &fk) in 約数和表.iter().enumerate().skip(1) {
            // 約数和を直接合計した値と比較。
            let 直接: i64 = (1..=k as i64).filter(|d| k as i64 % d == 0).sum();
            assert_eq!(fk, 直接, "σ({k})");
        }
    }

    #[test]
    fn ゼロと一() {
        let f = 乗法的関数表(0, 1i64, |a, b| a * b, |_p, _c| 0);
        assert_eq!(f, vec![1]); // f[0] のみ
        let f = 乗法的関数表(1, 1i64, |a, b| a * b, |_p, _c| 0);
        assert_eq!(f, vec![1, 1]); // f[0], f[1]=単位
    }
}
