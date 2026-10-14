use cargo_snippet::snippet;

#[snippet("オフライン_動的_連結性")]
#[snippet("OfflineDynamicConnectivity")]
#[snippet("動的_連結性")]
#[snippet("時間_方向_セグ木")]
#[snippet("DynamicConnectivity")]
/// オフライン動的連結性で使う巻き戻し（Undo）可能 Union-Find。経路圧縮せずユニオン by サイズで結合し、
/// 各結合をスタックに積んで任意時点へ復元する。`連結成分数` も追跡する。
struct 巻き戻しDSU {
    親orサイズ: Vec<i64>,
    履歴: Vec<(usize, i64)>,
    成分数: usize,
}

#[snippet("オフライン_動的_連結性")]
#[snippet("OfflineDynamicConnectivity")]
#[snippet("動的_連結性")]
#[snippet("時間_方向_セグ木")]
#[snippet("DynamicConnectivity")]
impl 巻き戻しDSU {
    fn new(n: usize) -> Self {
        Self {
            親orサイズ: vec![-1; n],
            履歴: Vec::new(),
            成分数: n,
        }
    }

    /// `x` の根（経路圧縮なし）。
    fn 根(&self, mut x: usize) -> usize {
        while self.親orサイズ[x] >= 0 {
            x = self.親orサイズ[x] as usize;
        }
        x
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

    /// 現在の連結成分数。
    fn 連結成分数(&self) -> usize {
        self.成分数
    }

    /// `x` と `y` を結合する。既に同じ集合なら `false`。
    fn 統合(&mut self, x: usize, y: usize) -> bool {
        let mut a = self.根(x);
        let mut b = self.根(y);
        if a == b {
            return false;
        }
        if self.親orサイズ[a] > self.親orサイズ[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.履歴.push((a, self.親orサイズ[a]));
        self.履歴.push((b, self.親orサイズ[b]));
        self.親orサイズ[a] += self.親orサイズ[b];
        self.親orサイズ[b] = a as i64;
        self.成分数 -= 1;
        true
    }

    /// 現在のスナップショット（履歴長）。
    fn 現在時点(&self) -> usize {
        self.履歴.len()
    }

    /// `時点` まで状態を巻き戻す（履歴は 1 結合につき 2 件）。成分数も復元する。
    fn ロールバック(&mut self, 時点: usize) {
        let 結合数 = (self.履歴.len() - 時点) / 2;
        while self.履歴.len() > 時点 {
            let (i, v) = self.履歴.pop().unwrap();
            self.親orサイズ[i] = v;
        }
        self.成分数 += 結合数;
    }
}

#[snippet("オフライン_動的_連結性")]
#[snippet("OfflineDynamicConnectivity")]
#[snippet("動的_連結性")]
#[snippet("時間_方向_セグ木")]
#[snippet("DynamicConnectivity")]
/// オフライン動的連結性。辺の追加・削除がある時系列（時刻 `0..T`）に対し、各時刻での連結性を一括で求める。
///
/// 「各辺が存在する時刻区間 `[開始, 終了)`」を時間方向のセグメント木に載せて分割統治する。
/// 全体 `O((辺 + T) log T α)`。クエリを先読みできる（オフライン）ことが前提。
///
/// - [`Self::辺追加`] : 辺 `(u, v)` が存在する時刻区間を登録。
/// - [`Self::解く`] : 各時刻 `t` でコールバックに DSU を渡す。
struct オフライン動的連結性 {
    時刻数: usize,
    頂点数: usize,
    辺: Vec<Vec<(usize, usize)>>,
}

#[snippet("オフライン_動的_連結性")]
#[snippet("OfflineDynamicConnectivity")]
#[snippet("動的_連結性")]
#[snippet("時間_方向_セグ木")]
#[snippet("DynamicConnectivity")]
impl オフライン動的連結性 {
    /// 頂点数 `頂点数`、時刻数 `時刻数`（時刻は `0..時刻数`）で初期化する。
    fn new(頂点数: usize, 時刻数: usize) -> Self {
        Self {
            時刻数,
            頂点数,
            辺: vec![Vec::new(); 4 * 時刻数.max(1)],
        }
    }

    /// 辺 `(u, v)` が時刻区間 `[開始, 終了)` に存在することを登録する。
    fn 辺追加(&mut self, u: usize, v: usize, 開始: usize, 終了: usize) {
        if 開始 < 終了 {
            self.区間に追加(1, 0, self.時刻数, 開始, 終了, (u, v));
        }
    }

    fn 区間に追加(
        &mut self,
        節点: usize,
        nl: usize,
        nr: usize,
        l: usize,
        r: usize,
        辺: (usize, usize),
    ) {
        if r <= nl || nr <= l {
            return;
        }
        if l <= nl && nr <= r {
            self.辺[節点].push(辺);
            return;
        }
        let 中 = (nl + nr) / 2;
        self.区間に追加(2 * 節点, nl, 中, l, r, 辺);
        self.区間に追加(2 * 節点 + 1, 中, nr, l, r, 辺);
    }

    /// セグメント木を DFS し、各時刻 `t`（`0..時刻数`）でその時点の DSU をコールバックに渡す。
    fn 解く<F: FnMut(usize, &巻き戻しDSU)>(&self, mut 処理: F) {
        if self.時刻数 == 0 {
            return;
        }
        let mut dsu = 巻き戻しDSU::new(self.頂点数);
        self.探索(1, 0, self.時刻数, &mut dsu, &mut 処理);
    }

    fn 探索<F: FnMut(usize, &巻き戻しDSU)>(
        &self,
        節点: usize,
        nl: usize,
        nr: usize,
        dsu: &mut 巻き戻しDSU,
        処理: &mut F,
    ) {
        let スナップ = dsu.現在時点();
        for &(u, v) in &self.辺[節点] {
            dsu.統合(u, v);
        }
        if nr - nl == 1 {
            処理(nl, dsu);
        } else {
            let 中 = (nl + nr) / 2;
            self.探索(2 * 節点, nl, 中, dsu, 処理);
            self.探索(2 * 節点 + 1, 中, nr, dsu, 処理);
        }
        dsu.ロールバック(スナップ);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // (成分数, 全ペア (u, v, 連結か)) を時刻順に並べたもの。
    type 時刻別結果 = Vec<(usize, Vec<(usize, usize, bool)>)>;

    fn 根(親: &mut [usize], x: usize) -> usize {
        let mut x = x;
        while 親[x] != x {
            親[x] = 親[親[x]];
            x = 親[x];
        }
        x
    }

    // 総当たり: 各時刻ごとにその時点の有効辺だけで素朴 DSU を作り直し、(成分数, 全ペアの連結性) を返す。
    fn 素朴(
        頂点数: usize,
        時刻数: usize,
        辺: &[(usize, usize, usize, usize)],
    ) -> 時刻別結果 {
        let mut 結果 = Vec::new();
        for t in 0..時刻数 {
            let mut 親: Vec<usize> = (0..頂点数).collect();
            for &(u, v, l, r) in 辺 {
                if l <= t && t < r {
                    let a = 根(&mut 親, u);
                    let b = 根(&mut 親, v);
                    親[a] = b;
                }
            }
            let 成分: std::collections::HashSet<usize> =
                (0..頂点数).map(|v| 根(&mut 親, v)).collect();
            let mut ペア = Vec::new();
            for u in 0..頂点数 {
                for v in u + 1..頂点数 {
                    ペア.push((u, v, 根(&mut 親, u) == 根(&mut 親, v)));
                }
            }
            結果.push((成分.len(), ペア));
        }
        結果
    }

    #[test]
    fn 簡単な追加削除() {
        // 頂点3, 時刻0..3。辺(0,1)は時刻[0,2)、辺(1,2)は時刻[1,3)。
        let mut dc = オフライン動的連結性::new(3, 3);
        dc.辺追加(0, 1, 0, 2);
        dc.辺追加(1, 2, 1, 3);
        let mut 成分 = vec![0usize; 3];
        let mut つながり = vec![false; 3]; // 0-2 が連結か
        dc.解く(|t, dsu| {
            成分[t] = dsu.連結成分数();
            つながり[t] = dsu.同じ(0, 2);
        });
        // t=0: {0,1},{2} → 2成分, 0-2非連結
        // t=1: {0,1,2}    → 1成分, 0-2連結
        // t=2: {0},{1,2}  → 2成分, 0-2非連結
        assert_eq!(成分, vec![2, 1, 2]);
        assert_eq!(つながり, vec![false, true, false]);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x0123_4567u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..80 {
            let 頂点数 = (乱数() as usize % 6) + 2; // 2..=7
            let 時刻数 = (乱数() as usize % 8) + 1; // 1..=8
            let 辺数 = 乱数() as usize % 10;
            let mut 辺 = Vec::new();
            let mut dc = オフライン動的連結性::new(頂点数, 時刻数);
            for _ in 0..辺数 {
                let u = 乱数() as usize % 頂点数;
                let v = 乱数() as usize % 頂点数;
                if u == v {
                    continue;
                }
                let a = 乱数() as usize % 時刻数;
                let b = 乱数() as usize % 時刻数;
                let (l, r) = if a <= b { (a, b + 1) } else { (b, a + 1) };
                辺.push((u, v, l, r));
                dc.辺追加(u, v, l, r);
            }
            let 期待 = 素朴(頂点数, 時刻数, &辺);
            let mut 実成分 = vec![0usize; 時刻数];
            let mut 実ペア: Vec<Vec<(usize, usize, bool)>> = vec![Vec::new(); 時刻数];
            dc.解く(|t, dsu| {
                実成分[t] = dsu.連結成分数();
                let mut p = Vec::new();
                for u in 0..頂点数 {
                    for v in u + 1..頂点数 {
                        p.push((u, v, dsu.同じ(u, v)));
                    }
                }
                実ペア[t] = p;
            });
            for t in 0..時刻数 {
                assert_eq!(実成分[t], 期待[t].0, "成分数 t={t}");
                assert_eq!(実ペア[t], 期待[t].1, "連結性 t={t}");
            }
        }
    }
}
