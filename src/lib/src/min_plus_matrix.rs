use cargo_snippet::snippet;

/// 到達不能・無限大を表す番兵。加算しても溢れないよう十分小さく取る。
#[snippet("ミンプラス_行列")]
#[snippet("トロピカル_行列")]
#[snippet("MinPlusMatrix")]
const 無限: i64 = 1 << 60;

#[snippet("ミンプラス_行列")]
#[snippet("トロピカル_行列")]
#[snippet("MinPlusMatrix")]
/// min-plus（トロピカル）半環での正方行列の積 `(A⊗B)[i][j] = min_k (A[i][k] + B[k][j])`。`O(n^3)`。
/// `無限`（≒到達不能）同士の加算でも溢れないよう番兵で頭打ちにする。
#[allow(clippy::needless_range_loop)] // 行列積は i,k,j の添字でアクセスするのが自然
fn ミンプラス積(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut c = vec![vec![無限; n]; n];
    for i in 0..n {
        for k in 0..n {
            if a[i][k] >= 無限 {
                continue;
            }
            let aik = a[i][k];
            for j in 0..n {
                let v = aik.saturating_add(b[k][j]).min(無限);
                if v < c[i][j] {
                    c[i][j] = v;
                }
            }
        }
    }
    c
}

#[snippet("ミンプラス_行列")]
#[snippet("トロピカル_行列")]
#[snippet("MinPlusMatrix")]
/// min-plus 行列の `e` 乗を二分累乗で求める。`O(n^3 log e)`。
///
/// 入力 `行列[i][j]` を「辺 `i→j` の重み（辺が無ければ `無限` 以上）」とみなすと、結果の
/// `[i][j]` は **`i` から `j` へちょうど `e` 本の辺で行く最短歩道**の重み（到達不能なら `≥ 無限`）。
/// `e == 0` は単位元（対角 0・他 `無限`）。重み付き有向グラフで「辺数を固定した最短路」や
/// 「k 手後の最小コスト」を求めるのに使う。
fn ミンプラス累乗(行列: &[Vec<i64>], e: usize) -> Vec<Vec<i64>> {
    let n = 行列.len();
    // 単位元：対角 0、その他 無限。
    let mut 結果 = vec![vec![無限; n]; n];
    for (i, row) in 結果.iter_mut().enumerate() {
        row[i] = 0;
    }
    let mut 底: Vec<Vec<i64>> = 行列
        .iter()
        .map(|r| r.iter().map(|&x| x.min(無限)).collect())
        .collect();
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            結果 = ミンプラス積(&結果, &底);
        }
        底 = ミンプラス積(&底, &底);
        e >>= 1;
    }
    結果
}

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_range_loop)] // 行列・グラフを添字で総当たり検証するため

    use super::*;

    // 始点固定で「ちょうど step 辺」での最短コストを直接 DP するオラクル。
    fn ちょうどk辺最短(隣接: &[Vec<i64>], 始点: usize, k: usize) -> Vec<i64> {
        let n = 隣接.len();
        let mut dp = vec![無限; n];
        dp[始点] = 0;
        for _ in 0..k {
            let mut 次 = vec![無限; n];
            for u in 0..n {
                if dp[u] >= 無限 {
                    continue;
                }
                for v in 0..n {
                    if 隣接[u][v] < 無限 {
                        次[v] = 次[v].min(dp[u] + 隣接[u][v]);
                    }
                }
            }
            dp = 次;
        }
        dp
    }

    #[test]
    fn 単位元と1乗() {
        let w = vec![
            vec![無限, 5, 無限],
            vec![無限, 無限, 3],
            vec![2, 無限, 無限],
        ];
        // e=0 は単位元。
        let id = ミンプラス累乗(&w, 0);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(id[i][j], if i == j { 0 } else { 無限 });
            }
        }
        // e=1 は元の行列そのもの。
        assert_eq!(ミンプラス累乗(&w, 1), w);
        // 0→1→2 はちょうど 2 辺で 5+3=8。
        let p2 = ミンプラス累乗(&w, 2);
        assert_eq!(p2[0][2], 8);
        // 0→1→2→0 はちょうど 3 辺で 5+3+2=10。
        let p3 = ミンプラス累乗(&w, 3);
        assert_eq!(p3[0][0], 10);
    }

    #[test]
    fn 乱数でちょうどk辺最短と一致() {
        let mut seed = 0x2af1_60c0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..300 {
            let n = (乱数() as usize % 5) + 1; // 1..=5
            let mut w = vec![vec![無限; n]; n];
            for row in w.iter_mut() {
                for x in row.iter_mut() {
                    // 7 割の確率で辺を張る（重み 0..=9）。
                    if 乱数() % 10 < 7 {
                        *x = (乱数() % 10) as i64;
                    }
                }
            }
            for k in 0..=6usize {
                let p = ミンプラス累乗(&w, k);
                for 始点 in 0..n {
                    let 期待 = ちょうどk辺最短(&w, 始点, k);
                    for v in 0..n {
                        let 得 = p[始点][v].min(無限);
                        assert_eq!(
                            得,
                            期待[v].min(無限),
                            "n={n} k={k} 始点={始点} v={v} w={w:?}"
                        );
                    }
                }
            }
        }
    }
}
