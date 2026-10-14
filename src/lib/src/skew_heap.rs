use cargo_snippet::snippet;

#[snippet("スキュー_ヒープ")]
#[snippet("SkewHeap")]
#[snippet("マージ_可能_ヒープ")]
#[snippet("併合_可能_ヒープ")]
#[snippet("meldable_ヒープ")]
/// スキューヒープの節点（最小ヒープ）。値と左右の子を持つ自己平衡しない二分ヒープ。
struct 節点<T> {
    値: T,
    左: Option<Box<節点<T>>>,
    右: Option<Box<節点<T>>>,
}

#[snippet("スキュー_ヒープ")]
#[snippet("SkewHeap")]
#[snippet("マージ_可能_ヒープ")]
#[snippet("併合_可能_ヒープ")]
#[snippet("meldable_ヒープ")]
/// 2 つの部分木を併合する内部関数。
fn 併合節点<T: Ord>(
    a: Option<Box<節点<T>>>,
    b: Option<Box<節点<T>>>,
) -> Option<Box<節点<T>>> {
    match (a, b) {
        (None, x) | (x, None) => x,
        (Some(mut na), Some(mut nb)) => {
            if na.値 > nb.値 {
                std::mem::swap(&mut na, &mut nb); // na が小さい方の根
            }
            let 右 = na.右.take();
            na.右 = 併合節点(右, Some(nb));
            std::mem::swap(&mut na.左, &mut na.右); // skew: 子を入れ替え
            Some(na)
        }
    }
}

#[snippet("スキュー_ヒープ")]
#[snippet("SkewHeap")]
#[snippet("マージ_可能_ヒープ")]
#[snippet("併合_可能_ヒープ")]
#[snippet("meldable_ヒープ")]
/// 2 つのヒープを `O(log n)` で併合（meld）できる最小ヒープ（スキューヒープ）。
///
/// 標準の `BinaryHeap` は 2 つのヒープの併合に `O(n)` かかるが、本実装は併合・挿入・
/// 最小取り出しすべて償却 `O(log n)`。最小有向全域木や Dijkstra 風のヒープ統合で使う。
#[derive(Default)]
struct スキューヒープ<T: Ord> {
    根: Option<Box<節点<T>>>,
    個数: usize,
}

#[snippet("スキュー_ヒープ")]
#[snippet("SkewHeap")]
#[snippet("マージ_可能_ヒープ")]
#[snippet("併合_可能_ヒープ")]
#[snippet("meldable_ヒープ")]
impl<T: Ord> スキューヒープ<T> {
    /// 空のヒープを作る。
    fn new() -> Self {
        Self {
            根: None, 個数: 0
        }
    }

    /// 要素数を返す。
    fn 個数(&self) -> usize {
        self.個数
    }

    /// 空かどうかを返す。
    fn 空か(&self) -> bool {
        self.個数 == 0
    }

    /// 値 `v` を挿入する。償却 `O(log n)`。
    fn 挿入(&mut self, v: T) {
        let 節 = Some(Box::new(節点 {
            値: v,
            左: None,
            右: None,
        }));
        self.根 = 併合節点(self.根.take(), 節);
        self.個数 += 1;
    }

    /// 最小値への参照を返す（空なら `None`）。`O(1)`。
    fn 最小参照(&self) -> Option<&T> {
        self.根.as_ref().map(|n| &n.値)
    }

    /// 最小値を取り出して返す（空なら `None`）。償却 `O(log n)`。
    fn 最小取り出し(&mut self) -> Option<T> {
        let 根 = self.根.take()?;
        self.個数 -= 1;
        self.根 = 併合節点(根.左, 根.右);
        Some(根.値)
    }

    /// 別のヒープ `other` を自分に併合する。`other` は空になる。償却 `O(log n)`。
    fn 併合(&mut self, other: &mut Self) {
        self.根 = 併合節点(self.根.take(), other.根.take());
        self.個数 += other.個数;
        other.個数 = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    #[test]
    fn 基本動作() {
        let mut h = スキューヒープ::new();
        assert!(h.空か());
        for v in [5, 1, 3, 2, 4] {
            h.挿入(v);
        }
        assert_eq!(h.個数(), 5);
        assert_eq!(h.最小参照(), Some(&1));
        let mut 出 = Vec::new();
        while let Some(x) = h.最小取り出し() {
            出.push(x);
        }
        assert_eq!(出, vec![1, 2, 3, 4, 5]);
        assert!(h.空か());
    }

    #[test]
    fn 併合の確認() {
        let mut a = スキューヒープ::new();
        let mut b = スキューヒープ::new();
        for v in [1, 4, 7] {
            a.挿入(v);
        }
        for v in [2, 3, 9] {
            b.挿入(v);
        }
        a.併合(&mut b);
        assert!(b.空か());
        assert_eq!(a.個数(), 6);
        let mut 出 = Vec::new();
        while let Some(x) = a.最小取り出し() {
            出.push(x);
        }
        assert_eq!(出, vec![1, 2, 3, 4, 7, 9]);
    }

    #[test]
    fn 乱数で標準ヒープと一致() {
        let mut seed = 0x5e30_4ea0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..300 {
            let mut h = スキューヒープ::new();
            let mut oracle: BinaryHeap<Reverse<i64>> = BinaryHeap::new(); // 最小ヒープ
            for _ in 0..200 {
                if 乱数() % 3 == 0 && !oracle.is_empty() {
                    // 取り出し
                    assert_eq!(h.最小取り出し(), oracle.pop().map(|Reverse(x)| x));
                } else {
                    let v = (乱数() % 1000) as i64;
                    h.挿入(v);
                    oracle.push(Reverse(v));
                }
                assert_eq!(h.個数(), oracle.len());
                assert_eq!(h.最小参照().copied(), oracle.peek().map(|&Reverse(x)| x));
            }
        }
    }

    #[test]
    fn 乱数併合で一致() {
        let mut seed = 0x3e1d_44a0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..500 {
            let mut a = スキューヒープ::new();
            let mut b = スキューヒープ::new();
            let mut 全部 = Vec::new();
            for _ in 0..(乱数() as usize % 30) {
                let v = (乱数() % 500) as i64;
                a.挿入(v);
                全部.push(v);
            }
            for _ in 0..(乱数() as usize % 30) {
                let v = (乱数() % 500) as i64;
                b.挿入(v);
                全部.push(v);
            }
            a.併合(&mut b);
            全部.sort_unstable();
            let mut 出 = Vec::new();
            while let Some(x) = a.最小取り出し() {
                出.push(x);
            }
            assert_eq!(出, 全部);
        }
    }
}
