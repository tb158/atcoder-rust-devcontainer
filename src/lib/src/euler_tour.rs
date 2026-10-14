use crate::graph_build::無向グラフ;
use cargo_snippet::snippet;

#[snippet("オイラー_ツアー", include = "無向_グラフ")]
#[snippet("オイラー_ツアー_部分木")]
#[snippet("行きがけ_帰りがけ")]
#[snippet("部分木_区間")]
#[snippet("inout_時刻")]
/// オイラーツアー（部分木 → 区間への変換）。根 `根` から木を DFS し、各頂点 `v` の
/// 入時刻 `in_[v]` と出時刻 `out[v]` を返す（再帰なし）。O(N)。`in_` は行きがけ順 0..N。
///
/// `辺` は 0-indexed の頂点対（AtCoder の入力そのまま）。
///
/// 部分木 `v` は半開区間 `[in_[v], out[v])` に対応する（`out[v] - in_[v]` が部分木サイズ）。
/// `u` が `v` の部分木に含まれる ⇔ `in_[v] <= in_[u] < out[v]`。
/// この区間を BIT / セグ木の添字に使えば「部分木への一括加算・部分木和」を扱える。
fn オイラーツアー(n: usize, 辺: &[(usize, usize)], 根: usize) -> (Vec<usize>, Vec<usize>) {
    let グラフ = 無向グラフ(n, 辺);
    let mut in_ = vec![0usize; n];
    let mut out = vec![0usize; n];
    let mut 時刻 = 0usize;
    let mut 探索 = vec![(根, !0)];
    while let Some((now, prev)) = 探索.pop() {
        if now >= n {
            // 帰り
            let now = !now;
            out[now] = 時刻;
            continue;
        }
        // 行き
        in_[now] = 時刻;
        時刻 += 1;
        探索.push((!now, prev)); // 帰りを先に積む
        for &next in &グラフ[now] {
            if next != prev {
                探索.push((next, now));
            }
        }
    }
    (in_, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 部分木が区間になる() {
        //        0
        //       / \
        //      1   2
        //     / \
        //    3   4
        let (in_, out) = オイラーツアー(5, &[(0, 1), (0, 2), (1, 3), (1, 4)], 0);
        // 部分木サイズ = out - in
        assert_eq!(out[0] - in_[0], 5); // 全体
        assert_eq!(out[1] - in_[1], 3); // 1,3,4
        assert_eq!(out[2] - in_[2], 1); // 2 のみ
        // 3 は 1 の部分木に含まれる
        assert!(in_[1] <= in_[3] && in_[3] < out[1]);
        // 2 は 1 の部分木に含まれない
        assert!(!(in_[1] <= in_[2] && in_[2] < out[1]));
    }
}
