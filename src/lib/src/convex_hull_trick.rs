use crate::to_int::to_isize;
use cargo_snippet::snippet;

#[snippet("CHT")]
#[snippet("ConvexHullTrick")]
#[snippet("凸包_トリック")]
#[snippet("直線_集合_最小_傾き_単調")]
#[snippet("傾き_単調_DP_高速化")]
/// Convex Hull Trick（CHT, 最小クエリ版）。直線 `y = 傾き·x + 切片` を複数追加し、
/// 与えた `x` での最小値を高速に求める。DP の `min_j (a_j·x + b_j)` 型の高速化に使う。
///
/// 追加する直線の傾きは **単調非増加** の順で渡すこと（追加は償却 O(1)）。
/// クエリ `x` は任意で、凸性を使った二分探索で O(log 本数)。
/// 最大値が欲しいときは全ての `傾き`・`切片` の符号を反転して入れ、結果を反転する。
/// 評価は isize 素のまま行うので `傾き·x + 切片` が isize に収まる範囲で使うこと
/// （直線の不要判定は i128 で行うため安全）。
struct ConvexHullTrick {
    直線: Vec<(isize, isize)>, // (傾き, 切片)、下側凸包を成す
}

#[snippet("CHT")]
#[snippet("ConvexHullTrick")]
#[snippet("凸包_トリック")]
#[snippet("直線_集合_最小_傾き_単調")]
#[snippet("傾き_単調_DP_高速化")]
impl Default for ConvexHullTrick {
    fn default() -> Self {
        Self::new()
    }
}

#[snippet("CHT", include = "to_isize")]
#[snippet("ConvexHullTrick")]
#[snippet("凸包_トリック")]
#[snippet("直線_集合_最小_傾き_単調")]
#[snippet("傾き_単調_DP_高速化")]
impl ConvexHullTrick {
    /// 空の直線集合を作る。
    fn new() -> Self {
        Self { 直線: Vec::new() }
    }

    // 中央の直線 l2 が l1（古い）と l3（新しい）に挟まれて不要かを i128 で判定する。
    fn 不要(l1: (isize, isize), l2: (isize, isize), l3: (isize, isize)) -> bool {
        // (l3.切片 - l1.切片)(l1.傾き - l2.傾き) <= (l2.切片 - l1.切片)(l1.傾き - l3.傾き)
        (l3.1 as i128 - l1.1 as i128) * (l1.0 as i128 - l2.0 as i128)
            <= (l2.1 as i128 - l1.1 as i128) * (l1.0 as i128 - l3.0 as i128)
    }

    /// 直線 `y = 傾き·x + 切片` を追加する。傾きは単調非増加の順で渡すこと。
    /// 引数は任意の整数型でよい（[`Self::最小値`] も同様）。
    fn 直線追加(&mut self, 傾き: impl TryInto<isize>, 切片: impl TryInto<isize>) {
        let l = (to_isize(傾き), to_isize(切片));
        while self.直線.len() >= 2 {
            let n = self.直線.len();
            if Self::不要(self.直線[n - 2], self.直線[n - 1], l) {
                self.直線.pop();
            } else {
                break;
            }
        }
        self.直線.push(l);
    }

    fn 評価(l: (isize, isize), x: isize) -> isize {
        l.0 * x + l.1
    }

    /// `x` における全直線の最小値。直線が 1 本も無いときは呼ばないこと。
    fn 最小値(&self, x: impl TryInto<isize>) -> isize {
        let x = to_isize(x);
        debug_assert!(!self.直線.is_empty(), "CHT: 直線が空です");
        let (mut lo, mut hi) = (0usize, self.直線.len() - 1);
        while lo < hi {
            let m = (lo + hi) / 2;
            if Self::評価(self.直線[m], x) > Self::評価(self.直線[m + 1], x) {
                lo = m + 1;
            } else {
                hi = m;
            }
        }
        Self::評価(self.直線[lo], x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 総当たりと一致() {
        // 傾き単調非増加で追加
        let 直線群 = [(5, 3), (3, -1), (2, 4), (0, -2), (-1, 10), (-4, 0)];
        let mut cht = ConvexHullTrick::new();
        for &(a, b) in &直線群 {
            cht.直線追加(a, b);
        }
        for x in -20..=20 {
            let 期待 = 直線群.iter().map(|&(a, b)| a * x + b).min().unwrap();
            assert_eq!(cht.最小値(x), 期待, "x={x}");
        }
    }

    #[test]
    fn 一本でも動く() {
        let mut cht = ConvexHullTrick::new();
        cht.直線追加(2, 5);
        assert_eq!(cht.最小値(3), 11);
    }
}
