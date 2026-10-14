use cargo_snippet::snippet;

#[snippet("更新あり_Mo")]
#[snippet("時刻軸_Mo")]
#[snippet("MoWithUpdates")]
#[snippet("3次元_Mo")]
/// 点更新を挟むオフライン区間クエリを扱う「更新あり Mo（時刻軸 Mo, 3D Mo）」の状態インタフェース。
///
/// 素の [`Mo`](crate::mo) は配列が不変だが、こちらは「`t` 番目までの点更新を適用した配列」に対して
/// 区間 `[l, r)` のクエリに答える。区間ポインタ（追加/削除）に加え、時刻ポインタを
/// 前後させる `前進`/`後退` を実装する。更新位置が現在の対象区間内なら寄与も更新すること。
trait 時刻付きMo {
    /// 各クエリに対する答えの型。
    type 出力;
    /// 位置 `位置` の要素を現在の対象区間に加える。
    fn 追加(&mut self, 位置: usize);
    /// 位置 `位置` の要素を現在の対象区間から取り除く。
    fn 削除(&mut self, 位置: usize);
    /// 時刻 `t` の更新を適用して時刻を 1 進める。`[cl, cr)` は現在の対象区間。
    fn 前進(&mut self, t: usize, cl: usize, cr: usize);
    /// 時刻 `t` の更新を取り消して時刻を 1 戻す。`[cl, cr)` は現在の対象区間。
    fn 後退(&mut self, t: usize, cl: usize, cr: usize);
    /// 現在の対象区間・時刻に対する答えを返す。
    fn 答え(&mut self) -> Self::出力;
}

#[snippet("更新あり_Mo")]
#[snippet("時刻軸_Mo")]
#[snippet("MoWithUpdates")]
#[snippet("3次元_Mo")]
/// 更新あり Mo 本体。`クエリ` は `(l, r, t)`（「先頭 `t` 個の更新を適用した配列」での区間 `[l, r)`）の列。
///
/// 各クエリの答えを入力順に並べて返す。全体 `O(N^(5/3))`（更新コスト別）。
/// `状態` は空区間 `[0, 0)`・時刻 `0` の初期状態で渡すこと。`S` は [`時刻付きMo`] を実装した型。
fn 時刻付きmo実行<S: 時刻付きMo>(
    状態: &mut S,
    クエリ: &[(usize, usize, usize)],
) -> Vec<S::出力> {
    let q = クエリ.len();
    let mut 答え = Vec::with_capacity(q);
    if q == 0 {
        return 答え;
    }
    let 上限 = クエリ.iter().map(|&(_, r, _)| r).max().unwrap().max(1);
    // 3D Mo の最適ブロック幅はおよそ N^(2/3)。
    let ブロック = ((上限 as f64).powf(2.0 / 3.0) as usize).max(1);
    let mut 順: Vec<usize> = (0..q).collect();
    順.sort_by(|&a, &b| {
        let (la, ra, ta) = クエリ[a];
        let (lb, rb, tb) = クエリ[b];
        let (bla, blb) = (la / ブロック, lb / ブロック);
        if bla != blb {
            return bla.cmp(&blb);
        }
        let (bra, brb) = (ra / ブロック, rb / ブロック);
        if bra != brb {
            // r ブロックは l ブロックの偶奇でジグザグさせ移動を減らす。
            return if bla % 2 == 0 {
                bra.cmp(&brb)
            } else {
                brb.cmp(&bra)
            };
        }
        if bra % 2 == 0 {
            ta.cmp(&tb)
        } else {
            tb.cmp(&ta)
        }
    });
    let mut 出力: Vec<Option<S::出力>> = (0..q).map(|_| None).collect();
    let (mut cl, mut cr, mut ct) = (0usize, 0usize, 0usize);
    for &i in &順 {
        let (l, r, t) = クエリ[i];
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
        // 区間を確定させてから時刻を動かす（更新の寄与判定に [cl, cr) を使う）。
        while ct < t {
            状態.前進(ct, cl, cr);
            ct += 1;
        }
        while ct > t {
            ct -= 1;
            状態.後退(ct, cl, cr);
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
    use std::collections::HashSet;

    /// 点更新を挟む「区間内の異なる値の個数」。
    struct 種類数更新付き {
        arr: Vec<usize>,
        // (位置, 値)。適用/取り消しのたびに arr と値を入れ替える（swap は自己逆）。
        更新: Vec<(usize, usize)>,
        個数: Vec<usize>,
        種類: usize,
    }

    impl 種類数更新付き {
        fn new(arr: Vec<usize>, 更新: Vec<(usize, usize)>, 最大値: usize) -> Self {
            Self {
                arr,
                更新,
                個数: vec![0; 最大値 + 1],
                種類: 0,
            }
        }
        // 時刻 t の更新を arr に反映（前進・後退とも同一操作）。
        fn 適用(&mut self, t: usize, cl: usize, cr: usize) {
            let pos = self.更新[t].0;
            let 区間内 = cl <= pos && pos < cr;
            if 区間内 {
                self.削除(pos); // 旧値の寄与を外す
            }
            // arr[pos] と 更新値 を入れ替え（取り消し時は逆向きに戻る）。
            std::mem::swap(&mut self.arr[pos], &mut self.更新[t].1);
            if 区間内 {
                self.追加(pos); // 新値の寄与を入れる
            }
        }
    }

    impl 時刻付きMo for 種類数更新付き {
        type 出力 = usize;
        fn 追加(&mut self, 位置: usize) {
            let v = self.arr[位置];
            if self.個数[v] == 0 {
                self.種類 += 1;
            }
            self.個数[v] += 1;
        }
        fn 削除(&mut self, 位置: usize) {
            let v = self.arr[位置];
            self.個数[v] -= 1;
            if self.個数[v] == 0 {
                self.種類 -= 1;
            }
        }
        fn 前進(&mut self, t: usize, cl: usize, cr: usize) {
            self.適用(t, cl, cr);
        }
        fn 後退(&mut self, t: usize, cl: usize, cr: usize) {
            self.適用(t, cl, cr);
        }
        fn 答え(&mut self) -> usize {
            self.種類
        }
    }

    #[test]
    fn 更新あり区間種類数が総当たりと一致() {
        let mut seed = 0x3d05_5e70u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..200 {
            let n = (乱数() as usize % 8) + 1;
            let 値域 = (乱数() as usize % 4) + 1; // 値は 0..値域
            let arr: Vec<usize> = (0..n).map(|_| 乱数() as usize % 値域).collect();
            let u = 乱数() as usize % 6; // 更新の本数
            let 更新: Vec<(usize, usize)> = (0..u)
                .map(|_| (乱数() as usize % n, 乱数() as usize % 値域))
                .collect();

            // 全クエリ (l, r, t) を列挙。
            let mut クエリ = Vec::new();
            for t in 0..=u {
                for l in 0..n {
                    for r in l + 1..=n {
                        クエリ.push((l, r, t));
                    }
                }
            }
            let mut 状態 = 種類数更新付き::new(arr.clone(), 更新.clone(), 値域);
            let 結果 = 時刻付きmo実行(&mut 状態, &クエリ);

            for (k, &(l, r, t)) in クエリ.iter().enumerate() {
                // t 個の更新を適用した配列を素朴に作る。
                let mut a = arr.clone();
                for &(p, v) in 更新.iter().take(t) {
                    a[p] = v;
                }
                let 期待: usize = a[l..r].iter().collect::<HashSet<_>>().len();
                assert_eq!(
                    結果[k], 期待,
                    "n={n} l={l} r={r} t={t} arr={arr:?} 更新={更新:?}"
                );
            }
        }
    }

    #[test]
    fn 空クエリは空結果() {
        let mut 状態 = 種類数更新付き::new(vec![0, 1, 2], vec![], 2);
        assert!(時刻付きmo実行(&mut 状態, &[]).is_empty());
    }
}
