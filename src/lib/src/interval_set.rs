use crate::to_int::to_isize;
use cargo_snippet::snippet;
#[snippet("IntervalSet")]
#[snippet("RangeSet")]
#[snippet("区間_mex")]
#[snippet("区間を_set_で_管理")]
#[snippet("区間_管理")]
#[snippet("区間_集合")]
use std::collections::BTreeMap;

#[snippet("区間_集合")]
#[snippet("IntervalSet")]
#[snippet("RangeSet")]
#[snippet("区間を_set_で_管理")]
#[snippet("区間_管理")]
#[snippet("区間_mex")]
/// 互いに素な半開区間 `[l, r)` の集合を `BTreeMap` で管理する「区間を set で管理」テクニックの実装。
///
/// 区間の追加（和）・削除、点の被覆判定、被覆長、`mex`（被覆されていない最小値）を扱う。
/// 隣接する区間（端が接する）は自動的に併合され、常に互いに素かつ非隣接な正規形を保つ。
/// 各操作は併合・分割した区間数を `k` として `O((k+1) log n)`、全体で償却効率が良い。
#[derive(Default, Clone, Debug)]
struct 区間集合 {
    // 始点 -> 終点（半開 [始点, 終点)）。互いに素かつ隣接しない。
    区間: BTreeMap<isize, isize>,
    被覆: isize,
}

#[snippet("区間_集合", include = "to_isize")]
#[snippet("IntervalSet")]
#[snippet("RangeSet")]
#[snippet("区間を_set_で_管理")]
#[snippet("区間_管理")]
#[snippet("区間_mex")]
impl 区間集合 {
    /// 空の区間集合を作る。
    fn new() -> Self {
        Self::default()
    }

    /// `RangeBounds` を半開区間 `[l, r)`（isize）に解決する。土台の値域が無いので両端は必須
    /// （`..` / `l..` / `..r` は panic。`l..r` か `l..=r` を使う）。
    fn 半開<T: Copy + TryInto<isize>>(範囲: impl std::ops::RangeBounds<T>) -> (isize, isize) {
        use std::ops::Bound::{Excluded, Included, Unbounded};
        let l = match 範囲.start_bound() {
            Included(&x) => to_isize(x),
            Excluded(&x) => to_isize(x) + 1,
            Unbounded => panic!("区間集合: 下端は必須（l..r か l..=r を使う）"),
        };
        let r = match 範囲.end_bound() {
            Included(&x) => to_isize(x) + 1,
            Excluded(&x) => to_isize(x),
            Unbounded => panic!("区間集合: 上端は必須（l..r か l..=r を使う）"),
        };
        (l, r)
    }

    /// 区間（`l..r`=半開 / `l..=r`=閉）を追加（和を取る）。重なり・隣接する既存区間はすべて
    /// 1 本に併合される。整数型は任意（usize / isize など。ただし両端は同じ型）。
    fn 追加<T: Copy + TryInto<isize>>(&mut self, 範囲: impl std::ops::RangeBounds<T>) {
        let (l, r) = Self::半開(範囲);
        if l >= r {
            return;
        }
        let mut nl = l;
        let mut nr = r;
        // s <= r かつ e >= l の区間は [l, r) と重なるか接する。
        let 対象: Vec<isize> = self
            .区間
            .range(..=r)
            .filter(|&(_, &e)| e >= l)
            .map(|(&s, _)| s)
            .collect();
        for s in 対象 {
            let e = self.区間.remove(&s).unwrap();
            self.被覆 -= e - s;
            nl = nl.min(s);
            nr = nr.max(e);
        }
        self.被覆 += nr - nl;
        self.区間.insert(nl, nr);
    }

    /// 区間（`l..r`=半開 / `l..=r`=閉）を削除する。またがる既存区間は両端で分割される。
    fn 削除<T: Copy + TryInto<isize>>(&mut self, 範囲: impl std::ops::RangeBounds<T>) {
        let (l, r) = Self::半開(範囲);
        if l >= r {
            return;
        }
        // s < r かつ e > l の区間は [l, r) と真に重なる。
        let 対象: Vec<isize> = self
            .区間
            .range(..r)
            .filter(|&(_, &e)| e > l)
            .map(|(&s, _)| s)
            .collect();
        for s in 対象 {
            let e = self.区間.remove(&s).unwrap();
            self.被覆 -= e - s;
            if s < l {
                self.被覆 += l - s;
                self.区間.insert(s, l); // 左の残り
            }
            if e > r {
                self.被覆 += e - r;
                self.区間.insert(r, e); // 右の残り
            }
        }
    }

    /// 点 `x` が被覆されているか（いずれかの区間に含まれるか）を返す。
    fn 含む(&self, x: impl TryInto<isize>) -> bool {
        let x = to_isize(x);
        self.区間
            .range(..=x)
            .next_back()
            .is_some_and(|(_, &e)| e > x)
    }

    /// 点 `x` を含む区間 `[l, r)` を返す。含まれなければ `None`。
    fn 区間取得(&self, x: impl TryInto<isize>) -> Option<(isize, isize)> {
        let x = to_isize(x);
        self.区間
            .range(..=x)
            .next_back()
            .and_then(|(&s, &e)| if e > x { Some((s, e)) } else { None })
    }

    /// `x` 以上で被覆されていない最小の整数（mex）を返す。`x` 自身が空きならそのまま `x`。
    fn mex(&self, x: impl TryInto<isize>) -> isize {
        let x = to_isize(x);
        match self.区間取得(x) {
            Some((_, e)) => e, // 区間は非隣接なので終点 e は必ず空き
            None => x,
        }
    }

    /// 被覆されている整数の総数（= 全区間の長さの和）。
    fn 被覆長(&self) -> isize {
        self.被覆
    }

    /// 現在の区間一覧を始点昇順の `(l, r)` で返す。
    fn 区間列(&self) -> Vec<(isize, isize)> {
        self.区間.iter().map(|(&s, &e)| (s, e)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 基本動作() {
        let mut s = 区間集合::new();
        s.追加(1..5);
        s.追加(5..=7); // 隣接併合 → [1, 8)（`..=7` は [5,8) と同義）
        assert_eq!(s.区間列(), vec![(1, 8)]);
        assert_eq!(s.被覆長(), 7);
        s.削除(3..4); // [1,3) と [4,8)
        assert_eq!(s.区間列(), vec![(1, 3), (4, 8)]);
        assert!(s.含む(2));
        assert!(!s.含む(3));
        assert_eq!(s.mex(1), 3);
        assert_eq!(s.mex(4), 8);
        assert_eq!(s.mex(0), 0);
    }

    #[test]
    fn 乱数で配列オラクルと一致() {
        const M: isize = 30;
        let mut seed = 0x1efe_5e70u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..200 {
            let mut s = 区間集合::new();
            let mut 被覆 = vec![false; M as usize]; // 整数点 0..M の被覆状態
            for _ in 0..60 {
                let a = (乱数() as isize) % (M + 1);
                let b = (乱数() as isize) % (M + 1);
                let (l, r) = (a.min(b), a.max(b));
                if 乱数() % 2 == 0 {
                    s.追加(l..r);
                    for p in 被覆.iter_mut().take(r as usize).skip(l as usize) {
                        *p = true;
                    }
                } else {
                    s.削除(l..r);
                    for p in 被覆.iter_mut().take(r as usize).skip(l as usize) {
                        *p = false;
                    }
                }
                // 含む
                for (x, &cov) in 被覆.iter().enumerate() {
                    assert_eq!(s.含む(x as isize), cov, "含む x={x}");
                }
                // 被覆長
                let 期待長 = 被覆.iter().filter(|&&b| b).count() as isize;
                assert_eq!(s.被覆長(), 期待長);
                // mex
                for x in 0..M {
                    let 期待 = (x..M).find(|&p| !被覆[p as usize]).unwrap_or(M);
                    // 配列範囲外（>=M）は常に空きなので、期待が M を超えない範囲だけ照合。
                    if 期待 < M {
                        assert_eq!(s.mex(x), 期待, "mex x={x}");
                    }
                }
                // 正規形: 区間は互いに素・非隣接・昇順。
                let v = s.区間列();
                for w in v.windows(2) {
                    assert!(w[0].1 < w[1].0, "非隣接違反 {v:?}");
                }
            }
        }
    }
}
