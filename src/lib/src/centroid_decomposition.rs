use crate::graph_build::無向グラフ;
use cargo_snippet::snippet;

#[snippet("重心_分解", include = "無向_グラフ")]
#[snippet("Centroid_分解")]
#[snippet("CentroidDecomposition")]
#[snippet("木の_分割_統治")]
/// 木の重心分解。各頂点の「重心木」での親を返す（根は `!0`）。O(N log N)。
///
/// `辺` は 0-indexed の頂点対（AtCoder の入力そのまま）。返り値 `重心親[v]` は、
/// 重心分解で得られる木構造における `v` の親頂点。重心木の深さは O(log N) に収まるため、
/// 「2 頂点間のパスは必ず両者の重心木祖先のいずれかを通る」性質を使ったパスクエリの
/// 分割統治に用いる。連結な木を前提とする。
fn 重心分解(n: usize, 辺: &[(usize, usize)]) -> Vec<usize> {
    let グラフ = 無向グラフ(n, 辺);
    let mut 重心親 = vec![!0; n];
    let mut 削除済 = vec![false; n];
    let mut sz = vec![0usize; n];
    // (連結成分の開始頂点, 重心木での親)
    let mut 作業 = vec![(0usize, !0)];
    // DFS 用バッファは使い回す
    let mut 順 = Vec::new();
    let mut prev = vec![!0; n];
    while let Some((開始, 重心木親)) = 作業.pop() {
        // この連結成分を反復 DFS で走査し、部分木サイズを求める
        順.clear();
        prev[開始] = !0;
        let mut 探索 = vec![開始];
        while let Some(now) = 探索.pop() {
            順.push(now);
            for &next in &グラフ[now] {
                if next != prev[now] && !削除済[next] {
                    prev[next] = now;
                    探索.push(next);
                }
            }
        }
        let 全体 = 順.len();
        for &now in 順.iter().rev() {
            sz[now] = 1;
            for &next in &グラフ[now] {
                if next != prev[now] && !削除済[next] {
                    sz[now] += sz[next];
                }
            }
        }
        // 重心を探す: 重い子へ降りていく
        let mut 重心 = 開始;
        let mut 前 = !0;
        loop {
            let mut 次 = !0;
            for &v in &グラフ[重心] {
                if v != 前 && !削除済[v] && sz[v] * 2 > 全体 {
                    次 = v;
                    break;
                }
            }
            if 次 == !0 {
                break;
            }
            // 親方向の部分木サイズを補正してから降りる
            sz[重心] = 全体 - sz[次];
            前 = 重心;
            重心 = 次;
        }
        重心親[重心] = 重心木親;
        削除済[重心] = true;
        for &next in &グラフ[重心] {
            if !削除済[next] {
                作業.push((next, 重心));
            }
        }
    }
    重心親
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn スター型は中心が根() {
        // 0 を中心に 1,2,3,4 がぶら下がる
        let 親 = 重心分解(5, &[(0, 1), (0, 2), (0, 3), (0, 4)]);
        assert_eq!(親[0], usize::MAX);
        for &p in &親[1..5] {
            assert_eq!(p, 0);
        }
    }

    #[test]
    fn パス5本は中央が根() {
        // 0-1-2-3-4 の直線。重心は 2
        let 親 = 重心分解(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
        assert_eq!(親[2], usize::MAX);
        assert_eq!(親.iter().filter(|&&p| p == usize::MAX).count(), 1);
    }

    #[test]
    fn 全頂点が連結された重心木になる() {
        let 親 = 重心分解(7, &[(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)]);
        // 根は 1 個だけ
        assert_eq!(親.iter().filter(|&&p| p == usize::MAX).count(), 1);
        // 根まで辿れる（閉路なし・全頂点到達可能）
        for v in 0..7 {
            let mut cur = v;
            let mut 歩数 = 0;
            while 親[cur] != usize::MAX {
                cur = 親[cur];
                歩数 += 1;
                assert!(歩数 <= 7, "重心木に閉路がある");
            }
        }
    }
}
