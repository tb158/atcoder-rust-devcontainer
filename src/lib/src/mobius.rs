use cargo_snippet::snippet;

#[snippet("メビウス_関数")]
#[snippet("メビウス_関数_表")]
#[snippet("MobiusFunction")]
#[snippet("数論_メビウス")]
/// 数論的メビウス関数 `μ(0..=n)` を線形篩で `O(n)` 求める。
///
/// `μ(1)=1`、`n` が平方因子を持てば `0`、相異なる素因数 `k` 個の積なら `(-1)^k`。`μ(0)=0`。
/// メビウス反転・互いに素なペア数え・約数包除で使う。
/// 集合の包除である [`zeta_mobius`](crate::zeta_mobius) とは別物（あちらは部分集合の和）。
fn メビウス関数表(n: usize) -> Vec<isize> {
    let mut μ = vec![0isize; n + 1];
    if n >= 1 {
        μ[1] = 1;
    }
    let mut 素数 = Vec::new();
    let mut 合成 = vec![false; n + 1];
    for i in 2..=n {
        if !合成[i] {
            素数.push(i);
            μ[i] = -1;
        }
        for &p in &素数 {
            if i * p > n {
                break;
            }
            合成[i * p] = true;
            if i % p == 0 {
                μ[i * p] = 0; // p^2 が i*p を割る → 平方因子あり
                break;
            }
            μ[i * p] = -μ[i];
        }
    }
    μ
}

#[snippet("メビウス_関数")]
#[snippet("MobiusFunction")]
#[snippet("数論_メビウス")]
/// 単一の `n` についてメビウス関数 `μ(n)` を試し割りで `O(√n)` 求める。
///
/// 表が要らない・`n` が大きい単発計算向け。`μ(0)=0`、`μ(1)=1`。
fn メビウス関数(n: impl TryInto<u64>) -> isize {
    let mut n: u64 = n
        .try_into()
        .ok()
        .expect("メビウス関数: 引数が u64 に収まりません");
    if n == 0 {
        return 0;
    }
    let mut 符号 = 1isize;
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            n /= d;
            if n % d == 0 {
                return 0; // 平方因子
            }
            符号 = -符号;
        }
        d += 1;
    }
    if n > 1 {
        符号 = -符号; // 残った大きな素因数 1 個
    }
    符号
}

#[cfg(test)]
// n は索引と μ(n) の引数の両方に使うため範囲ループが自然。
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;

    #[test]
    fn 既知値() {
        let μ = メビウス関数表(30);
        assert_eq!(μ[1], 1);
        assert_eq!(μ[2], -1);
        assert_eq!(μ[4], 0); // 2^2
        assert_eq!(μ[6], 1); // 2·3
        assert_eq!(μ[12], 0); // 2^2·3
        assert_eq!(μ[30], -1); // 2·3·5
        assert_eq!(μ[0], 0);
    }

    #[test]
    fn 表と単一値が一致() {
        const N: usize = 5000;
        let μ = メビウス関数表(N);
        for n in 0..=N {
            assert_eq!(μ[n], メビウス関数(n as u64), "μ({n})");
        }
    }

    #[test]
    fn メビウス反転の確認() {
        // Σ_{d|n} μ(d) = [n == 1]。
        const N: usize = 2000;
        let μ = メビウス関数表(N);
        for n in 1..=N {
            let mut 和 = 0isize;
            let mut d = 1;
            while d <= n {
                if n % d == 0 {
                    和 += μ[d];
                }
                d += 1;
            }
            assert_eq!(和, isize::from(n == 1), "Σ_{{d|{n}}} μ(d)");
        }
    }
}
