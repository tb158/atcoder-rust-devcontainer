use cargo_snippet::snippet;

#[snippet("差分_制約系")]
#[snippet("差分_制約")]
#[snippet("DifferenceConstraints")]
/// **差分制約系**を解く。各制約 `(a, b, c)` は `x[b] − x[a] ≤ c` を表し、これらを同時に満たす整数割当
/// `x[0..変数数]` を 1 つ返す。実行不能（矛盾）なら `None`。`O(変数数 · 制約数)`。
///
/// 解はシフト不変（全体に定数を足しても成立）で、ここでは `max x` を 0 に寄せた解を返す。
/// 区間スケジューリングの順序制約や、座標差・時刻差の同時制約に使える。
fn 差分制約(変数数: usize, 制約: &[(usize, usize, isize)]) -> Option<Vec<isize>> {
    let n = 変数数;
    // 仮想始点から重み 0 で全頂点へ → 全頂点を距離 0 で初期化。
    let mut 距離 = vec![0isize; n];
    for 回 in 0..n {
        let mut 更新 = false;
        for &(a, b, c) in 制約 {
            if 距離[a] + c < 距離[b] {
                距離[b] = 距離[a] + c;
                更新 = true;
                if 回 == n - 1 {
                    return None; // n 回目でも緩和 → 負閉路 → 実行不能
                }
            }
        }
        if !更新 {
            break;
        }
    }
    Some(距離)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Floyd–Warshall による独立な負閉路検出オラクル（O(n³)）。
    fn 負閉路あり(n: usize, 制約: &[(usize, usize, isize)]) -> bool {
        const INF: isize = 1 << 60;
        let mut d = vec![vec![INF; n]; n];
        for (i, 行) in d.iter_mut().enumerate() {
            行[i] = 0;
        }
        for &(a, b, c) in 制約 {
            if c < d[a][b] {
                d[a][b] = c;
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if d[i][k] < INF && d[k][j] < INF {
                        let nd = d[i][k] + d[k][j];
                        if nd < d[i][j] {
                            d[i][j] = nd;
                        }
                    }
                }
            }
        }
        (0..n).any(|i| d[i][i] < 0)
    }

    fn 全制約成立(割当: &[isize], 制約: &[(usize, usize, isize)]) -> bool {
        制約.iter().all(|&(a, b, c)| 割当[b] - 割当[a] <= c)
    }

    #[test]
    fn 既知例() {
        // x1 - x0 <= 5, x0 - x1 <= -2  → 2 <= x1-x0 <= 5 で実行可能。
        let 制約 = vec![(0, 1, 5), (1, 0, -2)];
        let x = 差分制約(2, &制約).unwrap();
        assert!(全制約成立(&x, &制約));
        assert!((2..=5).contains(&(x[1] - x[0])));
        // x1-x0<=-1 かつ x0-x1<=-1 → 0<=-2 矛盾 → None。
        assert!(差分制約(2, &[(0, 1, -1), (1, 0, -1)]).is_none());
        // 制約なし → すべて 0。
        assert_eq!(差分制約(3, &[]).unwrap(), vec![0, 0, 0]);
    }

    #[test]
    fn 乱数で負閉路判定と割当が整合() {
        let mut seed = 0x1d93_b6a0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..4000 {
            let n = (乱数() as usize % 5) + 2; // 2..=6
            let mut 制約 = Vec::new();
            for a in 0..n {
                for b in 0..n {
                    if a != b && 乱数() % 3 == 0 {
                        let c = (乱数() % 21) as isize - 10; // -10..=10
                        制約.push((a, b, c));
                    }
                }
            }
            let 結果 = 差分制約(n, &制約);
            let 矛盾 = 負閉路あり(n, &制約);
            match 結果 {
                None => assert!(矛盾, "実行不能と判定したが負閉路なし 制約={制約:?}"),
                Some(x) => {
                    assert!(!矛盾, "解を返したが負閉路あり 制約={制約:?}");
                    assert!(全制約成立(&x, &制約), "解が制約違反 x={x:?} 制約={制約:?}");
                }
            }
        }
    }
}
