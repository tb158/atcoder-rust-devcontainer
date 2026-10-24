use cargo_snippet::snippet;

#[snippet("リンク_カット_木")]
#[snippet("LinkCutTree")]
#[snippet("動的_木")]
#[snippet("パス_和_動的")]
#[snippet("LCT")]
/// Link-Cut Tree。森に対する辺の追加・削除（[`Self::辺を張る`] / [`Self::辺を切る`]）、
/// 連結判定、2 頂点間パスの頂点値の総和を、すべてならし O(log N) で処理する。
/// 頂点番号は 0-indexed。
///
/// - [`Self::辺を張る`] : 別の木に属する 2 頂点を結ぶ（同一木の頂点に呼んではならない）。
/// - [`Self::辺を切る`] : 既存の辺を取り除く（隣接していない 2 頂点に呼んではならない）。
/// - [`Self::連結`] : 2 頂点が同じ木か。
/// - [`Self::パス和`] : 2 頂点間パス上の頂点値の総和。
/// - [`Self::値設定`] : 1 頂点の値を更新。
///
/// 要素型 `T` は加算できる型なら何でもよく（`isize`・`usize`・`ModInt` など）、
/// `T::default()` が加法単位元（0）であることを前提とする。
struct リンクカット木<T> {
    親: Vec<usize>,
    子: Vec<[usize; 2]>,
    値: Vec<T>,
    和: Vec<T>,
    反転: Vec<bool>,
}

#[snippet("リンク_カット_木")]
#[snippet("LinkCutTree")]
#[snippet("動的_木")]
#[snippet("パス_和_動的")]
#[snippet("LCT")]
impl<T> リンクカット木<T>
where
    T: Copy + Default + std::ops::Add<Output = T>,
{
    /// 各頂点の初期値から構築する。
    fn new(初期値: &[T]) -> Self {
        let n = 初期値.len();
        let mut s = Self {
            親: vec![0; n + 1],
            子: vec![[0, 0]; n + 1],
            値: vec![T::default(); n + 1],
            和: vec![T::default(); n + 1],
            反転: vec![false; n + 1],
        };
        for (i, &v) in 初期値.iter().enumerate() {
            s.値[i + 1] = v;
            s.和[i + 1] = v;
        }
        s
    }

    /// `x` がスプレー木の根（パス親へのリンクしか持たない）か。
    fn is_root(&self, x: usize) -> bool {
        let p = self.親[x];
        p == 0 || (self.子[p][0] != x && self.子[p][1] != x)
    }

    fn 更新(&mut self, x: usize) {
        if x == 0 {
            return;
        }
        self.和[x] = self.和[self.子[x][0]] + self.値[x] + self.和[self.子[x][1]];
    }

    fn 伝播(&mut self, x: usize) {
        if x != 0 && self.反転[x] {
            let l = self.子[x][0];
            let r = self.子[x][1];
            self.子[x][0] = r;
            self.子[x][1] = l;
            if l != 0 {
                self.反転[l] ^= true;
            }
            if r != 0 {
                self.反転[r] ^= true;
            }
            self.反転[x] = false;
        }
    }

    fn 回転(&mut self, x: usize) {
        let p = self.親[x];
        let g = self.親[p];
        let pがルート = self.is_root(p);
        let d = if self.子[p][1] == x { 1 } else { 0 };
        let b = self.子[x][d ^ 1];
        self.子[p][d] = b;
        if b != 0 {
            self.親[b] = p;
        }
        self.子[x][d ^ 1] = p;
        self.親[p] = x;
        self.親[x] = g;
        if !pがルート {
            if self.子[g][0] == p {
                self.子[g][0] = x;
            } else if self.子[g][1] == p {
                self.子[g][1] = x;
            }
        }
        self.更新(p);
        self.更新(x);
    }

    fn splay(&mut self, x: usize) {
        // 根から x までの遅延を先に解消する
        let mut スタック = vec![x];
        let mut y = x;
        while !self.is_root(y) {
            y = self.親[y];
            スタック.push(y);
        }
        while let Some(v) = スタック.pop() {
            self.伝播(v);
        }
        while !self.is_root(x) {
            let p = self.親[x];
            if self.is_root(p) {
                self.回転(x);
            } else {
                let g = self.親[p];
                let 同方向 = (self.子[g][1] == p) == (self.子[p][1] == x);
                if 同方向 {
                    self.回転(p);
                } else {
                    self.回転(x);
                }
                self.回転(x);
            }
        }
    }

    fn access(&mut self, x: usize) -> usize {
        let mut last = 0;
        let mut cur = x;
        while cur != 0 {
            self.splay(cur);
            self.子[cur][1] = last;
            self.更新(cur);
            last = cur;
            cur = self.親[cur];
        }
        self.splay(x);
        last
    }

    fn make_root(&mut self, x: usize) {
        self.access(x);
        self.反転[x] ^= true;
        self.伝播(x);
    }

    fn find_root(&mut self, x: usize) -> usize {
        self.access(x);
        let mut r = x;
        self.伝播(r);
        while self.子[r][0] != 0 {
            r = self.子[r][0];
            self.伝播(r);
        }
        self.splay(r);
        r
    }

    /// 別の木に属する頂点 `u`・`v` を結ぶ辺を張る。同じ木の 2 頂点に呼んではならない。
    fn 辺を張る(&mut self, u: usize, v: usize) {
        let (a, b) = (u + 1, v + 1);
        self.make_root(a);
        self.親[a] = b;
    }

    /// 既存の辺 `u - v` を取り除く。隣接していない 2 頂点に呼んではならない。
    fn 辺を切る(&mut self, u: usize, v: usize) {
        let (a, b) = (u + 1, v + 1);
        self.make_root(a);
        self.access(b);
        // a は b の左の子（直前の頂点）になっている
        self.子[b][0] = 0;
        self.親[a] = 0;
        self.更新(b);
    }

    /// 頂点 `u`・`v` が同じ木に属するか。
    fn 連結(&mut self, u: usize, v: usize) -> bool {
        if u == v {
            return true;
        }
        self.find_root(u + 1) == self.find_root(v + 1)
    }

    /// 任意頂点 `u` をその木の根にする。
    fn 根を変更(&mut self, u: usize) {
        self.make_root(u + 1);
    }

    /// 頂点 `u`・`v` 間のパス上にある頂点値の総和。同じ木に属することが前提。
    fn パス和(&mut self, u: usize, v: usize) -> T {
        let (a, b) = (u + 1, v + 1);
        self.make_root(a);
        self.access(b);
        self.和[b]
    }

    /// 頂点 `u` の値を `新値` に設定する。
    fn 値設定(&mut self, u: usize, 新値: T) {
        let a = u + 1;
        self.access(a);
        self.値[a] = 新値;
        self.更新(a);
    }

    /// 頂点 `u` の現在の値。
    fn 頂点値(&self, u: usize) -> T {
        self.値[u + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    // 素朴な森：辺集合と頂点値を保持し、連結性・パス和を BFS で求める
    struct 素朴森 {
        n: usize,
        値: Vec<isize>,
        隣接: Vec<Vec<usize>>,
    }
    impl 素朴森 {
        fn new(値: Vec<isize>) -> Self {
            let n = 値.len();
            Self {
                n,
                値,
                隣接: vec![Vec::new(); n],
            }
        }
        fn 連結(&self, u: usize, v: usize) -> bool {
            self.経路(u, v).is_some()
        }
        // u から v への頂点列（木なので一意）
        fn 経路(&self, u: usize, v: usize) -> Option<Vec<usize>> {
            let mut 親 = vec![usize::MAX; self.n];
            let mut 訪問 = vec![false; self.n];
            let mut q = VecDeque::new();
            q.push_back(u);
            訪問[u] = true;
            while let Some(x) = q.pop_front() {
                if x == v {
                    let mut 経路 = vec![v];
                    let mut c = v;
                    while c != u {
                        c = 親[c];
                        経路.push(c);
                    }
                    return Some(経路);
                }
                for &y in &self.隣接[x] {
                    if !訪問[y] {
                        訪問[y] = true;
                        親[y] = x;
                        q.push_back(y);
                    }
                }
            }
            None
        }
        fn パス和(&self, u: usize, v: usize) -> isize {
            self.経路(u, v).unwrap().iter().map(|&x| self.値[x]).sum()
        }
        fn 辺を張る(&mut self, u: usize, v: usize) {
            self.隣接[u].push(v);
            self.隣接[v].push(u);
        }
        fn 辺を切る(&mut self, u: usize, v: usize) {
            self.隣接[u].retain(|&x| x != v);
            self.隣接[v].retain(|&x| x != u);
        }
    }

    #[test]
    fn 連結とパス和の基本() {
        let mut lct = リンクカット木::new(&[1, 2, 3, 4]);
        assert!(!lct.連結(0, 3));
        lct.辺を張る(0, 1);
        lct.辺を張る(1, 2);
        lct.辺を張る(2, 3);
        assert!(lct.連結(0, 3));
        // パス 0-1-2-3 の和 = 1+2+3+4 = 10
        assert_eq!(lct.パス和(0, 3), 10);
        // 値を更新
        lct.値設定(2, 30);
        assert_eq!(lct.パス和(0, 3), 1 + 2 + 30 + 4);
        // 辺を切ると非連結
        lct.辺を切る(1, 2);
        assert!(!lct.連結(0, 3));
        assert_eq!(lct.パス和(0, 1), 1 + 2);
    }

    #[test]
    fn 乱数で素朴森と一致() {
        let mut seed = 0xcafe_f00du64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        let n = 12usize;
        let 初期値: Vec<isize> = (0..n).map(|_| (乱数() % 100) as isize).collect();
        let mut lct = リンクカット木::new(&初期値);
        let mut 森 = 素朴森::new(初期値);
        // 現在張られている辺
        let mut 辺集合: Vec<(usize, usize)> = Vec::new();
        for _ in 0..4000 {
            let u = (乱数() as usize) % n;
            let v = (乱数() as usize) % n;
            match 乱数() % 5 {
                0 => {
                    // 連結でなければ辺を張る
                    if u != v && !森.連結(u, v) {
                        lct.辺を張る(u, v);
                        森.辺を張る(u, v);
                        辺集合.push((u, v));
                    }
                }
                1 => {
                    // 既存の辺を 1 本切る
                    if !辺集合.is_empty() {
                        let i = (乱数() as usize) % 辺集合.len();
                        let (a, b) = 辺集合.swap_remove(i);
                        lct.辺を切る(a, b);
                        森.辺を切る(a, b);
                    }
                }
                2 => {
                    let 新値 = (乱数() % 100) as isize;
                    lct.値設定(u, 新値);
                    森.値[u] = 新値;
                }
                3 => {
                    assert_eq!(lct.連結(u, v), 森.連結(u, v), "連結 {u},{v}");
                }
                _ => {
                    if 森.連結(u, v) {
                        assert_eq!(lct.パス和(u, v), 森.パス和(u, v), "パス和 {u},{v}");
                    }
                }
            }
        }
    }
}
