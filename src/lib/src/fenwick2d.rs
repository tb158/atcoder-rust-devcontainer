use crate::range_bounds::半開;
use cargo_snippet::snippet;

#[snippet("二次元_BIT", include = "半開_区間")]
#[snippet("二次元_フェニック_木")]
#[snippet("2DBIT")]
#[snippet("二次元_累積和_更新あり")]
/// 二次元 BIT（Binary Indexed Tree / Fenwick 木）。
///
/// 1点加算と「矩形領域の和」を、どちらも O(log H · log W) で行える。
/// 静的な二次元累積和と違い、値の更新が混ざるクエリに使える。
///
/// 要素型 `T` は加減算できる型なら何でもよく（`i64`・`u64`・`f64` など）、
/// `T::default()` が加法単位元（0）であることを前提とする。
struct 二次元BIT<T> {
    高さ: usize,
    幅: usize,
    data: Vec<Vec<T>>,
}

#[snippet("二次元_BIT")]
#[snippet("二次元_フェニック_木")]
#[snippet("2DBIT")]
#[snippet("二次元_累積和_更新あり")]
impl<T> 二次元BIT<T>
where
    T: Copy + Default + std::ops::AddAssign + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    /// `高さ × 幅` の全要素 0（`T::default()`）で初期化する。
    fn new(高さ: usize, 幅: usize) -> Self {
        Self {
            高さ,
            幅,
            data: vec![vec![T::default(); 幅 + 1]; 高さ + 1],
        }
    }

    /// マス `(r, c)`（0-indexed）に `値` を加算する。
    fn 加算(&mut self, r: usize, c: usize, 値: T) {
        let mut i = r + 1;
        while i <= self.高さ {
            let mut j = c + 1;
            while j <= self.幅 {
                self.data[i][j] += 値;
                j += j & j.wrapping_neg();
            }
            i += i & i.wrapping_neg();
        }
    }

    /// 左上から `[0, r) × [0, c)` の矩形和。
    fn 接頭和(&self, r: usize, c: usize) -> T {
        let mut 和 = T::default();
        let mut i = r;
        while i > 0 {
            let mut j = c;
            while j > 0 {
                和 += self.data[i][j];
                j -= j & j.wrapping_neg();
            }
            i -= i & i.wrapping_neg();
        }
        和
    }

    /// 行範囲 × 列範囲（各 `a..b` / `a..=b` / `..` など）の矩形領域の和。
    fn 区間和(
        &self,
        行範囲: impl std::ops::RangeBounds<usize>,
        列範囲: impl std::ops::RangeBounds<usize>,
    ) -> T {
        let (r1, r2) = 半開(行範囲, self.高さ);
        let (c1, c2) = 半開(列範囲, self.幅);
        self.接頭和(r2, c2) + self.接頭和(r1, c1) - self.接頭和(r1, c2) - self.接頭和(r2, c1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 加算と矩形和() {
        let mut bit = 二次元BIT::<i64>::new(4, 4);
        bit.加算(1, 1, 3);
        bit.加算(2, 3, 5);
        bit.加算(0, 0, 2);
        assert_eq!(bit.区間和(.., ..), 10); // 全体
        assert_eq!(bit.区間和(1..3, 1..4), 8); // (1,1)=3 と (2,3)=5
        assert_eq!(bit.区間和(0..1, 0..1), 2); // (0,0)=2 のみ
        assert_eq!(bit.区間和(0..=0, 0..=0), 2); // 閉区間でも可
        assert_eq!(bit.区間和(3..4, 3..4), 0); // 空
    }
}
