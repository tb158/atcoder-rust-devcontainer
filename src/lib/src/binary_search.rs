//! 整数の範囲そのものを二分探索する拡張トレイト `RangeSearch`。
//! `slice::partition_point` の「配列ではなく添字・値の範囲」版で、
//! `(0..n).partition_point(|i| ...)` と書ける。
//!
//! ## 受け取る範囲
//!
//! `a..b` の `Range<T>` だけ（`a..=b` / `a..` / `..` はコンパイルエラー）。
//! 閉区間で書きたいときは `0..=10i64.pow(18)` ではなく `0..10i64.pow(18) + 1` とする。
//! `3..2` のように逆転した範囲は空区間に潰すので、panic もアンダーフローもしない。
//!
//! 対象は整数型（`usize` / `i64` / `u64` / `i32` など）。`f64` は対象外
//! （実数の二分探索・黄金分割探索は反復回数を決め打ちする別実装を使う）。
//!
//! ## partition_point
//!
//! 述語の評価は `1 + floor(log2(end - start))` 回。戻り値は必ず `start..=end` に収まる。
//! 述語は `FnMut` なので、評価に可変借用が要るもの（ac-library `LazySegtree::get` など）も渡せる。
//!
//! ```ignore
//! // a が昇順のとき、x 未満の要素数
//! let 個数 = (0..n).partition_point(|i| a[i] < x);
//! // 答えを直接二分探索（めぐる式）。最小の達成可能コスト
//! let 最小コスト = (0..10i64.pow(18) + 1).partition_point(|c| !達成可能(c));
//! // 遅延セグ木の値が条件を満たす間だけ進む（&mut self を要する get も渡せる）
//! let 更新r = (対象l..対象r.min(n)).partition_point(|i| seg.get(i).0 < 対象r);
//! ```
//!
//! ## equal_range
//!
//! `Less`（まだ手前・答えは右）/ `Equal`（該当）/ `Greater`（行き過ぎ・答えは左）が
//! この順に並ぶ前提で、`Equal` の半開区間を返す。実体は `partition_point` 2 回。
//! 該当なしなら空区間（全 `Less` なら `end..end`、全 `Greater` なら `start..start`、
//! 値の間に落ちるならその位置）。逆転範囲・空範囲は `start..start`。

use cargo_snippet::snippet;

#[snippet("range_二分探索")]
#[snippet("binary_search")]
trait RangeSearch<T> {
    /// 述語が最初に false になる位置を返す。全 true なら上端、全 false と空範囲なら下端。
    fn partition_point(&self, pred: impl FnMut(T) -> bool) -> T;

    /// `cmp` が `Less→Equal→Greater` の順に並ぶ前提で `Equal` の半開区間を返す。
    /// 例: `(0..n).equal_range(|i| a[i].cmp(&target))`
    fn equal_range(&self, cmp: impl FnMut(T) -> std::cmp::Ordering) -> std::ops::Range<T>;
}

#[snippet("range_二分探索")]
#[snippet("binary_search")]
impl<T> RangeSearch<T> for std::ops::Range<T>
where
    T: Copy
        + Ord
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Shr<u32, Output = T>,
{
    fn partition_point(&self, mut pred: impl FnMut(T) -> bool) -> T {
        let start = self.start;
        let end = self.end.max(start);
        if start == end || !pred(start) {
            return start;
        }
        let (mut left, mut right) = (start, end);
        loop {
            let mid = left + ((right - left) >> 1);
            if mid == left {
                break;
            }
            if pred(mid) {
                left = mid;
            } else {
                right = mid;
            }
        }
        right
    }

    fn equal_range(&self, mut cmp: impl FnMut(T) -> std::cmp::Ordering) -> std::ops::Range<T> {
        use std::cmp::Ordering::{Greater, Less};
        let lower = (self.start..self.end).partition_point(|x| cmp(x) == Less);
        let upper = (lower..self.end).partition_point(|x| cmp(x) != Greater);
        lower..upper
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    /// 線形走査版の partition_point（総当たり検証の答え合わせ用）
    fn naive_partition_point(start: usize, end: usize, pred: impl Fn(usize) -> bool) -> usize {
        (start..end).find(|&i| !pred(i)).unwrap_or(end.max(start))
    }

    #[test]
    fn partition_pointの基本() {
        let a = [1, 3, 3, 5, 8];
        // 3 未満の個数
        assert_eq!((0..a.len()).partition_point(|i| a[i] < 3), 1);
        // 3 以下の個数
        assert_eq!((0..a.len()).partition_point(|i| a[i] <= 3), 3);
        // 全 true → 上端
        assert_eq!((0..a.len()).partition_point(|i| a[i] < 100), 5);
        // 全 false → 下端
        assert_eq!((0..a.len()).partition_point(|i| a[i] < 0), 0);
        // 部分範囲でも下端・上端は範囲のもの
        assert_eq!((2..4).partition_point(|i| a[i] < 100), 4);
        assert_eq!((2..4).partition_point(|i| a[i] < 0), 2);
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)] // 逆転範囲の扱いを検証するため意図的
    fn 空範囲でも動く() {
        assert_eq!((3..3).partition_point(|_| true), 3);
        assert_eq!((3..2).partition_point(|_| true), 3);
        assert_eq!((3..2).partition_point(|_| false), 3);
        assert_eq!((3..3).equal_range(|_| Ordering::Equal), 3..3);
        assert_eq!((3..2).equal_range(|_| Ordering::Equal), 3..3);
        // 述語は一度も呼ばれない
        let mut calls = 0;
        (3..2).partition_point(|_| {
            calls += 1;
            true
        });
        assert_eq!(calls, 0);
    }

    #[test]
    fn 負の値() {
        // x*x >= 1000 になる最小の x
        assert_eq!((0i64..10i64.pow(9)).partition_point(|x| x * x < 1000), 32);
        // 下端が負でもアンダーフローしない
        assert_eq!((-100i64..100).partition_point(|x| x < -7), -7);
        assert_eq!((-100i64..-49).partition_point(|x| x < -1000), -100);
        assert_eq!((-100i64..100).equal_range(|x| x.cmp(&-30)), -30..-29);
    }

    #[test]
    fn 述語は可変借用を取れる() {
        let mut calls = 0usize;
        let pos = (0..1024).partition_point(|i| {
            calls += 1;
            i < 700
        });
        assert_eq!(pos, 700);
        // 下端の 1 回 + log2(1024) 回
        assert_eq!(calls, 11);
    }

    #[test]
    fn equal_rangeの基本() {
        let a = [1, 3, 3, 3, 5, 8];
        assert_eq!((0..a.len()).equal_range(|i| a[i].cmp(&3)), 1..4);
        // 該当なし（値の間に落ちる）
        assert_eq!((0..a.len()).equal_range(|i| a[i].cmp(&4)), 4..4);
        // 全 Less → 上端の空区間、全 Greater → 下端の空区間
        assert_eq!((0..a.len()).equal_range(|i| a[i].cmp(&100)), 6..6);
        assert_eq!((0..a.len()).equal_range(|i| a[i].cmp(&0)), 0..0);
        // 端に接するブロック
        assert_eq!((0..a.len()).equal_range(|i| a[i].cmp(&1)), 0..1);
        assert_eq!((0..a.len()).equal_range(|i| a[i].cmp(&8)), 5..6);
        // 部分範囲
        assert_eq!((2..5).equal_range(|i| a[i].cmp(&3)), 2..4);
    }

    #[test]
    fn equal_rangeは内部ブロックも対数回() {
        // 1024 点のうち 1 点だけ Equal でも見つかる
        let mut calls = 0usize;
        let range = (0..1024).equal_range(|i| {
            calls += 1;
            i.cmp(&500)
        });
        assert_eq!(range, 500..501);
        assert!(calls <= 25, "呼び出し回数が多すぎる: {calls}");
    }

    #[test]
    fn 総当たり_partition_point() {
        for n in 0..=8usize {
            for start in 0..=n {
                for end in 0..=n {
                    // 単調な述語をすべて列挙（境界 k の位置を動かす）
                    for k in 0..=n + 1 {
                        let pred = |i: usize| i < k;
                        assert_eq!(
                            (start..end).partition_point(pred),
                            naive_partition_point(start, end, pred),
                            "n={n} start={start} end={end} k={k}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn 総当たり_equal_range() {
        for n in 0..=8usize {
            for start in 0..=n {
                for end in start..=n {
                    for lo in start..=end {
                        for hi in lo..=end {
                            // [start, lo) が Less、[lo, hi) が Equal、[hi, end) が Greater
                            let cmp = |i: usize| {
                                if i < lo {
                                    Ordering::Less
                                } else if i < hi {
                                    Ordering::Equal
                                } else {
                                    Ordering::Greater
                                }
                            };
                            let expected = if lo == hi { hi..hi } else { lo..hi };
                            assert_eq!(
                                (start..end).equal_range(cmp),
                                expected,
                                "start={start} end={end} lo={lo} hi={hi}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn 総当たり_superslice相当() {
        // ソート済み配列に対する lower_bound / upper_bound と一致することを確認
        for n in 0..=6usize {
            for mask in 0..3u32.pow(n as u32) {
                let mut v: Vec<usize> = (0..n)
                    .map(|i| (mask / 3u32.pow(i as u32) % 3) as usize)
                    .collect();
                v.sort();
                for x in 0..4usize {
                    let expected_lower = v.iter().position(|&y| y >= x).unwrap_or(n);
                    let expected_upper = v.iter().position(|&y| y > x).unwrap_or(n);
                    assert_eq!(
                        (0..n).equal_range(|i| v[i].cmp(&x)),
                        expected_lower..expected_upper,
                        "v={v:?} x={x}"
                    );
                }
            }
        }
    }
}
