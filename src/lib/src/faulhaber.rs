use crate::mod_pow::{累乗_mod, 逆元};
use cargo_snippet::snippet;

#[snippet("累乗_和", include = "mod_逆元")]
#[snippet("ベルヌーイ_数")]
#[snippet("Faulhaber")]
#[snippet("Bernoulli")]
#[snippet("べき乗_和")]
/// `0..=上限` の階乗・逆階乗を素数 `法` 上で返す内部関数（二項係数用）。
fn 階乗表(上限: usize, 法: u64) -> (Vec<u64>, Vec<u64>) {
    let mut 階乗 = vec![1u64; 上限 + 1];
    for i in 1..=上限 {
        階乗[i] = 階乗[i - 1] * i as u64 % 法;
    }
    let mut 逆階乗 = vec![1u64; 上限 + 1];
    逆階乗[上限] = 逆元(階乗[上限], 法);
    for i in (0..上限).rev() {
        逆階乗[i] = 逆階乗[i + 1] * (i as u64 + 1) % 法;
    }
    (階乗, 逆階乗)
}

#[snippet("累乗_和")]
#[snippet("ベルヌーイ_数")]
#[snippet("Faulhaber")]
#[snippet("Bernoulli")]
#[snippet("べき乗_和")]
/// ベルヌーイ数 `B_0, B_1, …, B_k` を素数 `法` 上で返す（`B_1 = -1/2` の規約）。`O(k^2)`。
///
/// 累乗和（Faulhaber 公式）の係数として使う。
fn ベルヌーイ数(k: usize, 法: impl TryInto<u64>) -> Vec<usize> {
    let 法: u64 = 法
        .try_into()
        .ok()
        .expect("ベルヌーイ数: 引数が u64 に収まりません");
    let (階乗, 逆階乗) = 階乗表(k + 1, 法);
    let 二項 = |n: usize, r: usize| -> u64 {
        if r > n {
            0
        } else {
            階乗[n] * 逆階乗[r] % 法 * 逆階乗[n - r] % 法
        }
    };
    let mut b = vec![0u64; k + 1];
    b[0] = 1 % 法;
    for m in 1..=k {
        // (m+1)·B_m = -Σ_{j=0}^{m-1} C(m+1, j)·B_j
        let mut s = 0u64;
        for (j, &bj) in b.iter().enumerate().take(m) {
            s = (s + 二項(m + 1, j) * bj) % 法;
        }
        let 逆 = 逆元((m as u64 + 1) % 法, 法);
        b[m] = (法 - s) % 法 * 逆 % 法;
    }
    b.into_iter().map(|x| x as usize).collect()
}

#[snippet("累乗_和")]
#[snippet("Faulhaber")]
#[snippet("べき乗_和")]
/// 累乗和 `Σ_{i=1}^{n} i^k mod 法`（`法` は素数）を Faulhaber の公式で `O(k^2)` で返す。`n` は最大 `u64`。
///
/// 1 個の巨大な `n` と固定の `k` に対し、`n` をいくら大きくしても次数 `k` だけに依存して計算できる。
/// ラグランジュ補間（[`lagrange_interpolation`](crate::lagrange_interpolation)）とは別手法。
fn 累乗和(n: impl TryInto<u64>, k: usize, 法: impl TryInto<u64>) -> usize {
    let n: u64 = n
        .try_into()
        .ok()
        .expect("累乗和: 引数が u64 に収まりません");
    let 法: u64 = 法
        .try_into()
        .ok()
        .expect("累乗和: 引数が u64 に収まりません");
    if k == 0 {
        return (n % 法) as usize; // Σ_{i=1}^{n} 1 = n
    }
    let b = ベルヌーイ数(k, 法);
    let (階乗, 逆階乗) = 階乗表(k + 1, 法);
    let 二項 = |n: usize, r: usize| -> u64 {
        if r > n {
            0
        } else {
            階乗[n] * 逆階乗[r] % 法 * 逆階乗[n - r] % 法
        }
    };
    let m = (n % 法 + 1) % 法; // m = n+1（法で簡約）
    let mut s = 0u64;
    for (j, &bj) in b.iter().enumerate() {
        // C(k+1, j)·B_j·m^{k+1-j}
        let 項 = 二項(k + 1, j) * bj as u64 % 法 * 累乗_mod(m, (k + 1 - j) as u64, 法) % 法;
        s = (s + 項) % 法;
    }
    (s * 逆元((k as u64 + 1) % 法, 法) % 法) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 素朴累乗和(n: u64, k: usize, 法: u64) -> u64 {
        let mut s = 0u128;
        for i in 1..=n {
            s += 累乗_mod(i, k as u64, 法) as u128;
        }
        (s % 法 as u128) as u64
    }

    #[test]
    fn ベルヌーイ数の既知値() {
        // B_1 = -1/2, B_2 = 1/6, B_3 = 0, B_4 = -1/30。
        let 法 = 998244353u64;
        let b = ベルヌーイ数(6, 法);
        assert_eq!(b[0], 1);
        assert_eq!(b[1], (法 - 逆元(2, 法)) as usize); // -1/2
        assert_eq!(b[2], 逆元(6, 法) as usize); // 1/6
        assert_eq!(b[3], 0);
        assert_eq!(b[4], (法 - 逆元(30, 法)) as usize); // -1/30
        assert_eq!(b[5], 0);
    }

    #[test]
    fn 累乗和が素朴と一致() {
        for &法 in &[998244353u64, 1_000_000_007, 97] {
            for k in 0..=8usize {
                for n in 0..=40u64 {
                    assert_eq!(
                        累乗和(n, k, 法),
                        素朴累乗和(n, k, 法) as usize,
                        "n={n} k={k} 法={法}"
                    );
                }
            }
        }
    }

    #[test]
    fn 巨大なnでも公式が成立() {
        // n が法を超える領域でも T(n+1) の簡約が正しいことを、別の小法で総当たりと突き合わせる。
        let 法 = 101u64;
        for k in 1..=6usize {
            for n in 100..=250u64 {
                assert_eq!(
                    累乗和(n, k, 法),
                    素朴累乗和(n, k, 法) as usize,
                    "n={n} k={k}"
                );
            }
        }
    }
}
