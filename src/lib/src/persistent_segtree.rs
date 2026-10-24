use crate::range_bounds::半開;
use cargo_snippet::snippet;

#[snippet("永続_セグ木", include = "半開_区間")]
#[snippet("永続_セグメント_木")]
#[snippet("PersistentSegmentTree")]
#[snippet("版管理_セグ木")]
/// 永続セグメント木（区間和・一点加算）。各更新は元の版を壊さず新しい根を返すため、
/// 任意の過去の版に対してクエリできる。更新・クエリともに O(log N)、版ごとに O(log N) のノード。
///
/// 配列を 1 要素ずつ更新した版列を作れば「区間第 k 小」等の典型に使える
/// （単純な静的列の区間第 k 小なら [`crate::wavelet_matrix`] が手軽）。
///
/// 要素型 `T` は加算できる型なら何でもよく（`isize`・`usize`・`ModInt` など）、
/// `T::default()` が加法単位元（0）であることを前提とする。
struct 永続セグ木<T> {
    n: usize,
    和: Vec<T>,
    左: Vec<usize>,
    右: Vec<usize>,
}

#[snippet("永続_セグ木")]
#[snippet("永続_セグメント_木")]
#[snippet("PersistentSegmentTree")]
#[snippet("版管理_セグ木")]
impl<T> 永続セグ木<T>
where
    T: Copy + Default + std::ops::Add<Output = T>,
{
    /// サイズ `n` の全 0 の木を作り、`(構造体, 初期版の根)` を返す。
    fn new(n: usize) -> (Self, usize) {
        let mut t = Self {
            n,
            和: vec![T::default()],
            左: vec![0],
            右: vec![0],
        };
        let 根 = t.構築(0, n.max(1));
        (t, 根)
    }

    fn 節点追加(&mut self, 和: T, 左: usize, 右: usize) -> usize {
        let id = self.和.len();
        self.和.push(和);
        self.左.push(左);
        self.右.push(右);
        id
    }

    fn 構築(&mut self, lo: usize, hi: usize) -> usize {
        if hi - lo <= 1 {
            return self.節点追加(T::default(), 0, 0);
        }
        let m = (lo + hi) / 2;
        let l = self.構築(lo, m);
        let r = self.構築(m, hi);
        self.節点追加(T::default(), l, r)
    }

    /// 版 `根` の位置 `pos` に `値` を加え、新しい版の根を返す。元の版は不変。
    fn 一点加算(&mut self, 根: usize, pos: usize, 値: T) -> usize {
        let n = self.n.max(1);
        self.更新(根, 0, n, pos, 値)
    }

    fn 更新(&mut self, 節: usize, lo: usize, hi: usize, pos: usize, 値: T) -> usize {
        if hi - lo <= 1 {
            return self.節点追加(self.和[節] + 値, 0, 0);
        }
        let m = (lo + hi) / 2;
        if pos < m {
            let nl = self.更新(self.左[節], lo, m, pos, 値);
            self.節点追加(self.和[節] + 値, nl, self.右[節])
        } else {
            let nr = self.更新(self.右[節], m, hi, pos, 値);
            self.節点追加(self.和[節] + 値, self.左[節], nr)
        }
    }

    /// 版 `根` における区間（`a..b` / `a..=b` / `..` など）の和。
    fn 区間和(&self, 根: usize, 範囲: impl std::ops::RangeBounds<usize>) -> T {
        let (l, r) = 半開(範囲, self.n);
        self.和取得(根, 0, self.n.max(1), l, r)
    }

    fn 和取得(&self, 節: usize, lo: usize, hi: usize, l: usize, r: usize) -> T {
        if r <= lo || hi <= l {
            return T::default();
        }
        if l <= lo && hi <= r {
            return self.和[節];
        }
        let m = (lo + hi) / 2;
        self.和取得(self.左[節], lo, m, l, r) + self.和取得(self.右[節], m, hi, l, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 版ごとに独立して和が取れる() {
        let n = 8;
        let (mut t, 版0) = 永続セグ木::new(n);
        // 版1: pos2 に +5
        let 版1 = t.一点加算(版0, 2, 5);
        // 版2: 版1 から pos5 に +3
        let 版2 = t.一点加算(版1, 5, 3);
        // 版3: 版2 から pos2 に +1
        let 版3 = t.一点加算(版2, 2, 1);

        // 版0 は全 0 のまま
        assert_eq!(t.区間和(版0, 0..n), 0);
        // 版1: pos2=5
        assert_eq!(t.区間和(版1, 0..n), 5);
        assert_eq!(t.区間和(版1, 0..2), 0);
        assert_eq!(t.区間和(版1, 2..3), 5);
        // 版2: pos2=5, pos5=3
        assert_eq!(t.区間和(版2, 0..n), 8);
        assert_eq!(t.区間和(版2, 3..6), 3);
        // 版3: pos2=6, pos5=3
        assert_eq!(t.区間和(版3, 0..n), 9);
        assert_eq!(t.区間和(版3, 2..3), 6);
        // 過去版が壊れていない
        assert_eq!(t.区間和(版1, 0..n), 5);
        assert_eq!(t.区間和(版2, 0..n), 8);
    }

    #[test]
    fn 素朴な版列と一致() {
        let n = 16;
        let (mut t, 初版) = 永続セグ木::new(n);
        let mut 版 = vec![初版];
        let mut 配列群: Vec<Vec<isize>> = vec![vec![0isize; n]];
        // 線形合同法で更新列を作る
        let mut seed = 98765u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..30 {
            let pos = (乱数() as usize) % n;
            let 値 = (乱数() % 20) as isize - 10;
            let 新根 = t.一点加算(*版.last().unwrap(), pos, 値);
            let mut 新配列 = 配列群.last().unwrap().clone();
            新配列[pos] += 値;
            版.push(新根);
            配列群.push(新配列);
        }
        // 全版・全区間を突き合わせ
        for (v, 配列) in 版.iter().zip(&配列群) {
            for l in 0..n {
                for r in l..=n {
                    let 期待: isize = 配列[l..r].iter().sum();
                    assert_eq!(t.区間和(*v, l..r), 期待);
                }
            }
        }
    }
}
