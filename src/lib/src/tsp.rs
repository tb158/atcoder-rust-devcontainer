use cargo_snippet::snippet;

#[snippet("巡回_セールスマン")]
#[snippet("巡回_セールスマン_問題")]
#[snippet("TSP")]
#[snippet("bitDP")]
#[snippet("ビット_DP")]
#[snippet("集合_DP")]
/// 巡回セールスマン問題（TSP）をビット DP で解く。O(2^N · N^2)。N は 18 程度まで。
///
/// `距離[i][j]` は i→j のコスト（有向。到達不可は十分大きい値を入れる）。
/// 頂点 0 を始点に全頂点をちょうど 1 回ずつ訪れて 0 に戻る最小コストを返す。
/// 巡回が作れない場合は `None`。
///
/// 「戻らない最短ハミルトンパス」が欲しい場合は最後の `+ 距離[v][0]` を外し、
/// `dp[全集合][v]` の最小をそのまま使う。
// ビット DP では添字がそのまま集合・頂点のビット位置なので、範囲ループの添字が必須。
#[allow(clippy::needless_range_loop)]
fn 巡回セールスマン(距離: &[Vec<i64>]) -> Option<i64> {
    let n = 距離.len();
    if n == 0 {
        return Some(0);
    }
    const INF: i64 = i64::MAX / 4;
    // dp[訪問集合][現在地] = その状態に至る最小コスト
    let mut dp = vec![vec![INF; n]; 1 << n];
    dp[1][0] = 0; // 頂点 0 のみ訪問済み、現在地は 0
    for s in 1..(1usize << n) {
        for v in 0..n {
            if dp[s][v] >= INF || s & (1 << v) == 0 {
                continue;
            }
            for u in 0..n {
                if s & (1 << u) != 0 {
                    continue;
                }
                let 次 = s | (1 << u);
                let コスト = dp[s][v] + 距離[v][u];
                if コスト < dp[次][u] {
                    dp[次][u] = コスト;
                }
            }
        }
    }
    let 全集合 = (1usize << n) - 1;
    let 答え = (0..n)
        .filter(|&v| dp[全集合][v] < INF)
        .map(|v| dp[全集合][v] + 距離[v][0])
        .min()
        .unwrap_or(INF);
    if 答え >= INF { None } else { Some(答え) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 正方形の巡回() {
        // 4 頂点を一周。各辺 1、対角線 2 の正方形 0-1-2-3-0
        let d = vec![
            vec![0, 1, 2, 1],
            vec![1, 0, 1, 2],
            vec![2, 1, 0, 1],
            vec![1, 2, 1, 0],
        ];
        // 0->1->2->3->0 = 1+1+1+1 = 4
        assert_eq!(巡回セールスマン(&d), Some(4));
    }

    #[test]
    fn 一頂点は0() {
        let d = vec![vec![0]];
        assert_eq!(巡回セールスマン(&d), Some(0));
    }
}
