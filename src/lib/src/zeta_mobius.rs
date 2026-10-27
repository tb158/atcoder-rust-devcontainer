use cargo_snippet::snippet;

#[snippet("部分集合_列挙")]
#[snippet("ビットの_部分集合")]
#[snippet("submask")]
/// `ビット` の部分集合を降順（`ビット` → … → `0`）に全列挙する。空集合を含み `2^popcount` 個
/// （全マスクを回すと合計 `3^N`）。相方は `ビット ^ 部分`。`ビット == 0` なら `0` を 1 個返す。
fn 部分集合(ビット: usize) -> impl Iterator<Item = usize> {
    let mut next = Some(ビット);
    std::iter::from_fn(move || {
        let 今 = next?;
        next = if 今 == 0 { None } else { Some((今 - 1) & ビット) };
        Some(今)
    })
}

#[snippet("部分集合の_分割_列挙", include = "部分集合_列挙")]
#[snippet("ビット_集合の_二分割", include = "部分集合_列挙")]
/// `ビット` を `(最下位ビットを含むグループ, 残り)` に分ける全通り（`2^(popcount-1)` 個）を列挙する。
/// 最下位ビットを片側に固定するので空グループが出ず、`(A, B)` と `(B, A)` を二重に数えない。
/// 集合を空でない部分に切る DP（TSP・集合被覆・シュタイナー木）向け。`ビット != 0` が前提。
fn 分割(ビット: usize) -> impl Iterator<Item = (usize, usize)> {
    let 最下位 = ビット & ビット.wrapping_neg();
    部分集合(ビット ^ 最下位).map(move |s| {
        let グループ = s | 最下位;
        (グループ, ビット ^ グループ)
    })
}

#[snippet("高速_ゼータ_変換")]
#[snippet("高速_メビウス_変換")]
#[snippet("SOSDP")]
#[snippet("部分集合_和")]
#[snippet("下位_集合_ゼータ")]
/// 下位集合に対する高速ゼータ変換（SOS DP, sum over subsets）。
/// `f[S]` を `g[S] = Σ_{T ⊆ S} f[T]` へ in-place に書き換える。O(N · 2^N)。
/// `f.len()` は 2 のべき乗（= 2^N）であること。逆変換は [`下位集合メビウス変換`]。
/// 要素型は加算できれば何でもよい（isize / usize / f64 / ModInt など）。
fn 下位集合ゼータ変換<T: Copy + std::ops::AddAssign>(f: &mut [T]) {
    let n = f.len();
    let mut bit = 1;
    while bit < n {
        for s in 0..n {
            if s & bit != 0 {
                f[s] += f[s ^ bit];
            }
        }
        bit <<= 1;
    }
}

#[snippet("高速_メビウス_変換")]
#[snippet("高速_ゼータ_変換")]
#[snippet("部分集合_和")]
#[snippet("下位_集合_メビウス")]
/// [`下位集合ゼータ変換`] の逆変換。`g[S] = Σ_{T ⊆ S} f[T]` から `f` を復元する。O(N · 2^N)。
/// 符号なし整数は変換途中で負になり得るので isize / ModInt 等を使うこと（ゼータ像の復元なら符号なしでも可）。
fn 下位集合メビウス変換<T: Copy + std::ops::SubAssign>(f: &mut [T]) {
    let n = f.len();
    let mut bit = 1;
    while bit < n {
        for s in 0..n {
            if s & bit != 0 {
                f[s] -= f[s ^ bit];
            }
        }
        bit <<= 1;
    }
}

#[snippet("高速_ゼータ_変換")]
#[snippet("高速_メビウス_変換")]
#[snippet("SOSDP")]
#[snippet("上位_集合_和")]
#[snippet("上位_集合_ゼータ")]
/// 上位集合に対する高速ゼータ変換。`f[S]` を `g[S] = Σ_{T ⊇ S} f[T]` へ in-place に書き換える。
/// O(N · 2^N)。逆変換は [`上位集合メビウス変換`]。要素型は加算できれば何でもよい。
fn 上位集合ゼータ変換<T: Copy + std::ops::AddAssign>(f: &mut [T]) {
    let n = f.len();
    let mut bit = 1;
    while bit < n {
        for s in 0..n {
            if s & bit == 0 {
                f[s] += f[s | bit];
            }
        }
        bit <<= 1;
    }
}

#[snippet("高速_メビウス_変換")]
#[snippet("高速_ゼータ_変換")]
#[snippet("上位_集合_和")]
#[snippet("上位_集合_メビウス")]
/// [`上位集合ゼータ変換`] の逆変換。`g[S] = Σ_{T ⊇ S} f[T]` から `f` を復元する。O(N · 2^N)。
fn 上位集合メビウス変換<T: Copy + std::ops::SubAssign>(f: &mut [T]) {
    let n = f.len();
    let mut bit = 1;
    while bit < n {
        for s in 0..n {
            if s & bit == 0 {
                f[s] -= f[s | bit];
            }
        }
        bit <<= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 部分集合は総当たりと一致() {
        for n in 0..=8usize {
            for mask in 0..(1usize << n) {
                let 得: Vec<usize> = 部分集合(mask).collect();
                // 総当たり: mask 以下の値のうち mask に含まれるものを降順に。
                let 期待: Vec<usize> = (0..=mask).rev().filter(|&s| s & mask == s).collect();
                assert_eq!(得, 期待, "mask={mask:b}");
                assert_eq!(得.len(), 1 << mask.count_ones(), "mask={mask:b}");
            }
        }
    }

    #[test]
    fn 部分集合の総数は3のべき() {
        for n in 0..=7u32 {
            let 総数: usize = (0..(1usize << n)).map(|mask| 部分集合(mask).count()).sum();
            assert_eq!(総数, 3usize.pow(n), "n={n}");
        }
        // 空集合でも空集合自身を 1 個返す。
        assert_eq!(部分集合(0).collect::<Vec<_>>(), vec![0]);
    }

    #[test]
    fn 分割は素な二分割を重複なく全列挙() {
        for n in 1..=8usize {
            for mask in 1..(1usize << n) {
                let 得: Vec<(usize, usize)> = 分割(mask).collect();
                let 最下位 = mask & mask.wrapping_neg();
                let mut 既出 = std::collections::HashSet::new();
                for &(グループ, 残り) in &得 {
                    assert_eq!(グループ | 残り, mask, "mask={mask:b}");
                    assert_eq!(グループ & 残り, 0, "mask={mask:b}");
                    assert_ne!(グループ, 0, "mask={mask:b}");
                    assert_eq!(グループ & 最下位, 最下位, "mask={mask:b}"); // 最下位は常にグループ側
                    assert!(既出.insert(グループ), "重複 mask={mask:b}");
                }
                assert_eq!(得.len(), 1 << (mask.count_ones() - 1), "mask={mask:b}");

                // 総当たり: 素な二分割 (A, B) を A/B 入れ替えで同一視した集合と一致するか。
                let 期待: std::collections::HashSet<(usize, usize)> = (1..=mask)
                    .filter(|&a| a & mask == a)
                    .map(|a| {
                        let b = mask ^ a;
                        if a & 最下位 != 0 { (a, b) } else { (b, a) }
                    })
                    .collect();
                assert_eq!(得.into_iter().collect::<std::collections::HashSet<_>>(), 期待);
            }
        }
    }

    #[test]
    fn 下位集合ゼータとメビウスは逆() {
        let 元 = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut f = 元.clone();
        下位集合ゼータ変換(&mut f);
        // g[0b11] = f[00]+f[01]+f[10]+f[11] = 1+2+3+4 = 10
        assert_eq!(f[0b11], 10);
        assert_eq!(f[0b111], 元.iter().sum::<isize>()); // 全集合 = 総和
        下位集合メビウス変換(&mut f);
        assert_eq!(f, 元);
    }

    #[test]
    fn ジェネリック型でも動く() {
        use ac_library::ModInt998244353 as M;
        let 元: Vec<M> = (1..=8).map(M::new).collect();
        let mut f = 元.clone();
        下位集合ゼータ変換(&mut f);
        assert_eq!(f[0b111], 元.iter().copied().sum::<M>());
        下位集合メビウス変換(&mut f);
        assert_eq!(f, 元);

        // ゼータのみなら符号なし整数でも使える
        let mut g = vec![1usize, 2, 3, 4];
        下位集合ゼータ変換(&mut g);
        assert_eq!(g, vec![1, 3, 4, 10]);
    }

    #[test]
    fn 上位集合ゼータとメビウスは逆() {
        let 元 = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut f = 元.clone();
        上位集合ゼータ変換(&mut f);
        // g[0b00] = 総和（全ての上位集合）
        assert_eq!(f[0], 元.iter().sum::<isize>());
        // g[0b11] = f[11]+f[111]? 0b11 の上位集合は 0b11(3),0b111(7) → 4+8 = 12
        assert_eq!(f[0b11], 4 + 8);
        上位集合メビウス変換(&mut f);
        assert_eq!(f, 元);
    }
}
