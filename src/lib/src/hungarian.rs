use cargo_snippet::snippet;

#[snippet("割当_問題")]
#[snippet("ハンガリアン_法")]
#[snippet("Hungarian")]
#[snippet("KuhnMunkres")]
#[snippet("最小_費用_割当")]
/// ハンガリアン法（Kuhn–Munkres）で割当問題（最小重み完全マッチング）を解く。`O(n²m)`。
/// `コスト[i][j]` を行 `i`・列 `j` の費用とし、各行に相異なる列を割り当てて総費用を最小化する。
///
/// 行数 `n <= 列数 m` であること（足りない列があると割り当て不能）。費用は負でもよい。
/// 返り値は `(最小総費用, 割当)`。`割当[i]` は行 `i` に割り当てられた列（0 始まり）。
fn 割当問題(コスト: &[Vec<i64>]) -> (i64, Vec<usize>) {
    let n = コスト.len();
    if n == 0 {
        return (0, Vec::new());
    }
    assert!(
        コスト[0].len() >= n,
        "ハンガリアン法: 列数は行数以上である必要があります"
    );
    let 行列 = pathfinding::matrix::Matrix::from_rows(コスト.iter().cloned())
        .expect("コストの全行は同じ列数であること");
    pathfinding::kuhn_munkres::kuhn_munkres_min(&行列)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 全順列による総当たり（行 i を相異なる列へ割り当てる最小費用）。
    fn 総当たり(コスト: &[Vec<i64>]) -> i64 {
        let n = コスト.len();
        let m = コスト[0].len();
        let mut 列: Vec<usize> = (0..m).collect();
        let mut 最良 = i64::MAX;
        // n 個の列を順列として選ぶ。
        fn 再帰(
            i: usize,
            n: usize,
            列: &mut Vec<usize>,
            コスト: &[Vec<i64>],
            現在: i64,
            最良: &mut i64,
        ) {
            if i == n {
                *最良 = (*最良).min(現在);
                return;
            }
            for k in i..列.len() {
                列.swap(i, k);
                再帰(i + 1, n, 列, コスト, 現在 + コスト[i][列[i]], 最良);
                列.swap(i, k);
            }
        }
        再帰(0, n, &mut 列, コスト, 0, &mut 最良);
        最良
    }

    #[test]
    fn 小さな例() {
        let コスト = vec![vec![4, 1, 3], vec![2, 0, 5], vec![3, 2, 2]];
        let (費用, 割当) = 割当問題(&コスト);
        // 検算: 割当の総費用が返り値と一致し、最適であること。
        let 実費: i64 = (0..3).map(|i| コスト[i][割当[i]]).sum();
        assert_eq!(費用, 実費);
        assert_eq!(費用, 総当たり(&コスト));
        // 割当は相異なる列
        let mut c = 割当.clone();
        c.sort_unstable();
        c.dedup();
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0xa11c_e5e5u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..400 {
            let n = (乱数() as usize % 6) + 1;
            let m = n + (乱数() as usize % 3); // m >= n
            let コスト: Vec<Vec<i64>> = (0..n)
                .map(|_| (0..m).map(|_| (乱数() % 41) as i64 - 20).collect())
                .collect();
            let (費用, 割当) = 割当問題(&コスト);
            let 実費: i64 = (0..n).map(|i| コスト[i][割当[i]]).sum();
            assert_eq!(費用, 実費, "費用が割当と不一致");
            assert_eq!(費用, 総当たり(&コスト), "n={n} m={m} コスト={コスト:?}");
            // 相異なる列
            let mut c = 割当.clone();
            c.sort_unstable();
            c.dedup();
            assert_eq!(c.len(), n, "列が重複");
        }
    }
}
