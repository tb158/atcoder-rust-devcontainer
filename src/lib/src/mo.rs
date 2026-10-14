use cargo_snippet::snippet;

#[snippet("Mo")]
#[snippet("Mo_の_アルゴリズム")]
#[snippet("平方_分割_クエリ")]
#[snippet("オフライン_区間_クエリ")]
/// Mo のアルゴリズムで扱う区間状態。現在の対象区間 `[l, r)` を 1 要素ずつ伸縮させながら、
/// 各クエリの答えを得るためのインタフェースを実装する。
///
/// 伸縮は端点の単純な追加・削除で答えを更新できる問題（区間内の種類数・同値ペア数など）に
/// 適用できる。状態は `状態.追加 / 削除` の呼び出しに対し O(1)〜O(log) で更新できること。
trait Mo {
    /// 各クエリに対する答えの型。
    type 出力;
    /// 位置 `位置` の要素を現在の対象区間に加える。
    fn 追加(&mut self, 位置: usize);
    /// 位置 `位置` の要素を現在の対象区間から取り除く。
    fn 削除(&mut self, 位置: usize);
    /// 現在の対象区間に対する答えを返す。
    fn 答え(&mut self) -> Self::出力;
}

#[snippet("Mo")]
#[snippet("Mo_の_アルゴリズム")]
#[snippet("平方_分割_クエリ")]
#[snippet("オフライン_区間_クエリ")]
/// Mo のアルゴリズム本体。`クエリ` は半開区間 `[l, r)` の列で、各クエリの答えを
/// 入力順に並べて返す。全体 O((N + Q)·√N · 更新コスト)。
///
/// `状態` は空区間 `[0, 0)` を表す初期状態で渡すこと。`S` は [`Mo`] を実装した型。
fn mo実行<S: Mo>(状態: &mut S, クエリ: &[(usize, usize)]) -> Vec<S::出力> {
    let q = クエリ.len();
    let mut 答え = Vec::with_capacity(q);
    if q == 0 {
        return 答え;
    }
    // 右端の最大値からブロック幅を決める
    let 上限 = クエリ.iter().map(|&(_, r)| r).max().unwrap().max(1);
    let ブロック = ((上限 as f64).sqrt() as usize).max(1);
    let mut 順: Vec<usize> = (0..q).collect();
    順.sort_by(|&a, &b| {
        let (la, ra) = クエリ[a];
        let (lb, rb) = クエリ[b];
        let (ba, bb) = (la / ブロック, lb / ブロック);
        if ba != bb {
            return ba.cmp(&bb);
        }
        // 同ブロック内は r を交互に昇順/降順（ジグザグ）でポインタ移動を減らす
        if ba % 2 == 0 {
            ra.cmp(&rb)
        } else {
            rb.cmp(&ra)
        }
    });
    let mut 出力: Vec<Option<S::出力>> = (0..q).map(|_| None).collect();
    let (mut cl, mut cr) = (0usize, 0usize); // 現在の対象区間 [cl, cr)
    for &i in &順 {
        let (l, r) = クエリ[i];
        while cr < r {
            状態.追加(cr);
            cr += 1;
        }
        while cl > l {
            cl -= 1;
            状態.追加(cl);
        }
        while cr > r {
            cr -= 1;
            状態.削除(cr);
        }
        while cl < l {
            状態.削除(cl);
            cl += 1;
        }
        出力[i] = Some(状態.答え());
    }
    for o in 出力 {
        答え.push(o.unwrap());
    }
    答え
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 区間内の異なる値の個数を数える状態。
    struct 種類数 {
        値: Vec<usize>,
        個数: Vec<usize>,
        種類: usize,
    }

    impl 種類数 {
        fn new(値: Vec<usize>, 最大値: usize) -> Self {
            Self {
                値,
                個数: vec![0; 最大値 + 1],
                種類: 0,
            }
        }
    }

    impl Mo for 種類数 {
        type 出力 = usize;
        fn 追加(&mut self, 位置: usize) {
            let v = self.値[位置];
            if self.個数[v] == 0 {
                self.種類 += 1;
            }
            self.個数[v] += 1;
        }
        fn 削除(&mut self, 位置: usize) {
            let v = self.値[位置];
            self.個数[v] -= 1;
            if self.個数[v] == 0 {
                self.種類 -= 1;
            }
        }
        fn 答え(&mut self) -> usize {
            self.種類
        }
    }

    #[test]
    fn 区間種類数が総当たりと一致() {
        let 配列 = vec![1usize, 2, 1, 3, 2, 2, 4, 1, 3, 0];
        let n = 配列.len();
        let 最大値 = *配列.iter().max().unwrap();
        // 全ての半開区間 [l, r), l < r
        let mut クエリ = Vec::new();
        for l in 0..n {
            for r in l + 1..=n {
                クエリ.push((l, r));
            }
        }
        let mut 状態 = 種類数::new(配列.clone(), 最大値);
        let 結果 = mo実行(&mut 状態, &クエリ);

        for (k, &(l, r)) in クエリ.iter().enumerate() {
            let mut s = std::collections::HashSet::new();
            for &x in &配列[l..r] {
                s.insert(x);
            }
            assert_eq!(結果[k], s.len(), "区間 [{l}, {r})");
        }
    }

    #[test]
    fn 空クエリは空結果() {
        let mut 状態 = 種類数::new(vec![1, 2, 3], 3);
        let 結果 = mo実行(&mut 状態, &[]);
        assert!(結果.is_empty());
    }
}
