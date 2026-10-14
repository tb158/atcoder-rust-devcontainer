use crate::range_bounds::半開;
use cargo_snippet::snippet;

#[snippet("いもす_法", include = "半開_区間")]
#[snippet("いもす")]
#[snippet("区間_加算")]
/// いもす法(1次元)。「区間への一括加算」を O(1) で記録し、最後に O(N) で各点の値へ展開する。
///
/// 区間加算クエリが大量にあり、最終的な各点の値だけが欲しいときに使う。
/// （累積和が「点の配列 → 区間和」なのに対し、いもす法は「区間加算 → 点の値」。）
///
/// 値の型 `T` は加減算できる符号付きの型（`i64`・`f64`・ModInt など）で、
/// `T::default()` が加法単位元（0）であることを前提とする。
///
/// `usize` / `u64` などの符号なし整数は使えない（差分配列が途中で負の値を持つため）。
/// 符号なしで扱いたい場合は `std::num::Wrapping<u64>` を使う（最終値が `u64` に収まるなら正しい）。
struct いもす法<T> {
    差分: Vec<T>,
}

#[snippet("いもす_法")]
#[snippet("いもす")]
#[snippet("区間_加算")]
impl<T> いもす法<T>
where
    // Neg is never called; it only rejects unsigned types (usize/u64) at compile time,
    // whose 0 - 値 in 区間加算 would overflow.
    T: Copy + Default + std::ops::AddAssign + std::ops::SubAssign + std::ops::Neg,
{
    /// 長さ `長さ` の配列に対するいもす法を、全要素 0（`T::default()`）で初期化する。
    fn new(長さ: usize) -> Self {
        Self {
            差分: vec![T::default(); 長さ + 1],
        }
    }

    /// 区間（`a..b` / `a..=b` / `..` など）に `値` を加算する（記録のみ。展開は `展開()`）。
    fn 区間加算(&mut self, 範囲: impl std::ops::RangeBounds<usize>, 値: T) {
        let (l, r) = 半開(範囲, self.差分.len() - 1);
        self.差分[l] += 値;
        self.差分[r] -= 値;
    }

    /// 記録した加算をすべて反映した、各点の最終的な値を返す。
    fn 展開(&self) -> Vec<T> {
        let mut 累積 = T::default();
        self.差分[..self.差分.len() - 1]
            .iter()
            .map(|&d| {
                累積 += d;
                累積
            })
            .collect()
    }
}

#[snippet("二次元_いもす_法", include = "半開_区間")]
#[snippet("いもす_2d")]
/// いもす法(2次元)。矩形領域への一括加算を O(1) で記録し、最後に O(HW) で各マスの値へ展開する。
///
/// 値の型 `T` は加減算できる符号付きの型（`i64`・`f64`・ModInt など）で、
/// `T::default()` が加法単位元（0）であることを前提とする。
///
/// `usize` / `u64` などの符号なし整数は使えない（差分配列が途中で負の値を持つため）。
/// 符号なしで扱いたい場合は `std::num::Wrapping<u64>` を使う（最終値が `u64` に収まるなら正しい）。
struct 二次元いもす法<T> {
    差分: Vec<Vec<T>>,
    高さ: usize,
    幅: usize,
}

#[snippet("二次元_いもす_法")]
#[snippet("いもす_2d")]
impl<T> 二次元いもす法<T>
where
    // Neg is never called; it only rejects unsigned types (usize/u64) at compile time,
    // whose 0 - 値 in 区間加算 would overflow.
    T: Copy + Default + std::ops::AddAssign + std::ops::SubAssign + std::ops::Neg,
{
    /// `高さ × 幅` のグリッドに対する2次元いもす法を、全要素 0（`T::default()`）で初期化する。
    fn new(高さ: usize, 幅: usize) -> Self {
        Self {
            差分: vec![vec![T::default(); 幅 + 1]; 高さ + 1],
            高さ,
            幅,
        }
    }

    /// 行範囲 × 列範囲（各 `a..b` / `a..=b` / `..` など）の矩形領域に `値` を加算する（記録のみ）。
    fn 区間加算(
        &mut self,
        行範囲: impl std::ops::RangeBounds<usize>,
        列範囲: impl std::ops::RangeBounds<usize>,
        値: T,
    ) {
        let (r1, r2) = 半開(行範囲, self.高さ);
        let (c1, c2) = 半開(列範囲, self.幅);
        self.差分[r1][c1] += 値;
        self.差分[r1][c2] -= 値;
        self.差分[r2][c1] -= 値;
        self.差分[r2][c2] += 値;
    }

    /// 記録した加算をすべて反映した、各マスの最終的な値を `高さ × 幅` のグリッドで返す。
    fn 展開(&self) -> Vec<Vec<T>> {
        let mut g = self.差分.clone();
        // 各行を左→右に累積
        for 行 in g.iter_mut() {
            for j in 1..行.len() {
                let 左 = 行[j - 1];
                行[j] += 左;
            }
        }
        // 各列を上→下に累積
        for i in 1..g.len() {
            let (上, 下) = g.split_at_mut(i);
            for (下のマス, &上のマス) in 下[0].iter_mut().zip(上[i - 1].iter()) {
                *下のマス += 上のマス;
            }
        }
        g.truncate(self.高さ);
        for 行 in g.iter_mut() {
            行.truncate(self.幅);
        }
        g
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn いもす法1d() {
        let mut s = いもす法::<i64>::new(5);
        s.区間加算(0..3, 2); // [0,3) に +2
        s.区間加算(2.., 1); // [2,5) に +1（末尾まで）
        assert_eq!(s.展開(), vec![2, 2, 3, 1, 1]);
    }

    #[test]
    fn 符号なしは_wrapping_なら使える() {
        use std::num::Wrapping;
        let mut s = いもす法::<Wrapping<u64>>::new(3);
        s.区間加算(0..2, Wrapping(7u64));
        assert_eq!(
            s.展開().iter().map(|w| w.0).collect::<Vec<_>>(),
            vec![7, 7, 0]
        );
    }

    #[test]
    fn いもす法2d() {
        let mut s = 二次元いもす法::<i64>::new(3, 3);
        s.区間加算(0..2, 0..2, 1); // 左上 2x2 に +1
        s.区間加算(1..3, 1..3, 5); // 右下 2x2 に +5
        assert_eq!(s.展開(), vec![vec![1, 1, 0], vec![1, 6, 5], vec![0, 5, 5]]);
    }
}
