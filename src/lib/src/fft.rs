use cargo_snippet::snippet;
#[snippet("FFT")]
#[snippet("多項式_乗算_FFT")]
#[snippet("畳み込み_FFT")]
#[snippet("複素数_畳み込み")]
#[snippet("高速_フーリエ_変換")]
use num::complex::Complex64;

#[snippet("FFT")]
#[snippet("高速_フーリエ_変換")]
#[snippet("畳み込み_FFT")]
#[snippet("多項式_乗算_FFT")]
#[snippet("複素数_畳み込み")]
/// 複素 FFT（反復版）。`逆 = true` で逆変換。`a.len()` は 2 冪であること。内部関数。
fn fft内部(a: &mut [Complex64], 逆: bool) {
    let n = a.len();
    if n <= 1 {
        return;
    }
    // ビット反転並べ替え
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            a.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let 符号 = if 逆 { -1.0 } else { 1.0 };
        let 角度 = 2.0 * std::f64::consts::PI / len as f64 * 符号;
        let w_len = Complex64::new(角度.cos(), 角度.sin());
        let mut i = 0;
        while i < n {
            let mut w = Complex64::new(1.0, 0.0);
            for k in 0..len / 2 {
                let u = a[i + k];
                let v = a[i + k + len / 2] * w;
                a[i + k] = u + v;
                a[i + k + len / 2] = u - v;
                w *= w_len;
            }
            i += len;
        }
        len <<= 1;
    }
    if 逆 {
        let 逆数 = 1.0 / n as f64;
        for x in a.iter_mut() {
            *x *= 逆数;
        }
    }
}

#[snippet("FFT")]
#[snippet("高速_フーリエ_変換")]
#[snippet("畳み込み_FFT")]
#[snippet("多項式_乗算_FFT")]
#[snippet("複素数_畳み込み")]
/// 整数列 `a`, `b` の畳み込み（多項式の積の係数）を FFT で求める。O(M log M)。
///
/// 返り値の長さは `a.len() + b.len() - 1`。`ac_library::convolution` は素数 mod 専用なので、
/// mod を取らない素直な整数畳み込みが要るときにこちらを使う。値が大きいと `f64` 精度で
/// 誤差が出る点に注意（おおむね 結果の各項 < 2^52 まで安全）。
fn 畳み込み(a: &[i64], b: &[i64]) -> Vec<i64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let 結果長 = a.len() + b.len() - 1;
    let mut サイズ = 1usize;
    while サイズ < 結果長 {
        サイズ <<= 1;
    }
    let mut fa: Vec<Complex64> = (0..サイズ)
        .map(|i| Complex64::new(*a.get(i).unwrap_or(&0) as f64, 0.0))
        .collect();
    let mut fb: Vec<Complex64> = (0..サイズ)
        .map(|i| Complex64::new(*b.get(i).unwrap_or(&0) as f64, 0.0))
        .collect();
    fft内部(&mut fa, false);
    fft内部(&mut fb, false);
    for (x, y) in fa.iter_mut().zip(fb.iter()) {
        *x *= *y;
    }
    fft内部(&mut fa, true);
    fa.iter()
        .take(結果長)
        .map(|c| c.re.round() as i64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 素朴畳み込み(a: &[i64], b: &[i64]) -> Vec<i64> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        let mut res = vec![0i64; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                res[i + j] += x * y;
            }
        }
        res
    }

    #[test]
    fn 小さい例() {
        // (1 + 2x)(3 + 4x) = 3 + 10x + 8x^2
        assert_eq!(畳み込み(&[1, 2], &[3, 4]), vec![3, 10, 8]);
    }

    #[test]
    fn 素朴と一致() {
        let a = vec![1i64, 2, 3, 4, 5, 6, 7];
        let b = vec![9i64, 8, 7, 6, 5];
        assert_eq!(畳み込み(&a, &b), 素朴畳み込み(&a, &b));
    }

    #[test]
    fn 擬似乱数で一致() {
        // 線形合同法で適当な列を作って検証
        let mut seed = 12345u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 33) as i64 % 1000
        };
        for _ in 0..20 {
            let n = (乱数().unsigned_abs() % 30 + 1) as usize;
            let m = (乱数().unsigned_abs() % 30 + 1) as usize;
            let a: Vec<i64> = (0..n).map(|_| 乱数()).collect();
            let b: Vec<i64> = (0..m).map(|_| 乱数()).collect();
            assert_eq!(畳み込み(&a, &b), 素朴畳み込み(&a, &b));
        }
    }

    #[test]
    fn 空入力() {
        assert_eq!(畳み込み(&[], &[1, 2]), Vec::<i64>::new());
    }
}
