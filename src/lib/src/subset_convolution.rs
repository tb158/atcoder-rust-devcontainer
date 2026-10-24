use cargo_snippet::snippet;

#[snippet("部分集合_畳み込み")]
#[snippet("SubsetConvolution")]
#[snippet("ランク付き_ゼータ")]
#[snippet("部分集合_和_畳み込み")]
/// 下位集合方向のゼータ変換（SOS）。`f[S]` を `Σ_{T ⊆ S} f[T]` へ in-place 変換する。`O(n · 2^n)`。
/// 部分集合畳み込み内部で各ランク層に適用する補助関数。
fn 部分集合ゼータ<T: Copy + std::ops::AddAssign>(f: &mut [T]) {
    let m = f.len();
    let mut bit = 1;
    while bit < m {
        for s in 0..m {
            if s & bit != 0 {
                f[s] += f[s ^ bit];
            }
        }
        bit <<= 1;
    }
}

#[snippet("部分集合_畳み込み")]
#[snippet("SubsetConvolution")]
#[snippet("ランク付き_ゼータ")]
#[snippet("部分集合_和_畳み込み")]
/// [`部分集合ゼータ`] の逆（メビウス）。`Σ_{T ⊆ S} f[T]` から `f` を復元する。`O(n · 2^n)`。
fn 部分集合メビウス<T: Copy + std::ops::SubAssign>(f: &mut [T]) {
    let m = f.len();
    let mut bit = 1;
    while bit < m {
        for s in 0..m {
            if s & bit != 0 {
                f[s] -= f[s ^ bit];
            }
        }
        bit <<= 1;
    }
}

#[snippet("部分集合_畳み込み")]
#[snippet("SubsetConvolution")]
#[snippet("ランク付き_ゼータ")]
#[snippet("部分集合_和_畳み込み")]
/// 部分集合畳み込み（subset sum convolution）。`h[S] = Σ_{T ⊆ S} f[T]·g[S∖T]` を
/// `O(n^2 · 2^n)` で求める（`n` はビット数、配列長は `2^n`）。`T` と `S∖T` が**素**（重なり無し）な
/// 分割だけを足し合わせる点が、単なる下位集合ゼータ（[`zeta_mobius`](crate::zeta_mobius)）の積とは異なる。
///
/// 連結部分グラフ数え上げ・集合被覆 DP・独立集合の合成などに使う。
///
/// 要素型は加減乗できれば何でもよい（isize / f64 / ModInt など）。isize は乗算の重なりで
/// オーバーフローし得るので、MOD 付き数え上げでは ModInt をそのまま渡す。
fn 部分集合畳み込み<T>(f: &[T], g: &[T]) -> Vec<T>
where
    T: Copy + Default + std::ops::AddAssign + std::ops::SubAssign + std::ops::Mul<Output = T>,
{
    let m = f.len();
    assert_eq!(m, g.len(), "f と g は同じ長さ");
    assert!(m.is_power_of_two(), "長さは 2 のべき乗");
    let n = m.trailing_zeros() as usize; // ビット数

    // ランク付きゼータ: fh[k][S] = (popcount(S)==k ? f[S] : 0) を部分集合ゼータ変換したもの。
    let mut fh = vec![vec![T::default(); m]; n + 1];
    let mut gh = vec![vec![T::default(); m]; n + 1];
    for (s, (&fv, &gv)) in f.iter().zip(g).enumerate() {
        let k = (s as u32).count_ones() as usize;
        fh[k][s] = fv;
        gh[k][s] = gv;
    }
    for k in 0..=n {
        部分集合ゼータ(&mut fh[k]);
        部分集合ゼータ(&mut gh[k]);
    }

    // 各 S でランクについての（n を超えない範囲の）畳み込み。
    let mut hh = vec![vec![T::default(); m]; n + 1];
    for i in 0..=n {
        for j in 0..=(n - i) {
            for s in 0..m {
                hh[i + j][s] += fh[i][s] * gh[j][s];
            }
        }
    }

    // 各ランク層をメビウスで戻し、popcount に対応する層の値を取り出す。
    for 層 in hh.iter_mut() {
        部分集合メビウス(層);
    }
    let mut h = vec![T::default(); m];
    for (s, hs) in h.iter_mut().enumerate() {
        let k = (s as u32).count_ones() as usize;
        *hs = hh[k][s];
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    // 定義どおりの総当たり: h[S] = Σ_{T ⊆ S} f[T]·g[S∖T]。
    fn 素朴(f: &[isize], g: &[isize]) -> Vec<isize> {
        let m = f.len();
        let mut h = vec![0isize; m];
        for s in 0..m {
            // S の部分集合 T をすべて列挙。
            let mut t = s;
            loop {
                h[s] += f[t] * g[s ^ t];
                if t == 0 {
                    break;
                }
                t = (t - 1) & s;
            }
        }
        h
    }

    #[test]
    fn 小さな例() {
        // n=2: f, g の長さ 4。
        let f = vec![1, 2, 3, 4];
        let g = vec![5, 6, 7, 8];
        assert_eq!(部分集合畳み込み(&f, &g), 素朴(&f, &g));
    }

    #[test]
    fn modintでも一致() {
        use ac_library::ModInt998244353 as M;
        let f = vec![1isize, 2, 3, 4, 5, 6, 7, 8];
        let g = vec![8isize, 7, 6, 5, 4, 3, 2, 1];
        let 期待 = 部分集合畳み込み(&f, &g);
        let fm: Vec<M> = f.iter().map(|&x| M::new(x)).collect();
        let gm: Vec<M> = g.iter().map(|&x| M::new(x)).collect();
        let 得 = 部分集合畳み込み(&fm, &gm);
        for (a, b) in 得.iter().zip(&期待) {
            assert_eq!(a.val() as isize, *b);
        }
    }

    #[test]
    fn 乱数で素朴と一致() {
        let mut seed = 0x5ab5_e7c0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..300 {
            let n = (乱数() as usize % 6) + 1; // 1..=6 ビット
            let m = 1usize << n;
            let f: Vec<isize> = (0..m).map(|_| (乱数() % 21) as isize - 10).collect();
            let g: Vec<isize> = (0..m).map(|_| (乱数() % 21) as isize - 10).collect();
            assert_eq!(部分集合畳み込み(&f, &g), 素朴(&f, &g), "n={n}");
        }
    }
}
