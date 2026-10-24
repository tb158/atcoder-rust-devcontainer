use cargo_snippet::snippet;

#[snippet("重み付き_UnionFind")]
#[snippet("重み付き_UF")]
#[snippet("WeightedUnionFind")]
#[snippet("ポテンシャル")]
/// 重み付き Union-Find（ポテンシャル付き DSU）。
///
/// 各要素に「根に対する相対的な値（ポテンシャル）」を持たせ、
/// 同じ連結成分に属する2要素 `x`, `y` の差 `weight(y) - weight(x)` を問い合わせできる。
/// 「A は B より d 大きい」といった差分制約の整合性判定・差の計算に使う。
///
/// 重みの型 `T` は可換群（加法・減算ができ、`T::default()` が単位元 0）であれば
/// 何でもよい（`isize` のほか、`mod` 付き整数の newtype などにも使える）。
struct 重み付きUnionFind<T> {
    親: Vec<usize>,
    rank: Vec<usize>,
    /// 親に対する相対重み
    重み: Vec<T>,
}

#[snippet("重み付き_UnionFind")]
#[snippet("重み付き_UF")]
#[snippet("WeightedUnionFind")]
#[snippet("ポテンシャル")]
impl<T> 重み付きUnionFind<T>
where
    T: Copy
        + Default
        + PartialEq
        + std::ops::AddAssign
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>,
{
    /// `n` 要素（各自が独立、重み 0 = `T::default()`）で初期化する。
    fn new(n: usize) -> Self {
        Self {
            親: (0..n).collect(),
            rank: vec![0; n],
            重み: vec![T::default(); n],
        }
    }

    /// `x` の根を返す（経路圧縮しつつ、`重み` を根からの相対値へ更新する）。
    fn 根(&mut self, x: usize) -> usize {
        if self.親[x] == x {
            return x;
        }
        let p = self.親[x];
        let r = self.根(p);
        // 親 `p` は再帰で根からの相対値に更新済み。その分を足して自分も根基準にする。
        let p重み = self.重み[p];
        self.重み[x] += p重み;
        self.親[x] = r;
        r
    }

    /// `x` の、自身が属する根に対する相対重み（ポテンシャル）。
    fn ポテンシャル(&mut self, x: usize) -> T {
        self.根(x);
        self.重み[x]
    }

    /// `weight(y) - weight(x) == 差` となるよう `x` と `y` を併合する。
    ///
    /// 戻り値:
    /// - 既に別集合 → 併合して `true`
    /// - 既に同集合で、その差が `差` と一致 → 何もせず `true`（無矛盾）
    /// - 既に同集合だが差が矛盾 → 何もせず `false`
    fn 併合(&mut self, x: usize, y: usize, 差: T) -> bool {
        let wx = self.ポテンシャル(x);
        let wy = self.ポテンシャル(y);
        let rx = self.根(x);
        let ry = self.根(y);
        if rx == ry {
            return wy - wx == 差;
        }
        if self.rank[rx] >= self.rank[ry] {
            // ry を rx の子にする: (wy + W) - wx == 差  =>  W = 差 + wx - wy
            self.重み[ry] = 差 + wx - wy;
            self.親[ry] = rx;
            if self.rank[rx] == self.rank[ry] {
                self.rank[rx] += 1;
            }
        } else {
            // rx を ry の子にする: wy - (wx + W) == 差  =>  W = wy - wx - 差
            self.重み[rx] = wy - wx - 差;
            self.親[rx] = ry;
        }
        true
    }

    /// `weight(y) - weight(x)` を返す。`x` と `y` が同じ集合でなければ `None`。
    fn 差(&mut self, x: usize, y: usize) -> Option<T> {
        if self.根(x) != self.根(y) {
            None
        } else {
            Some(self.ポテンシャル(y) - self.ポテンシャル(x))
        }
    }

    /// `x` と `y` が同じ連結成分にあるか。
    fn 同じ集合(&mut self, x: usize, y: usize) -> bool {
        self.根(x) == self.根(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 差分制約の整合性() {
        let mut uf = 重み付きUnionFind::<isize>::new(5);
        assert!(uf.併合(0, 1, 3)); // w(1)-w(0)=3
        assert!(uf.併合(1, 2, 2)); // w(2)-w(1)=2
        assert_eq!(uf.差(0, 2), Some(5)); // w(2)-w(0)=5
        assert!(uf.同じ集合(0, 2));
        assert_eq!(uf.差(0, 4), None); // 別集合

        // 既知の関係と一致する併合は true
        assert!(uf.併合(0, 2, 5));
        // 矛盾する併合は false（状態は壊さない）
        assert!(!uf.併合(0, 2, 4));
        assert_eq!(uf.差(0, 2), Some(5));
    }
}
