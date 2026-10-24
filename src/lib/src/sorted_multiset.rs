use cargo_snippet::snippet;

#[snippet("ソート済み_多重集合")]
#[snippet("OrderedMultiset")]
#[snippet("SortedMultiset")]
#[snippet("ソート済み_インデックスアクセス可")]
/// ソート済み多重集合（平方分割）。常に昇順に並んだ多重集合で、挿入・1 個削除・k 番目・
/// 順位（`x` 未満／以下の個数）・`x` 以上の最小／`x` 以下の最大をまとめて扱う。
/// 値を事前に知らなくてよい（オンライン）ので、二分探索をその場で書かずにここへ押し付けられる。
///
/// 内部は長さ 1024 以下の昇順 `Vec` を並べたバケット列。各バケット末尾への二分探索で対象バケットを
/// 決め、バケット内も二分探索する。1024 を超えたバケットは半分に割り、空になったバケットは除去する。
/// バケット数を `B`（≤ 挿入回数 / 512 + 1。`N = 2·10^5` なら約 400）として
///
/// - [`Self::挿入`] / [`Self::削除`] : 二分探索 + バケット内の `Vec` 途中挿入・削除で `O(log N + 1024 + B)`
/// - [`Self::以上の最小`] / [`Self::以下の最大`] / [`Self::最小`] / [`Self::最大`] : `O(log N)`
/// - [`Self::k番目`] / [`Self::未満の個数`] / [`Self::以下の個数`] : バケット長の線形走査で `O(B + log N)`
/// - [`Self::含む個数`] : `O(log N + x がまたがるバケット数)`
///
/// `BTreeSet` / `BTreeMap` は k 番目・順位を取れない。[`二進トライ`](crate::binary_trie::二進トライ) は
/// 非負整数専用でメモリも重く、BIT＋座標圧縮は値を先読みできるとき限定、
/// [`陰的Treap`](crate::treap::陰的Treap) は位置で管理する列なので、値の大小で並べたいならこちらを使う。
///
/// 値の型は `Ord + Copy` なら何でもよい（整数・`(isize, usize)` などのタプル・`char`）。
/// 引数も返り値も値渡しにして呼び出し側の `&` / `*` を省くため `Copy` を要求する。
#[derive(Clone, Debug)]
struct ソート済み多重集合<T> {
    バケット: Vec<Vec<T>>, // 各バケットは空でない昇順列で、順に連結しても昇順
    長さ: usize,
}

#[snippet("ソート済み_多重集合")]
#[snippet("OrderedMultiset")]
#[snippet("SortedMultiset")]
#[snippet("ソート済み_インデックスアクセス可")]
impl<T: Ord + Copy> ソート済み多重集合<T> {
    /// バケットを半分に割る長さ。小さいと k 番目・順位の線形走査が、大きいと `Vec` の途中挿入が重い。
    /// `N = 2·10^5`・値 `[0, 10^9)` の乱数で挿入／k 番目／順位を各 `N` 回した合計（1.89.0 release）:
    /// 256 → 109ms, 512 → 66ms, 1024 → 54ms, 2048 → 57ms, 4096 → 74ms。
    const 分割閾値: usize = 1024;

    /// 空の多重集合を作る。
    fn new() -> Self {
        Self {
            バケット: Vec::new(),
            長さ: 0,
        }
    }

    /// 要素数（重複込み）。
    fn 長さ(&self) -> usize {
        self.長さ
    }

    /// 空かどうか。
    fn 空か(&self) -> bool {
        self.長さ == 0
    }

    /// 昇順に見て true が続きある所から false になる述語 `前か` が、初めて false になる要素の位置
    /// `(バケット番号, バケット内位置)`。全要素で true なら `(バケット数, 0)`。
    fn 境界(&self, 前か: impl Fn(T) -> bool) -> (usize, usize) {
        let i = self.バケット.partition_point(|b| 前か(b[b.len() - 1]));
        let j = self.バケット.get(i).map_or(0, |b| b.partition_point(|&y| 前か(y)));
        (i, j)
    }

    /// 値 `x` を 1 個追加する。
    fn 挿入(&mut self, x: T) {
        self.長さ += 1;
        if self.バケット.is_empty() {
            self.バケット.push(vec![x]);
            return;
        }
        // x 以上の最初の要素の直前へ。全要素より大きければ末尾バケットの最後へ。
        let (mut i, mut j) = self.境界(|y| y < x);
        if i == self.バケット.len() {
            i -= 1;
            j = self.バケット[i].len();
        }
        let b = &mut self.バケット[i];
        b.insert(j, x);
        if b.len() > Self::分割閾値 {
            let 後半 = b.split_off(b.len() / 2);
            self.バケット.insert(i + 1, 後半);
        }
    }

    /// 値 `x` を 1 個だけ削除し、削除できたら `true`（無ければ何もせず `false`）。
    fn 削除(&mut self, x: T) -> bool {
        let (i, j) = self.境界(|y| y < x);
        match self.バケット.get_mut(i) {
            Some(b) if b[j] == x => {
                b.remove(j);
                if b.is_empty() {
                    self.バケット.remove(i);
                }
                self.長さ -= 1;
                true
            }
            _ => false,
        }
    }

    /// 0 始まりで `k` 番目に小さい値（重複込み）。`k >= 長さ` なら panic。
    fn k番目(&self, k: usize) -> T {
        let mut 残り = k;
        for b in &self.バケット {
            if 残り < b.len() {
                return b[残り];
            }
            残り -= b.len();
        }
        panic!("ソート済み多重集合: k = {k} が範囲外（長さ {}）", self.長さ)
    }

    /// `x` 未満の要素数（`x` の順位。`k番目(未満の個数(x))` が `x` 以上の最小）。
    fn 未満の個数(&self, x: T) -> usize {
        let (i, j) = self.境界(|y| y < x);
        self.バケット[..i].iter().map(Vec::len).sum::<usize>() + j
    }

    /// `x` 以下の要素数。
    fn 以下の個数(&self, x: T) -> usize {
        let (i, j) = self.境界(|y| y <= x);
        self.バケット[..i].iter().map(Vec::len).sum::<usize>() + j
    }

    /// 値 `x` の個数。
    fn 含む個数(&self, x: T) -> usize {
        let (i, j) = self.境界(|y| y < x);
        let (i2, j2) = self.境界(|y| y <= x);
        self.バケット[i..i2].iter().map(Vec::len).sum::<usize>() + j2 - j
    }

    /// `x` 以上の最小値（無ければ `None`）。
    fn 以上の最小(&self, x: T) -> Option<T> {
        let (i, j) = self.境界(|y| y < x);
        self.バケット.get(i).map(|b| b[j])
    }

    /// `x` 以下の最大値（無ければ `None`）。
    fn 以下の最大(&self, x: T) -> Option<T> {
        let (i, j) = self.境界(|y| y <= x);
        if j > 0 {
            Some(self.バケット[i][j - 1])
        } else {
            // 境界がバケット先頭なので直前のバケットの末尾
            self.バケット[..i].last().map(|b| b[b.len() - 1])
        }
    }

    /// 最小値（空なら `None`）。
    fn 最小(&self) -> Option<T> {
        self.バケット.first().map(|b| b[0])
    }

    /// 最大値（空なら `None`）。
    fn 最大(&self) -> Option<T> {
        self.バケット.last().map(|b| b[b.len() - 1])
    }

    /// 全要素を昇順に列挙する（重複込み）。
    fn 昇順(&self) -> impl Iterator<Item = T> {
        self.バケット.iter().flatten().copied()
    }
}

#[snippet("ソート済み_多重集合")]
#[snippet("OrderedMultiset")]
#[snippet("SortedMultiset")]
#[snippet("ソート済み_インデックスアクセス可")]
impl<T: Ord + Copy> Default for ソート済み多重集合<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 内部の不変条件: 各バケットは空でなく閾値以下、連結すると昇順、総数が長さと一致。
    fn 不変条件を確認(s: &ソート済み多重集合<isize>) {
        let 閾値 = ソート済み多重集合::<isize>::分割閾値;
        assert!(s.バケット.iter().all(|b| !b.is_empty() && b.len() <= 閾値));
        let 全部: Vec<isize> = s.昇順().collect();
        assert!(全部.is_sorted());
        assert_eq!(全部.len(), s.長さ());
    }

    #[test]
    fn 空集合() {
        let mut s = ソート済み多重集合::<isize>::new();
        assert!(s.空か());
        assert_eq!(s.長さ(), 0);
        assert_eq!(s.最小(), None);
        assert_eq!(s.最大(), None);
        assert_eq!(s.未満の個数(0), 0);
        assert_eq!(s.以下の個数(0), 0);
        assert_eq!(s.含む個数(0), 0);
        assert_eq!(s.以上の最小(0), None);
        assert_eq!(s.以下の最大(0), None);
        assert!(!s.削除(0));
        assert_eq!(s.昇順().count(), 0);
    }

    #[test]
    fn 重複と境界() {
        let mut s = ソート済み多重集合::new();
        for x in [5isize, 1, 5, 9, 5] {
            s.挿入(x);
        }
        // [1, 5, 5, 5, 9]
        assert_eq!(s.k番目(0), 1);
        assert_eq!(s.k番目(s.長さ() - 1), 9);
        assert_eq!(s.含む個数(5), 3);
        assert_eq!(s.未満の個数(5), 1);
        assert_eq!(s.以下の個数(5), 4);
        assert_eq!(s.以上の最小(5), Some(5));
        assert_eq!(s.以下の最大(5), Some(5));
        // 存在しない値の順位
        assert_eq!(s.未満の個数(3), 1);
        assert_eq!(s.以下の個数(3), 1);
        assert_eq!(s.含む個数(3), 0);
        assert_eq!(s.未満の個数(0), 0);
        assert_eq!(s.以下の個数(100), 5);
        assert_eq!(s.以上の最小(6), Some(9));
        assert_eq!(s.以上の最小(10), None);
        assert_eq!(s.以下の最大(4), Some(1));
        assert_eq!(s.以下の最大(0), None);
        // 削除は 1 個だけ、無い値は false
        assert!(s.削除(5));
        assert_eq!(s.含む個数(5), 2);
        assert!(!s.削除(7));
        assert_eq!(s.昇順().collect::<Vec<_>>(), vec![1, 5, 5, 9]);
        // 全削除で空に戻り、再挿入もできる
        for x in [9, 5, 1, 5] {
            assert!(s.削除(x));
        }
        assert!(s.空か());
        assert_eq!(s.最小(), None);
        assert!(!s.削除(5));
        s.挿入(-3);
        assert_eq!((s.最小(), s.最大()), (Some(-3), Some(-3)));
    }

    #[test]
    #[should_panic(expected = "範囲外")]
    fn k番目の範囲外() {
        let mut s = ソート済み多重集合::new();
        s.挿入(1isize);
        s.k番目(1);
    }

    #[test]
    fn タプルでも使える() {
        let mut s = ソート済み多重集合::new();
        for (i, v) in [3isize, 1, 3].into_iter().enumerate() {
            s.挿入((v, i));
        }
        assert_eq!(s.k番目(1), (3, 0));
        assert_eq!(s.未満の個数((3, 0)), 1);
        assert_eq!(s.以上の最小((3, 1)), Some((3, 2)));
        assert_eq!(s.以下の最大((2, 9)), Some((1, 1)));
    }

    #[test]
    fn 単調挿入で分割() {
        // 末尾（先頭）に挿入し続けると同じバケットばかり伸びて分割が連続する
        let n = 5000;
        let mut 昇 = ソート済み多重集合::new();
        let mut 降 = ソート済み多重集合::new();
        for x in 0..n {
            昇.挿入(x);
            降.挿入(n - 1 - x);
        }
        for s in [&昇, &降] {
            不変条件を確認(s);
            assert!(s.バケット.len() > 1);
            assert!((0..n).all(|k| s.k番目(k as usize) == k));
        }
    }

    #[test]
    fn 乱数でソート済みvecと一致() {
        let mut seed = 0x0bad_5eedu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        // 値域が狭いほど同じ値の連続がバケット境界をまたぐ
        for 値域 in [3u64, 50, 2_000_000_000] {
            let 下限 = -(値域 as isize) / 2;
            let mut s = ソート済み多重集合::new();
            let mut オラクル: Vec<isize> = Vec::new(); // 素朴なソート済み Vec
            // 前半は挿入多めで閾値を超えて分割させ、後半は削除多めで全削除まで進める
            for 段階 in 0..2 {
                let mut 手数 = 0;
                while if 段階 == 0 { 手数 < 15000 } else { !オラクル.is_empty() } {
                    手数 += 1;
                    let 操作 = 乱数() % 20;
                    // 挿入値は [下限, 下限+値域)、問い合わせ値はその 1 つ外側まで
                    let x = 下限 + (乱数() % 値域) as isize;
                    let q = 下限 - 1 + (乱数() % (値域 + 2)) as isize;
                    let 挿入率 = if 段階 == 0 { 12 } else { 2 };
                    if 操作 < 挿入率 {
                        s.挿入(x);
                        let p = オラクル.partition_point(|&y| y < x);
                        オラクル.insert(p, x);
                    } else if 操作 < 14 && !オラクル.is_empty() {
                        // 存在する値の削除
                        let x = オラクル.remove(乱数() as usize % オラクル.len());
                        assert!(s.削除(x));
                    } else if 操作 < 15 {
                        // 存在するとは限らない値の削除
                        let p = オラクル.partition_point(|&y| y < q);
                        let ある = オラクル.get(p) == Some(&q);
                        if ある {
                            オラクル.remove(p);
                        }
                        assert_eq!(s.削除(q), ある);
                    } else {
                        let 未満 = オラクル.partition_point(|&y| y < q);
                        let 以下 = オラクル.partition_point(|&y| y <= q);
                        assert_eq!(s.未満の個数(q), 未満);
                        assert_eq!(s.以下の個数(q), 以下);
                        assert_eq!(s.含む個数(q), 以下 - 未満);
                        assert_eq!(s.以上の最小(q), オラクル.get(未満).copied());
                        assert_eq!(s.以下の最大(q), 以下.checked_sub(1).map(|i| オラクル[i]));
                        if !オラクル.is_empty() {
                            let k = 乱数() as usize % オラクル.len();
                            assert_eq!(s.k番目(k), オラクル[k]);
                        }
                    }
                    assert_eq!(s.長さ(), オラクル.len());
                    assert_eq!(s.空か(), オラクル.is_empty());
                    assert_eq!(s.最小(), オラクル.first().copied());
                    assert_eq!(s.最大(), オラクル.last().copied());
                    if 手数 % 500 == 0 {
                        不変条件を確認(&s);
                        assert!(s.昇順().eq(オラクル.iter().copied()));
                    }
                }
                if 段階 == 0 {
                    assert!(s.バケット.len() > 1, "分割が起きていない");
                }
            }
            assert!(s.空か() && s.バケット.is_empty());
        }
    }
}
