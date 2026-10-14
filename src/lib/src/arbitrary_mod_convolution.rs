#[snippet("ArbitraryModConvolution")]
#[snippet("Garner_畳み込み")]
#[snippet("任意_modNTT")]
#[snippet("任意_mod_畳み込み")]
#[snippet("畳み込み_任意_mod")]
use ac_library::{ButterflyCache, Modulus, StaticModInt, convolution, inv_mod};
use cargo_snippet::snippet;
#[snippet("ArbitraryModConvolution")]
#[snippet("Garner_畳み込み")]
#[snippet("任意_modNTT")]
#[snippet("任意_mod_畳み込み")]
#[snippet("畳み込み_任意_mod")]
use std::cell::RefCell;
#[snippet("ArbitraryModConvolution")]
#[snippet("Garner_畳み込み")]
#[snippet("任意_modNTT")]
#[snippet("任意_mod_畳み込み")]
#[snippet("畳み込み_任意_mod")]
use std::thread::LocalKey;

#[snippet("任意_mod_畳み込み")]
#[snippet("ArbitraryModConvolution")]
#[snippet("任意_modNTT")]
#[snippet("畳み込み_任意_mod")]
#[snippet("Garner_畳み込み")]
/// NTT 素数 1: `167772161 = 5·2^25 + 1`。
#[derive(Copy, Clone, PartialEq, Eq)]
enum 法1 {}
#[snippet("任意_mod_畳み込み")]
#[snippet("ArbitraryModConvolution")]
#[snippet("任意_modNTT")]
#[snippet("畳み込み_任意_mod")]
#[snippet("Garner_畳み込み")]
impl Modulus for 法1 {
    const VALUE: u32 = 167_772_161;
    const HINT_VALUE_IS_PRIME: bool = true;
    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
        thread_local! {
            static C: RefCell<Option<ButterflyCache<法1>>> = RefCell::default();
        }
        &C
    }
}

#[snippet("任意_mod_畳み込み")]
#[snippet("ArbitraryModConvolution")]
#[snippet("任意_modNTT")]
#[snippet("畳み込み_任意_mod")]
#[snippet("Garner_畳み込み")]
/// NTT 素数 2: `469762049 = 7·2^26 + 1`。
#[derive(Copy, Clone, PartialEq, Eq)]
enum 法2 {}
#[snippet("任意_mod_畳み込み")]
#[snippet("ArbitraryModConvolution")]
#[snippet("任意_modNTT")]
#[snippet("畳み込み_任意_mod")]
#[snippet("Garner_畳み込み")]
impl Modulus for 法2 {
    const VALUE: u32 = 469_762_049;
    const HINT_VALUE_IS_PRIME: bool = true;
    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
        thread_local! {
            static C: RefCell<Option<ButterflyCache<法2>>> = RefCell::default();
        }
        &C
    }
}

#[snippet("任意_mod_畳み込み")]
#[snippet("ArbitraryModConvolution")]
#[snippet("任意_modNTT")]
#[snippet("畳み込み_任意_mod")]
#[snippet("Garner_畳み込み")]
/// NTT 素数 3: `1224736769 = 73·2^24 + 1`。
#[derive(Copy, Clone, PartialEq, Eq)]
enum 法3 {}
#[snippet("任意_mod_畳み込み")]
#[snippet("ArbitraryModConvolution")]
#[snippet("任意_modNTT")]
#[snippet("畳み込み_任意_mod")]
#[snippet("Garner_畳み込み")]
impl Modulus for 法3 {
    const VALUE: u32 = 1_224_736_769;
    const HINT_VALUE_IS_PRIME: bool = true;
    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
        thread_local! {
            static C: RefCell<Option<ButterflyCache<法3>>> = RefCell::default();
        }
        &C
    }
}

#[snippet("任意_mod_畳み込み")]
#[snippet("ArbitraryModConvolution")]
#[snippet("任意_modNTT")]
#[snippet("畳み込み_任意_mod")]
#[snippet("Garner_畳み込み")]
/// 1 つの NTT 素数 `M` 上で `a * b`（係数を `法` で還元したもの）を畳み込む。
fn 単一畳み込み<M: Modulus>(a: &[u64], b: &[u64], 法: u64) -> Vec<u64> {
    let fa: Vec<StaticModInt<M>> = a.iter().map(|&x| StaticModInt::new(x % 法)).collect();
    let fb: Vec<StaticModInt<M>> = b.iter().map(|&x| StaticModInt::new(x % 法)).collect();
    convolution(&fa, &fb)
        .iter()
        .map(|x| x.val() as u64)
        .collect()
}

#[snippet("任意_mod_畳み込み")]
#[snippet("ArbitraryModConvolution")]
#[snippet("任意_modNTT")]
#[snippet("畳み込み_任意_mod")]
#[snippet("Garner_畳み込み")]
/// 任意の法 `法` での畳み込み `c[k] = Σ_i a[i]·b[k-i] mod 法`。O(n log n)。
///
/// 3 つの NTT 素数で畳み込み、Garner（CRT）で厳密値を復元してから `法` を取る。
/// `法` と係数が ~2·10^9 未満なら正しい（厳密値が 3 素数の積 ≈ 9.6·10^25 に収まる範囲）。
/// mod が NTT 素数（998244353 等）なら `ac_library::convolution` の方が速い。
fn 任意mod畳み込み(
    a: &[impl TryInto<u64> + Copy],
    b: &[impl TryInto<u64> + Copy],
    法: impl TryInto<u64>,
) -> Vec<usize> {
    let a: Vec<u64> = a
        .iter()
        .map(|&x| {
            x.try_into()
                .ok()
                .expect("任意mod畳み込み: 引数が u64 に収まりません")
        })
        .collect();
    let b: Vec<u64> = b
        .iter()
        .map(|&x| {
            x.try_into()
                .ok()
                .expect("任意mod畳み込み: 引数が u64 に収まりません")
        })
        .collect();
    let (a, b) = (&a[..], &b[..]);
    let 法: u64 = 法
        .try_into()
        .ok()
        .expect("任意mod畳み込み: 引数が u64 に収まりません");
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let p1 = 法1::VALUE as i128;
    let p2 = 法2::VALUE as i128;
    let p3 = 法3::VALUE as i128;
    let r1 = 単一畳み込み::<法1>(a, b, 法);
    let r2 = 単一畳み込み::<法2>(a, b, 法);
    let r3 = 単一畳み込み::<法3>(a, b, 法);
    // Garner 用の逆元（混合基数）
    let inv_p1_mod_p2 = inv_mod(p1 as i64, p2 as i64) as i128;
    let inv_p1p2_mod_p3 = inv_mod((p1 * p2 % p3) as i64, p3 as i64) as i128;
    let 法i = 法 as i128;
    (0..r1.len())
        .map(|i| {
            let a1 = r1[i] as i128;
            let a2 = r2[i] as i128;
            let a3 = r3[i] as i128;
            // x = a1 + p1·k1 + p1·p2·k2
            let k1 = ((a2 - a1) * inv_p1_mod_p2).rem_euclid(p2);
            let t = a1 + p1 * k1;
            let k2 = ((a3 - t) * inv_p1p2_mod_p3).rem_euclid(p3);
            let x = t + p1 * p2 * k2;
            x.rem_euclid(法i) as usize
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 素朴(a: &[u64], b: &[u64], 法: u64) -> Vec<u64> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        let mut c = vec![0u128; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                c[i + j] += (x % 法) as u128 * (y % 法) as u128;
            }
        }
        c.iter().map(|&v| (v % 法 as u128) as u64).collect()
    }

    #[test]
    fn 小さな例() {
        let a = [1u64, 2, 3];
        let b = [4u64, 5, 6];
        // (1+2x+3x^2)(4+5x+6x^2) = 4 +13x +28x^2 +27x^3 +18x^4
        assert_eq!(
            任意mod畳み込み(&a, &b, 1_000_000_007),
            vec![4, 13, 28, 27, 18]
        );
    }

    #[test]
    fn 空入力() {
        assert!(任意mod畳み込み(&[] as &[u64], &[1, 2], 7).is_empty());
        assert!(任意mod畳み込み(&[1, 2], &[] as &[u64], 7).is_empty());
    }

    #[test]
    fn 大きな係数で素朴と一致() {
        let mut seed = 0x1357_2468u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..60 {
            let 法 = [1_000_000_007u64, 998_244_353, 1_000_000_000, 13, 2][乱数() as usize % 5];
            let n = (乱数() as usize % 40) + 1;
            let m = (乱数() as usize % 40) + 1;
            let a: Vec<u64> = (0..n).map(|_| 乱数() % 法).collect();
            let b: Vec<u64> = (0..m).map(|_| 乱数() % 法).collect();
            assert_eq!(
                任意mod畳み込み(&a, &b, 法),
                素朴(&a, &b, 法)
                    .into_iter()
                    .map(|x| x as usize)
                    .collect::<Vec<usize>>(),
                "法={法} n={n} m={m}"
            );
        }
    }
}
