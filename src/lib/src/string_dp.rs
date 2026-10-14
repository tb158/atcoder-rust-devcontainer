use cargo_snippet::snippet;

#[snippet("最長_共通_部分列")]
#[snippet("LCS")]
#[snippet("共通_部分列")]
/// 2つの列の最長共通部分列(LCS)の長さを返す。O(N*M)。
///
/// 文字列なら `s.chars().collect::<Vec<_>>()` や `s.as_bytes()` を渡す。
/// 連続している必要はなく、順序を保って共通に取れる最長の部分列の長さ。
fn 最長共通部分列<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 0..n {
        for (j, b要素) in b.iter().enumerate() {
            dp[i + 1][j + 1] = if a[i] == *b要素 {
                dp[i][j] + 1
            } else {
                dp[i][j + 1].max(dp[i + 1][j])
            };
        }
    }
    dp[n][m]
}

#[snippet("編集_距離")]
#[snippet("レーベンシュタイン_距離")]
#[snippet("レーベンシュタイン")]
/// レーベンシュタイン編集距離。`a` を `b` に変えるのに必要な
/// 「1文字の挿入・削除・置換」の最小回数を返す（各操作コスト 1）。O(N*M)。
fn 編集距離<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for (i, 行) in dp.iter_mut().enumerate() {
        行[0] = i; // a の先頭 i 文字を全削除
    }
    for (j, v) in dp[0].iter_mut().enumerate() {
        *v = j; // b の先頭 j 文字を全挿入
    }
    for i in 1..=n {
        for j in 1..=m {
            let 置換コスト = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + 置換コスト);
        }
    }
    dp[n][m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcsの長さ() {
        let a: Vec<char> = "axbycz".chars().collect();
        let b: Vec<char> = "abc".chars().collect();
        assert_eq!(最長共通部分列(&a, &b), 3);
        let c: Vec<char> = "abcde".chars().collect();
        let d: Vec<char> = "ace".chars().collect();
        assert_eq!(最長共通部分列(&c, &d), 3);
    }

    #[test]
    fn 編集距離の値() {
        let a: Vec<char> = "kitten".chars().collect();
        let b: Vec<char> = "sitting".chars().collect();
        assert_eq!(編集距離(&a, &b), 3);
        let c: Vec<char> = "abc".chars().collect();
        assert_eq!(編集距離(&c, &c), 0);
    }
}
