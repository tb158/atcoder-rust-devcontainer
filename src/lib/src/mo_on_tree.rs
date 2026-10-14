use crate::graph_build::無向グラフ;
use cargo_snippet::snippet;

#[snippet("木上_Mo", include = "無向_グラフ")]
#[snippet("パス_クエリ_Mo")]
#[snippet("TreeMo")]
#[snippet("オイラー_ツアー_Mo")]
/// 木のパスクエリ（頂点 `u`-`v` 間の単純パス上の集計）を扱う Mo の状態インタフェース。
///
/// オイラーツアー上の位置を出入りさせると、ある頂点はパス上にあるとき区間内にちょうど 1 回現れる。
/// ランナーがその「出現回数の偶奇」を管理し、パスに入る頂点で [`追加`](木Mo::追加)、
/// 出る頂点で [`削除`](木Mo::削除) を呼ぶ。状態は集合への頂点の出入りだけ実装すればよい。
/// 列に対する素の [`Mo`](crate::mo) とは別技法。
trait 木Mo {
    /// 各クエリに対する答えの型。
    type 出力;
    /// 頂点 `頂点` を現在の集合に加える。
    fn 追加(&mut self, 頂点: usize);
    /// 頂点 `頂点` を現在の集合から取り除く。
    fn 削除(&mut self, 頂点: usize);
    /// 現在の集合に対する答えを返す。
    fn 答え(&mut self) -> Self::出力;
}

#[snippet("木上_Mo")]
#[snippet("パス_クエリ_Mo")]
#[snippet("TreeMo")]
#[snippet("オイラー_ツアー_Mo")]
/// 木上の Mo 本体。`n` は頂点数、`辺` は AtCoder の入力そのまま（0-indexed の頂点対）、
/// `根` は根、`クエリ` は `(u, v)`（u-v パス）の列。
///
/// 全体 `O((N + Q)√N · 更新コスト)`。`状態` は空集合の初期状態で渡すこと。
fn 木上mo実行<S: 木Mo>(
    n: usize,
    辺: &[(usize, usize)],
    根: usize,
    クエリ: &[(usize, usize)],
    状態: &mut S,
) -> Vec<S::出力> {
    let q = クエリ.len();
    if q == 0 {
        return Vec::new();
    }
    let グラフ = 無向グラフ(n, 辺);
    // オイラーツアー（反復 DFS）。st/en は列上の位置、順[位置]=頂点。
    let mut st = vec![0usize; n];
    let mut en = vec![0usize; n];
    let mut 順 = Vec::with_capacity(2 * n);
    let mut depth = vec![0usize; n];
    let mut prev = vec![!0; n];
    let mut 時刻 = 0usize;
    let mut 探索 = vec![根];
    while let Some(now) = 探索.pop() {
        if now >= n {
            // 帰り
            let now = !now;
            en[now] = 時刻;
            順.push(now);
            時刻 += 1;
            continue;
        }
        // 行き
        depth[now] = if prev[now] == !0 { 0 } else { depth[prev[now]] + 1 };
        st[now] = 時刻;
        順.push(now);
        時刻 += 1;
        探索.push(!now); // 帰りを先に積む
        for &next in &グラフ[now] {
            if next != prev[now] {
                prev[next] = now;
                探索.push(next);
            }
        }
    }
    // 二分上昇による LCA。
    let log = (usize::BITS - n.leading_zeros()).max(1) as usize;
    let mut up = vec![vec![!0; n]; log];
    up[0].copy_from_slice(&prev);
    for k in 1..log {
        for v in 0..n {
            let m = up[k - 1][v];
            up[k][v] = if m == !0 {
                !0
            } else {
                up[k - 1][m]
            };
        }
    }
    let lca = |mut u: usize, mut v: usize| -> usize {
        if depth[u] < depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let 差 = depth[u] - depth[v];
        for (k, 上) in up.iter().enumerate() {
            if 差 >> k & 1 == 1 {
                u = 上[u];
            }
        }
        if u == v {
            return u;
        }
        for 上 in up.iter().rev() {
            if 上[u] != 上[v] {
                u = 上[u];
                v = 上[v];
            }
        }
        prev[u]
    };
    // 各クエリを区間 [l, r)（半開）＋追加 LCA に変換。
    let mut 区間: Vec<(usize, usize, Option<usize>, usize)> = Vec::with_capacity(q); // (l, r, extra, qi)
    for (qi, &(mut u, mut v)) in クエリ.iter().enumerate() {
        if st[u] > st[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let l = lca(u, v);
        if l == u {
            区間.push((st[u], st[v] + 1, None, qi)); // u は v の祖先
        } else {
            区間.push((en[u], st[v] + 1, Some(l), qi)); // パスは LCA を除く＋追加
        }
    }
    let b = ((2 * n) as f64).sqrt() as usize + 1;
    区間.sort_by(|x, y| {
        let (bx, by) = (x.0 / b, y.0 / b);
        if bx != by {
            bx.cmp(&by)
        } else if bx % 2 == 0 {
            x.1.cmp(&y.1)
        } else {
            y.1.cmp(&x.1)
        }
    });
    let mut 使用中 = vec![false; n];
    let mut 出力: Vec<Option<S::出力>> = (0..q).map(|_| None).collect();
    let (mut cl, mut cr) = (0usize, 0usize); // 現在の被覆位置 [cl, cr)
    for &(l, r, extra, qi) in &区間 {
        while cr < r {
            let v = 順[cr];
            if 使用中[v] {
                状態.削除(v);
            } else {
                状態.追加(v);
            }
            使用中[v] = !使用中[v];
            cr += 1;
        }
        while cl > l {
            cl -= 1;
            let v = 順[cl];
            if 使用中[v] {
                状態.削除(v);
            } else {
                状態.追加(v);
            }
            使用中[v] = !使用中[v];
        }
        while cr > r {
            cr -= 1;
            let v = 順[cr];
            if 使用中[v] {
                状態.削除(v);
            } else {
                状態.追加(v);
            }
            使用中[v] = !使用中[v];
        }
        while cl < l {
            let v = 順[cl];
            if 使用中[v] {
                状態.削除(v);
            } else {
                状態.追加(v);
            }
            使用中[v] = !使用中[v];
            cl += 1;
        }
        出力[qi] = Some(match extra {
            Some(w) => {
                状態.追加(w); // LCA を一時的に加える
                let a = 状態.答え();
                状態.削除(w);
                a
            }
            None => 状態.答え(),
        });
    }
    出力.into_iter().map(|o| o.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// パス上の異なる頂点重みの個数。
    struct 種類数木 {
        重み: Vec<usize>,
        個数: Vec<usize>,
        種類: usize,
    }
    impl 木Mo for 種類数木 {
        type 出力 = usize;
        fn 追加(&mut self, 頂点: usize) {
            let w = self.重み[頂点];
            if self.個数[w] == 0 {
                self.種類 += 1;
            }
            self.個数[w] += 1;
        }
        fn 削除(&mut self, 頂点: usize) {
            let w = self.重み[頂点];
            self.個数[w] -= 1;
            if self.個数[w] == 0 {
                self.種類 -= 1;
            }
        }
        fn 答え(&mut self) -> usize {
            self.種類
        }
    }

    // 親・深さから u-v パス上の頂点集合を素朴に得る。
    fn パス頂点(u: usize, v: usize, par: &[usize], depth: &[usize]) -> Vec<usize> {
        let (mut a, mut b) = (u, v);
        let mut 前 = Vec::new();
        let mut 後 = Vec::new();
        while depth[a] > depth[b] {
            前.push(a);
            a = par[a];
        }
        while depth[b] > depth[a] {
            後.push(b);
            b = par[b];
        }
        while a != b {
            前.push(a);
            後.push(b);
            a = par[a];
            b = par[b];
        }
        前.push(a); // LCA
        前.extend(後.into_iter().rev());
        前
    }

    #[test]
    fn パス種類数が総当たりと一致() {
        let mut seed = 0x73ee_3e00u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..300 {
            let n = (乱数() as usize % 30) + 1;
            // ランダム木（各頂点 i>=1 の親を 0..i から選ぶ）。
            let mut 辺 = Vec::new();
            let mut par = vec![usize::MAX; n];
            let mut depth = vec![0usize; n];
            for i in 1..n {
                let p = (乱数() as usize) % i;
                辺.push((p, i));
                par[i] = p;
                depth[i] = depth[p] + 1;
            }
            let 値域 = (乱数() as usize % 5) + 1;
            let 重み: Vec<usize> = (0..n).map(|_| 乱数() as usize % 値域).collect();

            // ランダムクエリ。
            let q = (乱数() as usize % 20) + 1;
            let クエリ: Vec<(usize, usize)> = (0..q)
                .map(|_| ((乱数() as usize) % n, (乱数() as usize) % n))
                .collect();

            let mut 状態 = 種類数木 {
                重み: 重み.clone(),
                個数: vec![0; 値域],
                種類: 0,
            };
            let 結果 = 木上mo実行(n, &辺, 0, &クエリ, &mut 状態);

            for (k, &(u, v)) in クエリ.iter().enumerate() {
                let 頂点 = パス頂点(u, v, &par, &depth);
                let 期待 = 頂点.iter().map(|&x| 重み[x]).collect::<HashSet<_>>().len();
                assert_eq!(結果[k], 期待, "n={n} u={u} v={v} 重み={重み:?}");
            }
        }
    }

    #[test]
    fn 空クエリは空結果() {
        let mut 状態 = 種類数木 {
            重み: vec![0, 0],
            個数: vec![0; 1],
            種類: 0,
        };
        assert!(木上mo実行(2, &[(0, 1)], 0, &[], &mut 状態).is_empty());
    }
}
