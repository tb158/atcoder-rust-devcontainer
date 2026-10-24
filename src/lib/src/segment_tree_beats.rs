use crate::range_bounds::半開;
use crate::to_int::to_isize;
use cargo_snippet::snippet;

#[snippet("セグ木_ビーツ", include = "半開_区間")]
#[snippet("SegmentTreeBeats")]
#[snippet("区間_chmin")]
#[snippet("区間_chmax")]
#[snippet("区間_最小値_更新")]
/// Segment Tree Beats の 1 ノード。最大側（最大値・厳密な 2 番目・最大値の個数）と
/// 最小側（最小値・厳密な 2 番目・最小値の個数）、区間和、要素数を持つ。
#[derive(Clone)]
struct 節点 {
    最大1: isize,
    最大2: isize,
    最大個数: isize,
    最小1: isize,
    最小2: isize,
    最小個数: isize,
    和: isize,
    要素数: isize,
}

#[snippet("SegmentTreeBeats")]
#[snippet("セグ木_ビーツ")]
#[snippet("区間_chmax")]
#[snippet("区間_chmin")]
#[snippet("区間_最小値_更新")]
const 無限大: isize = 1 << 60;

#[snippet("セグ木_ビーツ")]
#[snippet("SegmentTreeBeats")]
#[snippet("区間_chmin")]
#[snippet("区間_chmax")]
#[snippet("区間_最小値_更新")]
impl 節点 {
    fn 葉(v: isize) -> Self {
        Self {
            最大1: v,
            最大2: -無限大,
            最大個数: 1,
            最小1: v,
            最小2: 無限大,
            最小個数: 1,
            和: v,
            要素数: 1,
        }
    }

    fn 空() -> Self {
        Self {
            最大1: -無限大,
            最大2: -無限大,
            最大個数: 0,
            最小1: 無限大,
            最小2: 無限大,
            最小個数: 0,
            和: 0,
            要素数: 0,
        }
    }
}

#[snippet("セグ木_ビーツ")]
#[snippet("SegmentTreeBeats")]
#[snippet("区間_chmin")]
#[snippet("区間_chmax")]
#[snippet("区間_最小値_更新")]
/// 2 つの子ノードを統合する。
fn 合成(l: &節点, r: &節点) -> 節点 {
    let mut p = 節点::空();
    p.和 = l.和 + r.和;
    p.要素数 = l.要素数 + r.要素数;
    // 最大側
    if l.最大1 == r.最大1 {
        p.最大1 = l.最大1;
        p.最大個数 = l.最大個数 + r.最大個数;
        p.最大2 = l.最大2.max(r.最大2);
    } else if l.最大1 > r.最大1 {
        p.最大1 = l.最大1;
        p.最大個数 = l.最大個数;
        p.最大2 = l.最大2.max(r.最大1);
    } else {
        p.最大1 = r.最大1;
        p.最大個数 = r.最大個数;
        p.最大2 = r.最大2.max(l.最大1);
    }
    // 最小側
    if l.最小1 == r.最小1 {
        p.最小1 = l.最小1;
        p.最小個数 = l.最小個数 + r.最小個数;
        p.最小2 = l.最小2.min(r.最小2);
    } else if l.最小1 < r.最小1 {
        p.最小1 = l.最小1;
        p.最小個数 = l.最小個数;
        p.最小2 = l.最小2.min(r.最小1);
    } else {
        p.最小1 = r.最小1;
        p.最小個数 = r.最小個数;
        p.最小2 = r.最小2.min(l.最小1);
    }
    p
}

#[snippet("セグ木_ビーツ")]
#[snippet("SegmentTreeBeats")]
#[snippet("区間_chmin")]
#[snippet("区間_chmax")]
#[snippet("区間_最小値_更新")]
/// Segment Tree Beats。区間 chmin（`a[i] = min(a[i], x)`）・区間 chmax・区間和・区間最大・区間最小を
/// ならし O(log^2 N) で処理する。`ac_library::LazySegtree` では chmin/chmax は扱えない。
///
/// - [`Self::区間chmin`] / [`Self::区間chmax`] : 半開区間 `[l, r)` への min/max 更新。
/// - [`Self::区間和`] / [`Self::区間最大`] / [`Self::区間最小`] : 半開区間 `[l, r)` の集計。
struct セグ木ビーツ {
    n: usize,
    木: Vec<節点>,
}

#[snippet("セグ木_ビーツ", include = "to_isize")]
#[snippet("SegmentTreeBeats")]
#[snippet("区間_chmin")]
#[snippet("区間_chmax")]
#[snippet("区間_最小値_更新")]
impl セグ木ビーツ {
    /// 初期配列から構築する。
    fn new(配列: &[isize]) -> Self {
        let n = 配列.len();
        let mut t = Self {
            n,
            木: vec![節点::空(); 4 * n.max(1)],
        };
        if n > 0 {
            t.構築(1, 0, n, 配列);
        }
        t
    }

    fn 構築(&mut self, 節: usize, lo: usize, hi: usize, 配列: &[isize]) {
        if hi - lo == 1 {
            self.木[節] = 節点::葉(配列[lo]);
            return;
        }
        let m = (lo + hi) / 2;
        self.構築(2 * 節, lo, m, 配列);
        self.構築(2 * 節 + 1, m, hi, 配列);
        self.統合(節);
    }

    fn 統合(&mut self, 節: usize) {
        self.木[節] = 合成(&self.木[2 * 節].clone(), &self.木[2 * 節 + 1].clone());
    }

    /// 節点 `節` の最大値を `x` 以下に抑える（`x > 最大2` が前提）。
    fn chmin適用(&mut self, 節: usize, x: isize) {
        let nd = &mut self.木[節];
        if x >= nd.最大1 {
            return;
        }
        nd.和 -= (nd.最大1 - x) * nd.最大個数;
        if nd.最小1 == nd.最大1 {
            nd.最小1 = x;
        } else if nd.最小2 == nd.最大1 {
            nd.最小2 = x;
        }
        nd.最大1 = x;
    }

    /// 節点 `節` の最小値を `x` 以上に持ち上げる（`x < 最小2` が前提）。
    fn chmax適用(&mut self, 節: usize, x: isize) {
        let nd = &mut self.木[節];
        if x <= nd.最小1 {
            return;
        }
        nd.和 += (x - nd.最小1) * nd.最小個数;
        if nd.最大1 == nd.最小1 {
            nd.最大1 = x;
        } else if nd.最大2 == nd.最小1 {
            nd.最大2 = x;
        }
        nd.最小1 = x;
    }

    /// 親の cap（最大1）・floor（最小1）を子へ伝播する。
    fn 伝播(&mut self, 節: usize) {
        let cap = self.木[節].最大1;
        let floor = self.木[節].最小1;
        for 子 in [2 * 節, 2 * 節 + 1] {
            self.chmin適用(子, cap);
            self.chmax適用(子, floor);
        }
    }

    /// 区間（`a..b` / `a..=b` / `..` など）に対し `a[i] = min(a[i], x)`。ならし `O(log² N)`。
    fn 区間chmin(&mut self, 範囲: impl std::ops::RangeBounds<usize>, x: impl TryInto<isize>) {
        let x = to_isize(x);
        let (l, r) = 半開(範囲, self.n);
        if l < r {
            self.chmin内部(1, 0, self.n, l, r, x);
        }
    }

    fn chmin内部(&mut self, 節: usize, lo: usize, hi: usize, l: usize, r: usize, x: isize) {
        if r <= lo || hi <= l || self.木[節].最大1 <= x {
            return; // 範囲外、または効果なし
        }
        if l <= lo && hi <= r && self.木[節].最大2 < x {
            self.chmin適用(節, x); // 最大値群だけが下がる
            return;
        }
        self.伝播(節);
        let m = (lo + hi) / 2;
        self.chmin内部(2 * 節, lo, m, l, r, x);
        self.chmin内部(2 * 節 + 1, m, hi, l, r, x);
        self.統合(節);
    }

    /// 区間（`a..b` / `a..=b` / `..` など）に対し `a[i] = max(a[i], x)`。ならし `O(log² N)`。
    fn 区間chmax(&mut self, 範囲: impl std::ops::RangeBounds<usize>, x: impl TryInto<isize>) {
        let x = to_isize(x);
        let (l, r) = 半開(範囲, self.n);
        if l < r {
            self.chmax内部(1, 0, self.n, l, r, x);
        }
    }

    fn chmax内部(&mut self, 節: usize, lo: usize, hi: usize, l: usize, r: usize, x: isize) {
        if r <= lo || hi <= l || self.木[節].最小1 >= x {
            return;
        }
        if l <= lo && hi <= r && self.木[節].最小2 > x {
            self.chmax適用(節, x);
            return;
        }
        self.伝播(節);
        let m = (lo + hi) / 2;
        self.chmax内部(2 * 節, lo, m, l, r, x);
        self.chmax内部(2 * 節 + 1, m, hi, l, r, x);
        self.統合(節);
    }

    /// 区間（`a..b` / `a..=b` / `..` など）の総和。ならし `O(log N)`。
    fn 区間和(&mut self, 範囲: impl std::ops::RangeBounds<usize>) -> isize {
        let (l, r) = 半開(範囲, self.n);
        self.和内部(1, 0, self.n, l, r)
    }

    fn 和内部(&mut self, 節: usize, lo: usize, hi: usize, l: usize, r: usize) -> isize {
        if r <= lo || hi <= l {
            return 0;
        }
        if l <= lo && hi <= r {
            return self.木[節].和;
        }
        self.伝播(節);
        let m = (lo + hi) / 2;
        self.和内部(2 * 節, lo, m, l, r) + self.和内部(2 * 節 + 1, m, hi, l, r)
    }

    /// 区間（`a..b` / `a..=b` / `..` など）の最大値。ならし `O(log N)`。
    fn 区間最大(&mut self, 範囲: impl std::ops::RangeBounds<usize>) -> isize {
        let (l, r) = 半開(範囲, self.n);
        self.最大内部(1, 0, self.n, l, r)
    }

    fn 最大内部(&mut self, 節: usize, lo: usize, hi: usize, l: usize, r: usize) -> isize {
        if r <= lo || hi <= l {
            return -無限大;
        }
        if l <= lo && hi <= r {
            return self.木[節].最大1;
        }
        self.伝播(節);
        let m = (lo + hi) / 2;
        self.最大内部(2 * 節, lo, m, l, r)
            .max(self.最大内部(2 * 節 + 1, m, hi, l, r))
    }

    /// 区間（`a..b` / `a..=b` / `..` など）の最小値。ならし `O(log N)`。
    fn 区間最小(&mut self, 範囲: impl std::ops::RangeBounds<usize>) -> isize {
        let (l, r) = 半開(範囲, self.n);
        self.最小内部(1, 0, self.n, l, r)
    }

    fn 最小内部(&mut self, 節: usize, lo: usize, hi: usize, l: usize, r: usize) -> isize {
        if r <= lo || hi <= l {
            return 無限大;
        }
        if l <= lo && hi <= r {
            return self.木[節].最小1;
        }
        self.伝播(節);
        let m = (lo + hi) / 2;
        self.最小内部(2 * 節, lo, m, l, r)
            .min(self.最小内部(2 * 節 + 1, m, hi, l, r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 基本動作() {
        let mut t = セグ木ビーツ::new(&[1, 5, 3, 2, 4]);
        assert_eq!(t.区間和(..), 15); // 全体
        assert_eq!(t.区間最大(0..5), 5);
        assert_eq!(t.区間最小(0..=4), 1); // 閉区間でも可
        // [1,5,3,2,4] の [0,3) を chmin 3 → [1,3,3,2,4]
        t.区間chmin(0..3, 3);
        assert_eq!(t.区間和(0..5), 13);
        assert_eq!(t.区間最大(0..3), 3);
        // [1,3,3,2,4] の [2,5) を chmax 3 → [1,3,3,3,4]
        t.区間chmax(2..5, 3);
        assert_eq!(t.区間最小(2..5), 3);
        assert_eq!(t.区間和(0..5), 14);
    }

    #[test]
    fn 素朴な配列と一致() {
        let mut seed = 0x55aa_1234u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        let n = 40usize;
        let mut 配列: Vec<isize> = (0..n).map(|_| (乱数() % 200) as isize - 100).collect();
        let mut t = セグ木ビーツ::new(&配列);
        for _ in 0..3000 {
            let l = (乱数() as usize) % n;
            let r = l + 1 + (乱数() as usize) % (n - l);
            match 乱数() % 5 {
                0 => {
                    let x = (乱数() % 200) as isize - 100;
                    t.区間chmin(l..r, x);
                    for v in &mut 配列[l..r] {
                        *v = (*v).min(x);
                    }
                }
                1 => {
                    let x = (乱数() % 200) as isize - 100;
                    t.区間chmax(l..r, x);
                    for v in &mut 配列[l..r] {
                        *v = (*v).max(x);
                    }
                }
                2 => {
                    let 期待: isize = 配列[l..r].iter().sum();
                    assert_eq!(t.区間和(l..r), 期待, "和 [{l},{r})");
                }
                3 => {
                    let 期待 = *配列[l..r].iter().max().unwrap();
                    assert_eq!(t.区間最大(l..r), 期待, "最大 [{l},{r})");
                }
                _ => {
                    let 期待 = *配列[l..r].iter().min().unwrap();
                    assert_eq!(t.区間最小(l..r), 期待, "最小 [{l},{r})");
                }
            }
        }
    }
}
