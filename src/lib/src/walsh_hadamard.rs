use cargo_snippet::snippet;

#[snippet("xor_畳み込み")]
#[snippet("アダマール_変換")]
#[snippet("WalshHadamard")]
#[snippet("ウォルシュ_アダマール")]
#[snippet("XorConvolution")]
/// 高速ウォルシュ–アダマール変換（in-place）。長さ `2^k` の配列を `O(n log n)` で変換する。
/// 2 回適用すると各要素が `n` 倍になる（自己逆変換）。XOR 畳み込みの土台。
/// 変換途中で負の値が出るため、符号なし整数は不可（isize / f64 / ModInt などを使う）。
fn アダマール変換<T>(a: &mut [T])
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    let n = a.len();
    let mut 幅 = 1;
    while 幅 < n {
        let mut i = 0;
        while i < n {
            for j in i..i + 幅 {
                let x = a[j];
                let y = a[j + 幅];
                a[j] = x + y; // 偶パリティ側
                a[j + 幅] = x - y; // 奇パリティ側
            }
            i += 幅 << 1;
        }
        幅 <<= 1;
    }
}

#[snippet("xor_畳み込み")]
#[snippet("アダマール_変換")]
#[snippet("WalshHadamard")]
#[snippet("ウォルシュ_アダマール")]
#[snippet("XorConvolution")]
/// XOR 畳み込み。`h[k] = Σ_{i ^ j = k} f[i]·g[j]` を `O(n log n)` で返す（配列長は `2^k`）。
///
/// ビット XOR をキーにした数え上げ・確率分布の合成に使う。
/// 下位/上位集合ゼータ（[`zeta_mobius`](crate::zeta_mobius)）が AND/OR 畳み込みなのに対し、こちらは XOR。
/// 要素型は isize / f64 / ModInt など（符号なし整数は不可。ModInt は mod が奇素数なら `n` で割れる）。
fn xor畳み込み<T>(f: &[T], g: &[T]) -> Vec<T>
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::MulAssign
        + std::ops::DivAssign
        + From<u8>,
{
    assert_eq!(f.len(), g.len(), "f と g は同じ長さ");
    assert!(f.len().is_power_of_two(), "長さは 2 のべき乗");
    let n = f.len();
    if n == 0 {
        return Vec::new();
    }
    let mut a = f.to_vec();
    let mut b = g.to_vec();
    アダマール変換(&mut a);
    アダマール変換(&mut b);
    a.iter_mut().zip(&b).for_each(|(x, y)| *x *= *y);
    アダマール変換(&mut a); // 逆変換は同じ変換 + n で割る
    let 除数 = (0..n.trailing_zeros()).fold(T::from(1u8), |d, _| d + d);
    a.iter_mut().for_each(|x| *x /= 除数);
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    // 定義どおりの総当たり: h[k] = Σ_{i ^ j = k} f[i]·g[j]。
    fn 素朴(f: &[isize], g: &[isize]) -> Vec<isize> {
        let n = f.len();
        let mut h = vec![0isize; n];
        for (i, &fi) in f.iter().enumerate() {
            for (j, &gj) in g.iter().enumerate() {
                h[i ^ j] += fi * gj;
            }
        }
        h
    }

    #[test]
    fn 小さな例() {
        let f = vec![1, 2, 3, 4];
        let g = vec![5, 6, 7, 8];
        assert_eq!(xor畳み込み(&f, &g), 素朴(&f, &g));
    }

    #[test]
    fn 変換は2回で元のn倍() {
        let mut a = vec![3isize, 1, 4, 1, 5, 9, 2, 6];
        let 元 = a.clone();
        アダマール変換(&mut a);
        アダマール変換(&mut a);
        for (x, y) in a.iter().zip(&元) {
            assert_eq!(*x, *y * 8);
        }
    }

    #[test]
    fn modintでも一致() {
        use ac_library::ModInt998244353 as M;
        let f = vec![1isize, 2, 3, 4];
        let g = vec![5isize, 6, 7, 8];
        let 期待 = xor畳み込み(&f, &g);
        let fm: Vec<M> = f.iter().map(|&x| M::new(x)).collect();
        let gm: Vec<M> = g.iter().map(|&x| M::new(x)).collect();
        let 得 = xor畳み込み(&fm, &gm);
        for (a, b) in 得.iter().zip(&期待) {
            assert_eq!(a.val() as isize, *b);
        }
    }

    #[test]
    fn 乱数で素朴と一致() {
        let mut seed = 0x7a17_e3c9u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..300 {
            let k = (乱数() as usize % 7) + 1; // 1..=7 ビット
            let n = 1usize << k;
            let f: Vec<isize> = (0..n).map(|_| (乱数() % 41) as isize - 20).collect();
            let g: Vec<isize> = (0..n).map(|_| (乱数() % 41) as isize - 20).collect();
            assert_eq!(xor畳み込み(&f, &g), 素朴(&f, &g), "k={k}");
        }
    }
}
