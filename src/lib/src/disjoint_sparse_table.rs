use crate::range_bounds::半開;
use cargo_snippet::snippet;

#[snippet("分割_スパース_テーブル", include = "半開_区間")]
#[snippet("DisjointSparseTable")]
#[snippet("非冪等_区間_クエリ")]
#[snippet("区間_積")]
#[snippet("区間_和_クエリ")]
/// 分割スパーステーブル（Disjoint Sparse Table）。静的（更新なし）配列に対し、
/// 「結合的だが冪等とは限らない」二項演算（和・積・xor・gcd・行列積・アフィン合成など）の
/// 区間クエリを前処理 `O(N log N)`・各クエリ `O(1)` で答える。
///
/// 通常のスパーステーブル（[`crate::sparse_table`]）は冪等演算（min/max/gcd）専用。こちらは
/// **非可換・非冪等**な演算でも使える。
///
/// - `merge` は結合的であること（可換・冪等は不要）。順序は `merge(左側, 右側)` で保たれる。
struct 分割スパーステーブル<T, F> {
    元: Vec<T>,
    テーブル: Vec<Vec<T>>,
    merge: F,
}

#[snippet("分割_スパース_テーブル")]
#[snippet("DisjointSparseTable")]
#[snippet("非冪等_区間_クエリ")]
#[snippet("区間_積")]
#[snippet("区間_和_クエリ")]
impl<T: Copy, F: Fn(T, T) -> T> 分割スパーステーブル<T, F> {
    /// 配列と結合的な二項演算 `merge` から分割スパーステーブルを構築する。
    ///
    /// # 例
    /// ```ignore
    /// let st = 分割スパーステーブル::new(&a, |x, y| x + y); // 区間和
    /// assert_eq!(st.区間(1..4), a[1] + a[2] + a[3]);
    /// ```
    // Indices are read with offsets (`i-1`, `i+1`) and in reverse, and `k` drives the shift
    // width `1 << k`, so plain range loops are clearer than `enumerate` here.
    #[allow(clippy::needless_range_loop)]
    fn new(配列: &[T], merge: F) -> Self {
        let n = 配列.len();
        let mut レベル数 = 1;
        while (1usize << レベル数) < n {
            レベル数 += 1;
        }
        // 各レベルの行を元配列で初期化（ブロック中央の要素はそのままで正しい）。
        let mut テーブル = vec![配列.to_vec(); レベル数];
        for k in 0..レベル数 {
            let 半 = 1usize << k;
            let 幅 = 半 * 2;
            let mut s = 0;
            while s < n {
                let 中 = s + 半; // 右半分の先頭（左半分は [s, 中)）
                // 左半分: 中央へ向かう累積 op(a[i], ..., a[中-1])。
                let 左終 = 中.min(n);
                for i in (s..左終 - 1).rev() {
                    テーブル[k][i] = merge(配列[i], テーブル[k][i + 1]);
                }
                // 右半分: 中央からの累積 op(a[中], ..., a[i])。
                if 中 < n {
                    let 右終 = (s + 幅).min(n);
                    for i in 中 + 1..右終 {
                        テーブル[k][i] = merge(テーブル[k][i - 1], 配列[i]);
                    }
                }
                s += 幅;
            }
        }
        Self {
            元: 配列.to_vec(),
            テーブル,
            merge,
        }
    }

    /// 区間（`a..b` / `a..=b` / `..` など）に対する `merge` の畳み込み（左→右の順）を
    /// O(1) で返す。空区間は問い合わせ不可。
    fn 区間(&self, 範囲: impl std::ops::RangeBounds<usize>) -> T {
        let (l, r) = 半開(範囲, self.元.len());
        debug_assert!(l < r, "分割スパーステーブル: 空区間は問い合わせ不可");
        let 右 = r - 1; // 閉区間の右端
        if l == 右 {
            return self.元[l];
        }
        // l と 右 が初めて分かれる最上位ビット = 両者を含む最小ブロックのレベル。
        let k = (usize::BITS - 1 - (l ^ 右).leading_zeros()) as usize;
        (self.merge)(self.テーブル[k][l], self.テーブル[k][右])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 最大公約数(mut a: isize, mut b: isize) -> isize {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a.abs() // 非負に正規化（符号は結合順序に依存しうるため）
    }

    #[test]
    fn 区間和と区間最大() {
        let a = [3isize, 1, 4, 1, 5, 9, 2, 6];
        let 和 = 分割スパーステーブル::new(&a, |x, y| x + y);
        assert_eq!(和.区間(..), a.iter().sum()); // 全体
        assert_eq!(和.区間(2..5), 4 + 1 + 5);
        assert_eq!(和.区間(3..4), 1); // 単一要素
        assert_eq!(和.区間(3..=3), 1); // 閉区間でも可

        let 最大 = 分割スパーステーブル::new(&a, std::cmp::max);
        assert_eq!(最大.区間(0..4), 4);
        assert_eq!(最大.区間(3..8), 9);
    }

    #[test]
    fn 乱数で区間和と一致() {
        let mut seed = 0x1234_abcdu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..200 {
            let n = (乱数() as usize % 60) + 1;
            let a: Vec<isize> = (0..n).map(|_| (乱数() % 201) as isize - 100).collect();
            let st = 分割スパーステーブル::new(&a, |x, y| x + y);
            let gst = 分割スパーステーブル::new(&a, 最大公約数);
            for _ in 0..40 {
                let l = 乱数() as usize % n;
                let r = l + 1 + 乱数() as usize % (n - l);
                let 和: isize = a[l..r].iter().sum();
                assert_eq!(st.区間(l..r), 和, "和 a={a:?} l={l} r={r}");
                let mut g = a[l]; // DST は単一要素で元の値を返すため、始点も a[l]
                for &v in &a[l + 1..r] {
                    g = 最大公約数(g, v);
                }
                assert_eq!(gst.区間(l..r), g, "gcd a={a:?} l={l} r={r}");
            }
        }
    }

    #[test]
    fn 非可換アフィン合成と一致() {
        // x -> a*x + b の合成（左から順に適用）。非可換だが結合的。
        const M: isize = 1_000_000_007;
        let 合成 = |f: (isize, isize), g: (isize, isize)| {
            // 先に f、その後 g を適用: x -> g.0*(f.0*x + f.1) + g.1
            ((g.0 * f.0) % M, (g.0 * f.1 + g.1) % M)
        };
        let mut seed = 0xfeed_face_u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..150 {
            let n = (乱数() as usize % 50) + 1;
            let a: Vec<(isize, isize)> = (0..n)
                .map(|_| ((乱数() % M as u64) as isize, (乱数() % M as u64) as isize))
                .collect();
            let st = 分割スパーステーブル::new(&a, 合成);
            for _ in 0..40 {
                let l = 乱数() as usize % n;
                let r = l + 1 + 乱数() as usize % (n - l);
                let mut acc = a[l];
                for &v in &a[l + 1..r] {
                    acc = 合成(acc, v);
                }
                assert_eq!(st.区間(l..r), acc, "a={a:?} l={l} r={r}");
            }
        }
    }
}
