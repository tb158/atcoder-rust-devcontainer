use cargo_snippet::snippet;

#[snippet("クヌース_最適化")]
#[snippet("KnuthOptimization")]
#[snippet("区間_DP_最適化")]
#[snippet("最適_二分探索木")]
#[snippet("石の_合体")]
/// クヌース（Knuth–Yao）最適化。区間 DP `dp[i][j] = 区間コスト(i, j) + min_{i<=k<j} (dp[i][k] + dp[k+1][j])`
/// を、最適分割点の単調性 `opt[i][j-1] <= opt[i][j] <= opt[i+1][j]` を用いて `O(n^2)` で解く。
///
/// `区間コスト(i, j)` が四角不等式（QI）と区間単調性を満たすとき正しい（石の合体・最適二分探索木など、
/// 区間長に関する凸なコスト）。素朴な区間 DP は `O(n^3)`。`n` 要素を 1 つに併合する総コストを返す。
/// [`crate::divide_and_conquer_dp`]（層状の遷移）とは別物。
fn クヌース最適化<F: Fn(usize, usize) -> i64>(n: usize, 区間コスト: F) -> i64 {
    if n == 0 {
        return 0;
    }
    let mut dp = vec![vec![0i64; n]; n];
    let mut opt = vec![vec![0usize; n]; n];
    for (i, 行) in opt.iter_mut().enumerate() {
        行[i] = i;
    }
    // 区間長 長 = j - i を 1 から n-1 まで広げる。
    for 長 in 1..n {
        for i in 0..n - 長 {
            let j = i + 長;
            let 下 = opt[i][j - 1];
            let 上 = opt[i + 1][j].min(j - 1);
            let mut 最良 = i64::MAX;
            let mut 最良k = 下;
            for k in 下..=上 {
                let v = dp[i][k] + dp[k + 1][j];
                if v < 最良 {
                    最良 = v;
                    最良k = k;
                }
            }
            dp[i][j] = 最良 + 区間コスト(i, j);
            opt[i][j] = 最良k;
        }
    }
    dp[0][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    // 素朴な O(n^3) 区間 DP。
    fn 素朴<F: Fn(usize, usize) -> i64>(n: usize, 区間コスト: F) -> i64 {
        if n == 0 {
            return 0;
        }
        let mut dp = vec![vec![0i64; n]; n];
        for 長 in 1..n {
            for i in 0..n - 長 {
                let j = i + 長;
                let mut 最良 = i64::MAX;
                for k in i..j {
                    最良 = 最良.min(dp[i][k] + dp[k + 1][j]);
                }
                dp[i][j] = 最良 + 区間コスト(i, j);
            }
        }
        dp[0][n - 1]
    }

    #[test]
    fn 石の合体を乱数検証() {
        let mut seed = 0x7e57_1234u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..300 {
            let n = (乱数() as usize % 30) + 1;
            let a: Vec<i64> = (0..n).map(|_| (乱数() % 20) as i64 + 1).collect();
            // 累積和で区間コスト = a[i..=j] の総和（QI を満たす）。
            let mut s = vec![0i64; n + 1];
            for i in 0..n {
                s[i + 1] = s[i] + a[i];
            }
            let 区間コスト = |i: usize, j: usize| s[j + 1] - s[i];
            assert_eq!(
                クヌース最適化(n, 区間コスト),
                素朴(n, 区間コスト),
                "a={a:?}"
            );
        }
    }

    #[test]
    fn 単一要素は0() {
        assert_eq!(クヌース最適化(1, |_, _| 100), 0);
        assert_eq!(クヌース最適化(0, |_, _| 100), 0);
    }
}
