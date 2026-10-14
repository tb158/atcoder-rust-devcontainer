use crate::graph_build::無向グラフ;
use cargo_snippet::snippet;

#[snippet("木の_重心", include = "無向_グラフ")]
#[snippet("重心")]
#[snippet("centroid")]
/// 木の重心を求める（再帰なし）。重心とは、その頂点を取り除いたとき残る
/// 各連結成分の最大サイズが `n/2` 以下になる頂点。木の重心は 1 個か 2 個ある。
///
/// `辺` は AtCoder の入力そのまま（0-indexed の頂点対）。戻り値は重心の頂点番号（昇順、長さ 1 か 2）。
/// 重心を分割の基準に使うと、各部分木サイズが半分以下に抑えられる（重心分解の核）。
/// 離心数最小の「中心」は別物で [`木の中心`](crate::tree_diameter::木の中心) を使う。
fn 木の重心(n: usize, 辺: &[(usize, usize)]) -> Vec<usize> {
    if n <= 1 {
        return (0..n).collect();
    }
    let グラフ = 無向グラフ(n, 辺);

    // 根 0 から DFS 順と親を求める
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

    // 葉→根で部分木サイズ
    let mut sz = vec![1usize; n];
    for &now in 順.iter().rev() {
        if prev[now] != !0 {
            sz[prev[now]] += sz[now];
        }
    }

    // 各頂点を除いたときの最大成分サイズを評価
    let mut 重心 = Vec::new();
    for now in 0..n {
        let mut 最大成分 = n - sz[now]; // 親側の成分
        for &next in &グラフ[now] {
            if prev[next] == now {
                最大成分 = 最大成分.max(sz[next]);
            }
        }
        if 最大成分 <= n / 2 {
            重心.push(now);
        }
    }
    重心
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 奇数長の鎖は中央が重心() {
        // 0-1-2-3-4 → 中央 2
        assert_eq!(木の重心(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]), vec![2]);
    }

    #[test]
    fn 偶数長の鎖は重心が2個() {
        // 0-1-2-3 → 1 と 2
        assert_eq!(木の重心(4, &[(0, 1), (1, 2), (2, 3)]), vec![1, 2]);
    }

    #[test]
    fn 星グラフは中心が重心() {
        assert_eq!(木の重心(4, &[(0, 1), (0, 2), (0, 3)]), vec![0]);
    }
}
