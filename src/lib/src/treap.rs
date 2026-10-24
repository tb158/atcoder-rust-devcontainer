use crate::range_bounds::半開;
use cargo_snippet::snippet;

#[snippet("陰的_Treap", include = "半開_区間")]
#[snippet("ImplicitTreap")]
#[snippet("Treap")]
#[snippet("区間_反転")]
#[snippet("挿入_削除_可能_配列")]
/// 陰的 Treap（implicit treap）。配列を平衡二分木で表し、任意位置への挿入・削除、
/// 区間反転、区間和、一点取得を全て O(log N) で行える。添字をキーに持たず部分木サイズで
/// 位置を決める「陰的キー」方式。
///
/// - [`Self::挿入`] / [`Self::削除`] : 位置指定の挿入・削除。
/// - [`Self::反転`] : 半開区間 `[l, r)` を反転。
/// - [`Self::区間和`] : 半開区間 `[l, r)` の和。
/// - [`Self::取得`] : 1 要素の値。
///
/// 要素型 `T` は加算できる型なら何でもよく（`isize`・`usize`・`ModInt` など）、
/// `T::default()` が加法単位元（0）であることを前提とする。
///
/// 位置ではなく値の大小で並べた集合（k 番目・順位）が欲しいなら
/// [`ソート済み多重集合`](crate::sorted_multiset::ソート済み多重集合) を使う。
struct 陰的Treap<T> {
    値: Vec<T>,
    優先度: Vec<u64>,
    サイズ: Vec<usize>,
    和: Vec<T>,
    反転: Vec<bool>,
    左: Vec<usize>,
    右: Vec<usize>,
    rng: u64,
    根: usize,
}

#[snippet("陰的_Treap")]
#[snippet("ImplicitTreap")]
#[snippet("Treap")]
#[snippet("区間_反転")]
#[snippet("挿入_削除_可能_配列")]
impl<T> 陰的Treap<T>
where
    T: Copy + Default + std::ops::Add<Output = T>,
{
    fn new() -> Self {
        // 添字 0 を番兵ノードにする
        Self {
            値: vec![T::default()],
            優先度: vec![0],
            サイズ: vec![0],
            和: vec![T::default()],
            反転: vec![false],
            左: vec![0],
            右: vec![0],
            rng: 0x9e37_79b9_7f4a_7c15,
            根: 0,
        }
    }

    fn 乱数(&mut self) -> u64 {
        // xorshift64
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng = x;
        x
    }

    fn 新規ノード(&mut self, 値: T) -> usize {
        let p = self.乱数();
        self.値.push(値);
        self.優先度.push(p);
        self.サイズ.push(1);
        self.和.push(値);
        self.反転.push(false);
        self.左.push(0);
        self.右.push(0);
        self.値.len() - 1
    }

    /// 子の情報から `t` のサイズと和を再計算する。反転フラグはサイズ・和に影響しない。
    fn 更新(&mut self, t: usize) {
        if t == 0 {
            return;
        }
        let l = self.左[t];
        let r = self.右[t];
        self.サイズ[t] = 1 + self.サイズ[l] + self.サイズ[r];
        self.和[t] = self.値[t] + self.和[l] + self.和[r];
    }

    /// `t` に溜まった反転遅延を子へ押し下げる。
    fn 伝播(&mut self, t: usize) {
        if t == 0 || !self.反転[t] {
            return;
        }
        self.反転[t] = false;
        let l = self.左[t];
        let r = self.右[t];
        self.左[t] = r;
        self.右[t] = l;
        if l != 0 {
            self.反転[l] ^= true;
        }
        if r != 0 {
            self.反転[r] ^= true;
        }
    }

    /// `t` を「左 k 要素」と「残り」に分割し、`(左根, 右根)` を返す。
    fn split(&mut self, t: usize, k: usize) -> (usize, usize) {
        if t == 0 {
            return (0, 0);
        }
        self.伝播(t);
        let 左サイズ = self.サイズ[self.左[t]];
        if k <= 左サイズ {
            let (l, r) = self.split(self.左[t], k);
            self.左[t] = r;
            self.更新(t);
            (l, t)
        } else {
            let (l, r) = self.split(self.右[t], k - 左サイズ - 1);
            self.右[t] = l;
            self.更新(t);
            (t, r)
        }
    }

    /// 2 つの木を連結する。`l` の全要素が `r` の前に来る。
    fn merge(&mut self, l: usize, r: usize) -> usize {
        if l == 0 {
            return r;
        }
        if r == 0 {
            return l;
        }
        if self.優先度[l] > self.優先度[r] {
            self.伝播(l);
            let 新右 = self.merge(self.右[l], r);
            self.右[l] = 新右;
            self.更新(l);
            l
        } else {
            self.伝播(r);
            let 新左 = self.merge(l, self.左[r]);
            self.左[r] = 新左;
            self.更新(r);
            r
        }
    }

    /// 末尾に値を追加する。
    fn 末尾追加(&mut self, 値: T) {
        let 新 = self.新規ノード(値);
        let 根 = self.根;
        self.根 = self.merge(根, 新);
    }

    /// 位置 `pos`（0-indexed、`pos == 長さ` で末尾）に `値` を挿入する。
    fn 挿入(&mut self, pos: usize, 値: T) {
        let 新 = self.新規ノード(値);
        let 根 = self.根;
        let (l, r) = self.split(根, pos);
        let 左 = self.merge(l, 新);
        self.根 = self.merge(左, r);
    }

    /// 位置 `pos`（0-indexed）の要素を削除する。
    fn 削除(&mut self, pos: usize) {
        let 根 = self.根;
        let (l, r) = self.split(根, pos);
        let (_削除, r2) = self.split(r, 1);
        self.根 = self.merge(l, r2);
    }

    /// 区間（`a..b` / `a..=b` / `..` など）を反転する。
    fn 反転(&mut self, 範囲: impl std::ops::RangeBounds<usize>) {
        let (l, r) = 半開(範囲, self.長さ());
        if l >= r {
            return;
        }
        let 根 = self.根;
        let (a, bc) = self.split(根, l);
        let (b, c) = self.split(bc, r - l);
        if b != 0 {
            self.反転[b] ^= true;
        }
        let 左 = self.merge(a, b);
        self.根 = self.merge(左, c);
    }

    /// 区間（`a..b` / `a..=b` / `..` など）の和。
    fn 区間和(&mut self, 範囲: impl std::ops::RangeBounds<usize>) -> T {
        let (l, r) = 半開(範囲, self.長さ());
        if l >= r {
            return T::default();
        }
        let 根 = self.根;
        let (a, bc) = self.split(根, l);
        let (b, c) = self.split(bc, r - l);
        let 結果 = self.和[b];
        let 左 = self.merge(a, b);
        self.根 = self.merge(左, c);
        結果
    }

    /// 位置 `pos`（0-indexed）の値。
    fn 取得(&mut self, pos: usize) -> T {
        self.区間和(pos..pos + 1)
    }

    /// 要素数。
    fn 長さ(&self) -> usize {
        self.サイズ[self.根]
    }
}

#[snippet("陰的_Treap")]
#[snippet("ImplicitTreap")]
#[snippet("Treap")]
#[snippet("区間_反転")]
#[snippet("挿入_削除_可能_配列")]
impl<T> Default for 陰的Treap<T>
where
    T: Copy + Default + std::ops::Add<Output = T>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 末尾追加と取得() {
        let mut t = 陰的Treap::new();
        for v in [10isize, 20, 30, 40] {
            t.末尾追加(v);
        }
        assert_eq!(t.長さ(), 4);
        assert_eq!(t.取得(0), 10);
        assert_eq!(t.取得(3), 40);
        assert_eq!(t.区間和(0..4), 100);
        assert_eq!(t.区間和(1..3), 50);
    }

    #[test]
    fn 反転が効く() {
        let mut t = 陰的Treap::new();
        for v in [1isize, 2, 3, 4, 5] {
            t.末尾追加(v);
        }
        // [1,2,3,4,5] の [1,4) を反転 → [1,4,3,2,5]
        t.反転(1..4);
        assert_eq!(t.取得(0), 1);
        assert_eq!(t.取得(1), 4);
        assert_eq!(t.取得(2), 3);
        assert_eq!(t.取得(3), 2);
        assert_eq!(t.取得(4), 5);
        // 和は反転で不変
        assert_eq!(t.区間和(0..5), 15);
    }

    #[test]
    fn 素朴な配列と一致() {
        // 線形合同法でランダムな操作列を作り、Vec での素朴実装と突き合わせる
        let mut seed = 0xdead_beefu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        let mut t = 陰的Treap::new();
        let mut 配列: Vec<isize> = Vec::new();
        for _ in 0..2000 {
            let n = 配列.len();
            let 操作 = if n == 0 { 0 } else { 乱数() % 6 };
            match 操作 {
                0 => {
                    // 挿入
                    let pos = (乱数() as usize) % (n + 1);
                    let 値 = (乱数() % 100) as isize;
                    t.挿入(pos, 値);
                    配列.insert(pos, 値);
                }
                1 => {
                    // 削除
                    let pos = (乱数() as usize) % n;
                    t.削除(pos);
                    配列.remove(pos);
                }
                2 => {
                    // 反転
                    let l = (乱数() as usize) % n;
                    let r = l + 1 + (乱数() as usize) % (n - l);
                    t.反転(l..r);
                    配列[l..r].reverse();
                }
                3 => {
                    // 区間和
                    let l = (乱数() as usize) % n;
                    let r = l + 1 + (乱数() as usize) % (n - l);
                    let 期待: isize = 配列[l..r].iter().sum();
                    assert_eq!(t.区間和(l..r), 期待);
                }
                4 => {
                    // 一点取得
                    let pos = (乱数() as usize) % n;
                    assert_eq!(t.取得(pos), 配列[pos]);
                }
                _ => {
                    // 末尾追加
                    let 値 = (乱数() % 100) as isize;
                    t.末尾追加(値);
                    配列.push(値);
                }
            }
            assert_eq!(t.長さ(), 配列.len());
        }
        // 最終的に全要素一致
        for (i, &v) in 配列.iter().enumerate() {
            assert_eq!(t.取得(i), v, "位置 {i}");
        }
    }

    #[test]
    fn usize_でも使える() {
        let mut t: 陰的Treap<usize> = 陰的Treap::new();
        for v in [10usize, 20, 30, 40] {
            t.末尾追加(v);
        }
        assert_eq!(t.長さ(), 4);
        assert_eq!(t.取得(0), 10);
        assert_eq!(t.区間和(0..4), 100);
    }

    #[test]
    fn modint_でも使える() {
        use ac_library::ModInt998244353 as M;
        let mut t: 陰的Treap<M> = 陰的Treap::new();
        for v in [1, 2, 3, 4, 5] {
            t.末尾追加(M::new(v));
        }
        assert_eq!(t.長さ(), 5);
        assert_eq!(t.区間和(0..5), M::new(15));
        t.反転(1..4);
        assert_eq!(t.取得(1), M::new(4));
        assert_eq!(t.区間和(0..5), M::new(15));
    }
}
