use crate::graph_build::無向グラフ;
use cargo_snippet::snippet;

#[snippet("全方位_木_DP", include = "無向_グラフ")]
#[snippet("全頂点への_距離_総和")]
#[snippet("距離_総和")]
#[snippet("木の_距離_和")]
/// 全方位木DP（再帰なし版）の代表例。各頂点 `v` について
/// 「`v` から木の全頂点への距離（辺数）の総和」を一括で O(N) 計算する。
///
/// `辺` は AtCoder の入力そのまま（0-indexed の頂点対、重みなし無向木）。
/// 戻り値 `ans[v]` が頂点 `v` の距離総和。
fn 全頂点への距離総和(n: usize, 辺: &[(usize, usize)]) -> Vec<i64> {
    if n == 0 {
        return vec![];
    }
    let グラフ = 無向グラフ(n, 辺);

    // 根 0 から DFS 順（親が子より先に来る順）を作る
    let mut prev = vec![!0; n];
    let mut 順 = Vec::with_capacity(n);
    let mut 到達済 = vec![false; n];
    let mut 探索 = vec![0usize];
    到達済[0] = true;
    while let Some(now) = 探索.pop() {
        順.push(now);
        for &next in &グラフ[now] {
            if !到達済[next] {
                到達済[next] = true;
                prev[next] = now;
                探索.push(next);
            }
        }
    }

    // 葉→根: 部分木サイズと「部分木内の距離和」を集約
    let mut sz = vec![1i64; n];
    let mut 部分木距離和 = vec![0i64; n];
    for &now in 順.iter().rev() {
        let p = prev[now];
        if p != !0 {
            sz[p] += sz[now];
            部分木距離和[p] += 部分木距離和[now] + sz[now];
        }
    }

    // 根→葉: 根の付け替えで全体の距離和を伝播
    let mut ans = vec![0i64; n];
    ans[0] = 部分木距離和[0];
    for &now in &順 {
        for &next in &グラフ[now] {
            if prev[next] == now {
                ans[next] = ans[now] + (n as i64 - sz[next]) - sz[next];
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 直線の距離総和() {
        // 0-1-2-3-4
        let ans = 全頂点への距離総和(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
        assert_eq!(ans, vec![10, 7, 6, 7, 10]);
    }

    #[test]
    fn 星グラフの距離総和() {
        // 中心 0 に 1,2,3 がぶら下がる
        let ans = 全頂点への距離総和(4, &[(0, 1), (0, 2), (0, 3)]);
        assert_eq!(ans[0], 3); // 0 -> 各葉 距離1 ×3
        assert_eq!(ans[1], 5); // 1->0(1), 1->2(2), 1->3(2)
    }
}
