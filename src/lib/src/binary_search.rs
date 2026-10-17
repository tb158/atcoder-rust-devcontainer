//! 整数の範囲そのものを二分探索する拡張トレイト `RangeSearch`。
//! `slice::partition_point` の「配列ではなく添字・値の範囲」版で、
//! `(0..n).partition_point(|i| ...)` と書ける。
//! 範囲を `l, l+1, …, r-1` という列とみなし、戻り値は slice と同じく**列の先頭からの添字**
//! （`usize`）。`(l..r).partition_point(f)` は `(l..r).collect::<Vec<_>>().partition_point(|&x| f(x))` と同じ値。
//! `0..n` なら添字と値が一致する。値が欲しいときは `l + 添字` に戻す（負の範囲なら `l + 添字 as isize`）。
//!
//! ## 受け取る範囲
//!
//! `a..b` の `Range<T>` だけ（`a..=b` / `a..` / `..` はコンパイルエラー）。
//! 閉区間で書きたいときは `0..=10usize.pow(18)` ではなく `0..10usize.pow(18) + 1` とする。
//! `3..2` のように逆転した範囲は空区間とみなすので、panic もアンダーフローもしない。
//!
//! 対象は整数型（`usize` / `isize` / `u64` / `i32` など）。`f64` は対象外
//! （実数の二分探索・黄金分割探索は反復回数を決め打ちする別実装を使う）。
//! 型の決まらないリテラルだけの範囲は Rust の既定で `i32` になる。`(0..100)` や `(-100..100)` は
//! そのまま書けるが、`i32` に収まらない値は `1usize..` / `-1isize..` のように接尾辞を付ける。
//! `(0..n)`（`n: usize`）や、述語の中で `a[i]` に使う場合は自動で `usize` になる。
//!
//! ## partition_point
//!
//! 述語の評価は `1 + floor(log2(end - start))` 回。戻り値は必ず `0..=範囲の長さ` に収まる。
//! 述語は `FnMut` なので、評価に可変借用が要るもの（ac-library `LazySegtree::get` など）も渡せる。
//!
//! ```ignore
//! // a が昇順のとき、x 未満の要素数
//! let 個数 = (0..n).partition_point(|i| a[i] < x);
//! // 答えを直接二分探索（めぐる式）。最小の達成可能コスト
//! let 最小コスト = (0..10usize.pow(18) + 1).partition_point(|c| !達成可能(c));
//! // 遅延セグ木の値が条件を満たす間だけ進む（&mut self を要する get も渡せる）
//! // 戻り値は 対象l からの添字なので、位置に戻すには 対象l を足す
//! let 更新r = 対象l + (対象l..対象r.min(n)).partition_point(|i| seg.get(i).0 < 対象r);
//! // 負を含む範囲（l, r: isize）。f(x) が成り立たなくなる最小の x
//! let 境界 = l + (l..r).partition_point(|x| f(x)) as isize;
//! ```
//!
//! ## equal_range
//!
//! `Less`（まだ手前・答えは右）/ `Equal`（該当）/ `Greater`（行き過ぎ・答えは左）が
//! この順に並ぶ前提で、`Equal` の半開区間を添字（`Range<usize>`）で返す。
//! superslice の `equal_range` と同じ仕様で、実体は `partition_point` 2 回。
//! 該当なしなら空区間（全 `Less` なら `長さ..長さ`、全 `Greater` なら `0..0`、
//! 値の間に落ちるならその位置）。逆転範囲・空範囲は `0..0`。

use cargo_snippet::snippet;

#[snippet("range_二分探索")]
#[snippet("binary_search")]
trait RangeSearch<T> {
    /// 述語が最初に false になる要素の、範囲の先頭からの添字を返す（`slice::partition_point` と同じ）。
    /// 全 true なら範囲の長さ、全 false と空範囲なら 0。
    fn partition_point(&self, pred: impl FnMut(T) -> bool) -> usize;

    /// `cmp` が `Less→Equal→Greater` の順に並ぶ前提で `Equal` の半開区間を、範囲の先頭からの添字で返す。
    /// 例: `(0..n).equal_range(|i| a[i].cmp(&target))`
    fn equal_range(&self, cmp: impl FnMut(T) -> std::cmp::Ordering) -> std::ops::Range<usize>;
}

#[snippet("range_二分探索")]
#[snippet("binary_search")]
impl<T> RangeSearch<T> for std::ops::Range<T>
where
    T: Copy
        + Ord
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Shr<u32, Output = T>
        + TryInto<usize>,
{
    fn partition_point(&self, mut pred: impl FnMut(T) -> bool) -> usize {
        let start = self.start;
        let end = self.end.max(start);
        if start == end || !pred(start) {
            return 0;
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
        (right - start)
            .try_into()
            .ok()
            .expect("partition_point: 範囲の長さが usize に収まりません")
    }

    fn equal_range(&self, mut cmp: impl FnMut(T) -> std::cmp::Ordering) -> std::ops::Range<usize> {
        use std::cmp::Ordering::{Greater, Less};
        let lower = self.partition_point(|x| cmp(x) == Less);
        let upper = self.partition_point(|x| cmp(x) != Greater);
        lower..upper
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    /// 線形走査版の partition_point（総当たり検証の答え合わせ用）
    fn naive_partition_point(start: usize, end: usize, pred: impl Fn(usize) -> bool) -> usize {
        (start..end).position(|i| !pred(i)).unwrap_or((start..end).len())
    }

    #[test]
    fn partition_pointの基本() {
        let a = [1, 3, 3, 5, 8];
        // 3 未満の個数
        assert_eq!((0..a.len()).partition_point(|i| a[i] < 3), 1);
        // 3 以下の個数
        assert_eq!((0..a.len()).partition_point(|i| a[i] <= 3), 3);
        // 全 true → 範囲の長さ
        assert_eq!((0..a.len()).partition_point(|i| a[i] < 100), 5);
        // 全 false → 0
        assert_eq!((0..a.len()).partition_point(|i| a[i] < 0), 0);
        // 部分範囲では範囲の先頭からの添字（a[2..4].partition_point と同じ）
        assert_eq!((2..4).partition_point(|i| a[i] < 100), 2);
        assert_eq!((2..4).partition_point(|i| a[i] < 0), 0);
        assert_eq!((2..4).partition_point(|i| a[i] < 5), 1);
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)] // 逆転範囲の扱いを検証するため意図的
    fn 空範囲でも動く() {
        assert_eq!((3..3).partition_point(|_| true), 0);
        assert_eq!((3..2).partition_point(|_| true), 0);
        assert_eq!((3..2).partition_point(|_| false), 0);
        assert_eq!((3..3).equal_range(|_| Ordering::Equal), 0..0);
        assert_eq!((3..2).equal_range(|_| Ordering::Equal), 0..0);
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
        assert_eq!((0isize..10isize.pow(9)).partition_point(|x| x * x < 1000), 32);
        // 下端が負でもアンダーフローしない（戻り値は -100 からの添字）
        assert_eq!((-100isize..100).partition_point(|x| x < -7), 93);
        assert_eq!((-100isize..-49).partition_point(|x| x < -1000), 0);
        assert_eq!((-100isize..100).equal_range(|x| x.cmp(&-30)), 70..71);
    }

    #[test]
    fn 型注釈なしで書ける() {
        // リテラルだけの範囲は i32 になる
        assert_eq!((0..100).partition_point(|x| x * x < 1000), 32);
        assert_eq!((-100..100).partition_point(|x| x < -7), 93);
        assert_eq!((0..100).equal_range(|x| (x / 10).cmp(&3)), 30..40);
        // i32 に収まらない値は接尾辞を付ける
        assert_eq!((1usize..1_000_000_000_000_000_000).partition_point(|d| d < 7), 6);
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
        // 全 Less → 長さ..長さ、全 Greater → 0..0
        assert_eq!((0..a.len()).equal_range(|i| a[i].cmp(&100)), 6..6);
        assert_eq!((0..a.len()).equal_range(|i| a[i].cmp(&0)), 0..0);
        // 端に接するブロック
        assert_eq!((0..a.len()).equal_range(|i| a[i].cmp(&1)), 0..1);
        assert_eq!((0..a.len()).equal_range(|i| a[i].cmp(&8)), 5..6);
        // 部分範囲では範囲の先頭からの添字
        assert_eq!((2..5).equal_range(|i| a[i].cmp(&3)), 0..2);
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
                            let expected = lo - start..hi - start;
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
    fn 総当たり_sliceと同じ() {
        // 部分範囲でも slice::partition_point / superslice::equal_range と一致する
        use superslice::Ext;
        for n in 0..=6usize {
            for mask in 0..3u32.pow(n as u32) {
                let mut v: Vec<usize> = (0..n)
                    .map(|i| (mask / 3u32.pow(i as u32) % 3) as usize)
                    .collect();
                v.sort();
                for start in 0..=n {
                    for end in start..=n {
                        for x in 0..4usize {
                            assert_eq!(
                                (start..end).partition_point(|i| v[i] < x),
                                v[start..end].partition_point(|&y| y < x),
                                "v={v:?} start={start} end={end} x={x}"
                            );
                            assert_eq!(
                                (start..end).equal_range(|i| v[i].cmp(&x)),
                                v[start..end].equal_range(&x),
                                "v={v:?} start={start} end={end} x={x}"
                            );
                        }
                    }
                }
            }
        }
    }
}
