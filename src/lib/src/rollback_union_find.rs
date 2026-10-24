use cargo_snippet::snippet;

#[snippet("ロールバック_UnionFind")]
#[snippet("Undo_可能_UnionFind")]
#[snippet("巻き戻し_UnionFind")]
#[snippet("RollbackUnionFind")]
#[snippet("撤回_可能_DSU")]
/// 巻き戻し（Undo）可能な Union-Find。経路圧縮を行わずユニオン by サイズのみで結合し、
/// 各結合の変更を履歴に積むことで任意時点へ `O(操作数)` で復元できる。
///
/// `ac_library::Dsu` は経路圧縮するため undo できない。オフライン動的連結性
/// （時間方向のセグメント木上の分割統治）などで「結合 → 探索 → 巻き戻し」を繰り返す用途に使う。
/// `根`・`同じ`・`サイズ` は経路圧縮なしの O(log N)。
struct ロールバック可能UnionFind {
    /// 根なら `-(サイズ)`、非根なら親の添字。
    親orサイズ: Vec<isize>,
    /// `(書き換えた添字, 直前の値)` の履歴。1 回の結合で 2 件積む。
    履歴: Vec<(usize, isize)>,
}

#[snippet("ロールバック_UnionFind")]
#[snippet("Undo_可能_UnionFind")]
#[snippet("巻き戻し_UnionFind")]
#[snippet("RollbackUnionFind")]
#[snippet("撤回_可能_DSU")]
impl ロールバック可能UnionFind {
    fn new(n: usize) -> Self {
        Self {
            親orサイズ: vec![-1; n],
            履歴: Vec::new(),
        }
    }

    /// `x` の根（経路圧縮なし）。
    fn 根(&self, mut x: usize) -> usize {
        while self.親orサイズ[x] >= 0 {
            x = self.親orサイズ[x] as usize;
        }
        x
    }

    /// `x` と `y` を結合する。既に同じ集合なら `false`（履歴は積まない）。
    fn 統合(&mut self, x: usize, y: usize) -> bool {
        let mut a = self.根(x);
        let mut b = self.根(y);
        if a == b {
            return false;
        }
        // a を大きい方（より負＝サイズ大）にする
        if self.親orサイズ[a] > self.親orサイズ[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.履歴.push((a, self.親orサイズ[a]));
        self.履歴.push((b, self.親orサイズ[b]));
        self.親orサイズ[a] += self.親orサイズ[b];
        self.親orサイズ[b] = a as isize;
        true
    }

    /// `x` と `y` が同じ集合か。
    fn 同じ(&self, x: usize, y: usize) -> bool {
        self.根(x) == self.根(y)
    }

    /// `x` の属する集合の要素数。
    fn サイズ(&self, x: usize) -> usize {
        let r = self.根(x);
        (-self.親orサイズ[r]) as usize
    }

    /// 現在のスナップショット（履歴の長さ）。[`Self::ロールバック`] に渡す。
    fn 現在時点(&self) -> usize {
        self.履歴.len()
    }

    /// 直近の `統合` 1 回ぶんを取り消す（履歴 2 件を巻き戻す）。
    fn 巻き戻し(&mut self) {
        for _ in 0..2 {
            if let Some((i, v)) = self.履歴.pop() {
                self.親orサイズ[i] = v;
            }
        }
    }

    /// [`Self::現在時点`] で得た時点まで状態を巻き戻す。
    fn ロールバック(&mut self, 時点: usize) {
        while self.履歴.len() > 時点 {
            let (i, v) = self.履歴.pop().unwrap();
            self.親orサイズ[i] = v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 基本的な結合と判定() {
        let mut uf = ロールバック可能UnionFind::new(5);
        assert!(!uf.同じ(0, 1));
        assert!(uf.統合(0, 1));
        assert!(uf.統合(1, 2));
        assert!(uf.同じ(0, 2));
        assert_eq!(uf.サイズ(0), 3);
        assert!(!uf.統合(0, 2)); // 既に同じ
        assert!(!uf.同じ(0, 3));
    }

    #[test]
    fn 巻き戻しで状態が戻る() {
        let mut uf = ロールバック可能UnionFind::new(4);
        uf.統合(0, 1);
        let 時点 = uf.現在時点();
        uf.統合(1, 2);
        uf.統合(2, 3);
        assert!(uf.同じ(0, 3));
        assert_eq!(uf.サイズ(0), 4);
        uf.ロールバック(時点);
        // (1,2),(2,3) が取り消され (0,1) のみ残る
        assert!(uf.同じ(0, 1));
        assert!(!uf.同じ(0, 2));
        assert_eq!(uf.サイズ(0), 2);
    }

    #[test]
    fn 巻き戻し単発() {
        let mut uf = ロールバック可能UnionFind::new(3);
        uf.統合(0, 1);
        uf.統合(1, 2);
        assert!(uf.同じ(0, 2));
        uf.巻き戻し();
        assert!(!uf.同じ(0, 2));
        assert!(uf.同じ(0, 1));
        uf.巻き戻し();
        assert!(!uf.同じ(0, 1));
    }
}
