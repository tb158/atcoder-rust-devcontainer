// 組合せModInt（階乗・二項・多項）— StaticModInt / DynamicModInt 共通
//
// 階乗前計算が不要な数え上げ表（分割数・スターリング数・ベル数・モンモール数）は
// counting_tables.rs（Comb 非依存）にある。

use crate::to_int::to_isize;
use cargo_snippet::snippet;

// ModIntBase は ac_library 直下に再エクスポートされていないので modint:: 経由で使う
#[snippet("組合せ_ModInt_comb_factorial")]
use ac_library::modint::ModIntBase;

/// `M` は `ModInt998244353` / `ModInt1000000007` / `ModInt`（DynamicModInt。先に
/// `Mint::set_modulus(m)`）のいずれか。
#[snippet("組合せ_ModInt_comb_factorial")]
struct Comb<M> {
    max: usize,
    fac: Vec<M>,
    fac_inv: Vec<M>,
}

#[snippet("組合せ_ModInt_comb_factorial", include = "to_isize")]
impl<M: ModIntBase> Comb<M> {
    fn new(max_n: usize) -> Self {
        let mut fac = vec![M::new(1); max_n + 1];
        for i in 1..=max_n {
            fac[i] = fac[i - 1] * M::new(i);
        }
        let mut fac_inv = vec![M::new(1); max_n + 1];
        fac_inv[max_n] = fac[max_n].inv();
        for i in (0..max_n).rev() {
            fac_inv[i] = fac_inv[i + 1] * M::new(i + 1);
        }
        Self {
            max: max_n,
            fac,
            fac_inv,
        }
    }

    fn 階乗(&self, n: impl TryInto<isize>) -> M {
        let n = to_isize(n);
        if n < 0 {
            return M::raw(0);
        }
        let n = n as usize;
        assert!(
            n <= self.max,
            "階乗: n={n} exceeds precomputed max={}",
            self.max
        );
        self.fac[n]
    }

    fn 二項係数(&self, n: impl TryInto<isize>, k: impl TryInto<isize>) -> M {
        let (n, k) = (to_isize(n), to_isize(k));
        if k < 0 || n < k {
            return M::raw(0);
        }
        if k == 0 {
            return M::new(1);
        }
        if n <= self.max as isize {
            let n = n as usize;
            let k = k as usize;
            return self.fac[n] * self.fac_inv[k] * self.fac_inv[n - k];
        }
        // n が max を超える場合は分子を直接掛ける（fac_inv[k] を使うので k <= max が必要）
        assert!(
            k <= self.max as isize,
            "二項係数: k={k} exceeds precomputed max={}",
            self.max
        );
        let mut res = M::new(1);
        for i in 0..k {
            res *= M::new(n - i);
        }
        res * self.fac_inv[k as usize]
    }

    fn 多項係数<K: Copy + TryInto<isize>>(&self, n: impl TryInto<isize>, ks: &[K]) -> M {
        let n = to_isize(n);
        let ks: Vec<isize> = ks.iter().map(|&k| to_isize(k)).collect();
        if ks.iter().any(|&k| k < 0) {
            return M::raw(0);
        }
        let sum: isize = ks.iter().sum();
        if sum != n || n < 0 {
            return M::raw(0);
        }
        let n = n as usize;
        assert!(
            n <= self.max,
            "多項係数: n={n} exceeds precomputed max={}",
            self.max
        );
        let mut res = self.fac[n];
        for &k in &ks {
            res *= self.fac_inv[k as usize];
        }
        res
    }

    /// 重複組合せ `nHk = C(n + k - 1, k)`（n 種類から k 個選ぶ、同じ種類の重複可）。
    fn 重複組合せ(&self, n: impl TryInto<isize>, k: impl TryInto<isize>) -> M {
        let (n, k) = (to_isize(n), to_isize(k));
        if n < 0 || k < 0 {
            return M::raw(0);
        }
        if k == 0 {
            return M::new(1);
        }
        self.二項係数(n + k - 1, k)
    }

    /// カタラン数 `C(n) = 二項係数(2n, n) − 二項係数(2n, n+1)`（= 正しく対応した括弧列・二分木・
    /// 三角形分割・単調格子路の個数）を返す。`n < 0` は 0。前計算済みの階乗・逆階乗を使うので
    /// `2n ≤ max_n` が必要。
    fn カタラン数(&self, n: impl TryInto<isize>) -> M {
        let n = to_isize(n);
        if n < 0 {
            return M::raw(0);
        }
        self.二項係数(2 * n, n) - self.二項係数(2 * n, n + 1)
    }
}

// DynamicModInt の mod はグローバルなので、動的 ModInt を使うテストはこのロックで直列化する
// （counting_tables.rs のテストと共有）。
#[cfg(test)]
pub(crate) static MODINT_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
mod tests {
    use super::*;

    type Mint = ac_library::ModInt998244353;

    #[test]
    fn comb_cases() {
        let c = Comb::<Mint>::new(20);
        assert_eq!(c.二項係数(-1, 0).val(), 0);
        assert_eq!(c.二項係数(5, -1).val(), 0);
        assert_eq!(c.二項係数(3, 5).val(), 0);
        assert_eq!(c.二項係数(5, 0).val(), 1);
        assert_eq!(c.二項係数(5, 2).val(), 10);
        assert_eq!(c.二項係数(10, 3).val(), 120);
        // n が max を超える経路（分子を直接掛ける分岐）
        assert_eq!(c.二項係数(100, 2).val(), 4950);
        assert_eq!(
            c.多項係数(5, &[2, 1, 2]),
            c.二項係数(5, 2) * c.二項係数(3, 1)
        );
        assert_eq!(c.多項係数(5, &[2, 3]).val(), c.二項係数(5, 2).val());
        assert_eq!(c.多項係数(5, &[2, 2]).val(), 0);

        assert_eq!(c.重複組合せ(3, 2).val(), c.二項係数(4, 2).val());
        assert_eq!(c.重複組合せ(3, 2).val(), 6);
        assert_eq!(c.重複組合せ(5, 0).val(), 1);
        assert_eq!(c.重複組合せ(0, 0).val(), 1);
        assert_eq!(c.重複組合せ(0, 3).val(), 0);
        assert_eq!(c.重複組合せ(-1, 2).val(), 0);

        // カタラン数 1,1,2,5,14,42,132,...
        let 期待 = [1u64, 1, 2, 5, 14, 42, 132, 429, 1430];
        for (n, &e) in 期待.iter().enumerate() {
            assert_eq!(c.カタラン数(n as isize).val() as u64, e, "Catalan({n})");
        }
        assert_eq!(c.カタラン数(-1).val(), 0);

        // 任意の整数型を混在して渡せる（usize / u32 / isize …）
        assert_eq!(c.二項係数(10usize, 3u32).val(), 120);
        assert_eq!(c.階乗(5usize).val(), c.階乗(5isize).val());
        assert_eq!(
            c.多項係数(5usize, &[2u32, 3u32]).val(),
            c.二項係数(5, 2).val()
        );
        assert_eq!(c.重複組合せ(3usize, 2u64).val(), 6);
        assert_eq!(c.カタラン数(4usize).val(), 14);

        // 型エイリアスを差し替えれば別の法でも同じコードが動く
        let c7 = Comb::<ac_library::ModInt1000000007>::new(20);
        assert_eq!(c7.二項係数(10, 3).val(), 120);
    }

    #[test]
    fn 動的modでも動く() {
        let _guard = MODINT_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        type Dyn = ac_library::ModInt;
        Dyn::set_modulus(7);
        let c = Comb::<Dyn>::new(5);
        assert_eq!(c.二項係数(5, 2).val(), 3);
        Dyn::set_modulus(998_244_353);
    }
}
