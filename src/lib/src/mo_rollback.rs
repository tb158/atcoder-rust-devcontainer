use cargo_snippet::snippet;

#[snippet("ロールバック_Mo")]
#[snippet("巻き戻し_Mo")]
#[snippet("RollbackMo")]
#[snippet("追加のみ_Mo")]
/// 「追加は容易だが削除が難しい」区間クエリを扱うロールバック Mo の状態インタフェース。
///
/// 削除の代わりに、チェックポイント [`保存`](ロールバックMo::保存) と
/// [`巻き戻し`](ロールバックMo::巻き戻し) で状態を巻き戻す。区間の左側の一時追加を
/// 巻き戻すことで、各左ブロック内で右ポインタを単調増加に保ったまま処理できる。
/// 素の [`Mo`](crate::mo)・[更新あり Mo](crate::mo_with_updates) とは別アルゴリズム。
/// 連結性（rollback 可能 union-find）や max・mode 系で使う。
trait ロールバックMo {
    /// 各クエリに対する答えの型。
    type 出力;
    /// 位置 `位置` の要素を現在の集合に加える（削除は無い）。
    fn 追加(&mut self, 位置: usize);
    /// 現在の状態をチェックポイントとして積む。
    fn 保存(&mut self);
    /// 直近のチェックポイントまで状態を巻き戻す（その時点以降の追加を取り消す）。
    fn 巻き戻し(&mut self);
    /// 状態を空に戻す（各左ブロックの処理開始時に呼ばれる）。
    fn 全消去(&mut self);
    /// 現在の集合に対する答えを返す。
    fn 答え(&mut self) -> Self::出力;
}

#[snippet("ロールバック_Mo")]
#[snippet("巻き戻し_Mo")]
#[snippet("RollbackMo")]
#[snippet("追加のみ_Mo")]
/// ロールバック Mo 本体。`n` は要素数、`クエリ` は半開区間 `[l, r)`（`l < r`）の列。
///
/// 全体 `O((N + Q)√N · 追加コスト)`。`状態` は空の初期状態で渡すこと。
fn ロールバックmo実行<S: ロールバックMo>(
    状態: &mut S,
    n: usize,
    クエリ: &[(usize, usize)],
) -> Vec<S::出力> {
    let q = クエリ.len();
    if q == 0 {
        return Vec::new();
    }
    let b = ((n as f64).sqrt() as usize).max(1);
    let mut 順: Vec<usize> = (0..q).collect();
    順.sort_by(|&x, &y| {
        let (lx, rx) = クエリ[x];
        let (ly, ry) = クエリ[y];
        let (bx, by) = (lx / b, ly / b);
        if bx != by { bx.cmp(&by) } else { rx.cmp(&ry) }
    });
    let mut 答え: Vec<Option<S::出力>> = (0..q).map(|_| None).collect();
    let mut idx = 0;
    while idx < 順.len() {
        let ブロック = クエリ[順[idx]].0 / b;
        let r0 = ((ブロック + 1) * b).min(n); // このブロックの右端（半開、排他）
        状態.全消去();
        let mut 右 = r0; // 右ポインタ。集合は常に [r0, 右) を保持。
        while idx < 順.len() && クエリ[順[idx]].0 / b == ブロック {
            let i = 順[idx];
            let (l, r) = クエリ[i];
            if r <= r0 {
                // ブロック内に収まる短い区間: その場で追加→巻き戻し。
                状態.保存();
                for p in l..r {
                    状態.追加(p);
                }
                答え[i] = Some(状態.答え());
                状態.巻き戻し();
                idx += 1;
                continue;
            }
            while 右 < r {
                状態.追加(右);
                右 += 1;
            }
            状態.保存();
            for p in (l..r0).rev() {
                状態.追加(p); // 左側 [l, r0) を一時追加
            }
            答え[i] = Some(状態.答え());
            状態.巻き戻し(); // 左側だけ取り消し、右側 [r0, 右) は残す
            idx += 1;
        }
    }
    答え.into_iter().map(|o| o.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// 追加のみ＋巻き戻しで「区間内の異なる値の個数」を保つ状態。
    struct 種類数RB {
        値: Vec<usize>,
        個数: Vec<usize>,
        種類: usize,
        // 履歴: (追加した値, この追加で種類が増えたか)
        履歴: Vec<(usize, bool)>,
        印: Vec<usize>, // チェックポイント（履歴長）
    }

    impl 種類数RB {
        fn new(値: Vec<usize>, 最大値: usize) -> Self {
            Self {
                値,
                個数: vec![0; 最大値 + 1],
                種類: 0,
                履歴: Vec::new(),
                印: Vec::new(),
            }
        }
    }

    impl ロールバックMo for 種類数RB {
        type 出力 = usize;
        fn 追加(&mut self, 位置: usize) {
            let v = self.値[位置];
            self.個数[v] += 1;
            let 増えた = self.個数[v] == 1;
            if 増えた {
                self.種類 += 1;
            }
            self.履歴.push((v, 増えた));
        }
        fn 保存(&mut self) {
            self.印.push(self.履歴.len());
        }
        fn 巻き戻し(&mut self) {
            let 目標 = self.印.pop().unwrap();
            while self.履歴.len() > 目標 {
                let (v, 増えた) = self.履歴.pop().unwrap();
                self.個数[v] -= 1;
                if 増えた {
                    self.種類 -= 1;
                }
            }
        }
        fn 全消去(&mut self) {
            // 履歴を全て巻き戻して空状態に。
            while let Some((v, 増えた)) = self.履歴.pop() {
                self.個数[v] -= 1;
                if 増えた {
                    self.種類 -= 1;
                }
            }
            self.印.clear();
        }
        fn 答え(&mut self) -> usize {
            self.種類
        }
    }

    #[test]
    fn 区間種類数が総当たりと一致() {
        let mut seed = 0x40_11_ba_c0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..400 {
            let n = (乱数() as usize % 30) + 1;
            let 値域 = (乱数() as usize % 6) + 1;
            let 配列: Vec<usize> = (0..n).map(|_| 乱数() as usize % 値域).collect();
            // 全区間 [l, r) を列挙。
            let mut クエリ = Vec::new();
            for l in 0..n {
                for r in l + 1..=n {
                    クエリ.push((l, r));
                }
            }
            let mut 状態 = 種類数RB::new(配列.clone(), 値域);
            let 結果 = ロールバックmo実行(&mut 状態, n, &クエリ);
            for (k, &(l, r)) in クエリ.iter().enumerate() {
                let 期待 = 配列[l..r].iter().collect::<HashSet<_>>().len();
                assert_eq!(結果[k], 期待, "n={n} [{l},{r}) 配列={配列:?}");
            }
        }
    }

    #[test]
    fn 空クエリは空結果() {
        let mut 状態 = 種類数RB::new(vec![0, 1, 2], 2);
        assert!(ロールバックmo実行(&mut 状態, 3, &[]).is_empty());
    }
}
