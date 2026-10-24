use cargo_snippet::snippet;

#[snippet("gcd_畳み込み")]
#[snippet("lcm_畳み込み")]
#[snippet("約数_ゼータ_変換")]
#[snippet("倍数_ゼータ_変換")]
#[snippet("GcdConvolution")]
#[snippet("LcmConvolution")]
/// `2..n` の素数一覧を篩で求める内部関数（各ゼータ／メビウス変換で共有）。
fn 素数リスト(n: usize) -> Vec<usize> {
    let mut 合成 = vec![false; n];
    let mut 素数 = Vec::new();
    for p in 2..n {
        if !合成[p] {
            素数.push(p);
            let mut q = p * p;
            while q < n {
                合成[q] = true;
                q += p;
            }
        }
    }
    素数
}

#[snippet("倍数_ゼータ_変換")]
#[snippet("gcd_畳み込み")]
#[snippet("GcdConvolution")]
/// 倍数方向のゼータ変換。`f[i]` を `Σ_{i | j, j < n} f[j]`（倍数についての総和）へ in-place 変換する。
/// 添字は `1..n`（`f[0]` は使わない）。逆変換は [`倍数メビウス変換`]。`O(n log log n)`。
/// 要素型は加算できれば何でもよい（isize / usize / f64 / ModInt など）。
fn 倍数ゼータ変換<T: Copy + std::ops::AddAssign>(f: &mut [T]) {
    let n = f.len();
    for &p in &素数リスト(n) {
        let mut i = (n - 1) / p;
        while i >= 1 {
            f[i] += f[i * p];
            i -= 1;
        }
    }
}

#[snippet("倍数_ゼータ_変換")]
#[snippet("gcd_畳み込み")]
#[snippet("GcdConvolution")]
/// [`倍数ゼータ変換`] の逆。`Σ_{i | j} f[j]` から元の `f` を復元する。`O(n log log n)`。
fn 倍数メビウス変換<T: Copy + std::ops::SubAssign>(f: &mut [T]) {
    let n = f.len();
    for &p in &素数リスト(n) {
        let mut i = 1;
        while i * p < n {
            f[i] -= f[i * p];
            i += 1;
        }
    }
}

#[snippet("約数_ゼータ_変換")]
#[snippet("lcm_畳み込み")]
#[snippet("LcmConvolution")]
/// 約数方向のゼータ変換。`f[i]` を `Σ_{j | i} f[j]`（約数についての総和）へ in-place 変換する。
/// 添字は `1..n`。逆変換は [`約数メビウス変換`]。`O(n log log n)`。
/// 要素型は加算できれば何でもよい（isize / usize / f64 / ModInt など）。
fn 約数ゼータ変換<T: Copy + std::ops::AddAssign>(f: &mut [T]) {
    let n = f.len();
    for &p in &素数リスト(n) {
        let mut i = 1;
        while i * p < n {
            f[i * p] += f[i];
            i += 1;
        }
    }
}

#[snippet("約数_ゼータ_変換")]
#[snippet("lcm_畳み込み")]
#[snippet("LcmConvolution")]
/// [`約数ゼータ変換`] の逆。`Σ_{j | i} f[j]` から元の `f` を復元する。`O(n log log n)`。
fn 約数メビウス変換<T: Copy + std::ops::SubAssign>(f: &mut [T]) {
    let n = f.len();
    for &p in &素数リスト(n) {
        let mut i = (n - 1) / p;
        while i >= 1 {
            f[i * p] -= f[i];
            i -= 1;
        }
    }
}

#[snippet("gcd_畳み込み")]
#[snippet("GcdConvolution")]
/// GCD 畳み込み。`h[k] = Σ_{gcd(i, j) = k} f[i]·g[j]` を `O(n log log n)` で返す（添字 `1..n`、`h[0] = 0`）。
/// 「gcd がちょうど `k` になる組の数え上げ」などに使う。
/// 要素型は加減乗できれば何でもよい（isize / ModInt など。MOD 付き数え上げは ModInt をそのまま渡す）。
fn gcd畳み込み<T>(f: &[T], g: &[T]) -> Vec<T>
where
    T: Copy + Default + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign,
{
    assert_eq!(f.len(), g.len());
    let mut a = f.to_vec();
    let mut b = g.to_vec();
    倍数ゼータ変換(&mut a);
    倍数ゼータ変換(&mut b);
    a.iter_mut().zip(&b).for_each(|(x, y)| *x *= *y);
    倍数メビウス変換(&mut a);
    if !a.is_empty() {
        a[0] = T::default(); // 添字 0 は未使用
    }
    a
}

#[snippet("lcm_畳み込み")]
#[snippet("LcmConvolution")]
/// LCM 畳み込み。`h[k] = Σ_{lcm(i, j) = k} f[i]·g[j]` を `O(n log log n)` で返す（添字 `1..n`、`h[0] = 0`）。
/// 要素型は加減乗できれば何でもよい（isize / ModInt など）。
fn lcm畳み込み<T>(f: &[T], g: &[T]) -> Vec<T>
where
    T: Copy + Default + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign,
{
    assert_eq!(f.len(), g.len());
    let mut a = f.to_vec();
    let mut b = g.to_vec();
    約数ゼータ変換(&mut a);
    約数ゼータ変換(&mut b);
    a.iter_mut().zip(&b).for_each(|(x, y)| *x *= *y);
    約数メビウス変換(&mut a);
    if !a.is_empty() {
        a[0] = T::default(); // 添字 0 は未使用
    }
    a
}

#[cfg(test)]
// Loop indices double as divisibility / gcd parameters (`(i..n).step_by(i)`, `gcd(i, j)`), so range
// loops are the natural form here.
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    #[test]
    fn ゼータとメビウスが逆() {
        let mut seed = 0x1357_9bdfu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..200 {
            let n = (乱数() as usize % 50) + 2;
            let f: Vec<isize> = (0..n).map(|_| (乱数() % 21) as isize - 10).collect();
            // 倍数: ゼータ→メビウスで戻る
            let mut a = f.clone();
            倍数ゼータ変換(&mut a);
            倍数メビウス変換(&mut a);
            assert_eq!(a, f);
            // 約数: ゼータ→メビウスで戻る
            let mut b = f.clone();
            約数ゼータ変換(&mut b);
            約数メビウス変換(&mut b);
            assert_eq!(b, f);
            // 倍数ゼータの定義チェック
            let mut c = f.clone();
            倍数ゼータ変換(&mut c);
            for i in 1..n {
                let 期待: isize = (i..n).step_by(i).map(|j| f[j]).sum();
                assert_eq!(c[i], 期待, "倍数 i={i}");
            }
        }
    }

    #[test]
    fn modintでも一致() {
        use ac_library::ModInt998244353 as M;
        let f = vec![0isize, 3, 1, 4, 1, 5];
        let g = vec![0isize, 9, 2, 6, 5, 3];
        let 期待 = gcd畳み込み(&f, &g);
        let fm: Vec<M> = f.iter().map(|&x| M::new(x)).collect();
        let gm: Vec<M> = g.iter().map(|&x| M::new(x)).collect();
        let 得 = gcd畳み込み(&fm, &gm);
        for (a, b) in 得.iter().zip(&期待) {
            assert_eq!(a.val() as isize, *b);
        }
    }

    #[test]
    fn gcd_lcm畳み込みが総当たりと一致() {
        let mut seed = 0x2468_ace0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..200 {
            let n = (乱数() as usize % 40) + 2;
            let f: Vec<isize> = (0..n).map(|_| (乱数() % 11) as isize).collect();
            let g: Vec<isize> = (0..n).map(|_| (乱数() % 11) as isize).collect();
            let gc = gcd畳み込み(&f, &g);
            let lc = lcm畳み込み(&f, &g);
            // 総当たり
            let mut gc期待 = vec![0isize; n];
            let mut lc期待 = vec![0isize; n];
            for i in 1..n {
                for j in 1..n {
                    let k = gcd(i, j);
                    gc期待[k] += f[i] * g[j];
                    let l = i / gcd(i, j) * j; // lcm
                    if l < n {
                        lc期待[l] += f[i] * g[j];
                    }
                }
            }
            assert_eq!(gc, gc期待, "gcd n={n}");
            assert_eq!(lc, lc期待, "lcm n={n}");
        }
    }
}
