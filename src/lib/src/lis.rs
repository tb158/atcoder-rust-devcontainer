use cargo_snippet::snippet;

#[snippet("最長_増加_部分列")]
#[snippet("LIS")]
#[snippet("最長_増加")]
/// 最長増加部分列(LIS)の長さを返す。O(N log N)。
///
/// 要素型 `T` は順序比較できる型なら何でもよい（`i64`・`f64`・`&str` など `PartialOrd`）。
///
/// `狭義` で増加の意味を切り替える:
/// - `true`  … 狭義単調増加（同じ値は連続して採用できない。例 `[1,2,2,3]` の LIS は 3）
/// - `false` … 広義単調増加（同じ値も採用できる。例 `[1,2,2,3]` の LIS は 4）
///
/// 長さだけを返す（部分列そのものは復元しない）。
fn 最長増加部分列<T: PartialOrd + Copy>(配列: &[T], 狭義: bool) -> usize {
    // dp[k] = 「長さ k+1 の増加部分列」を作れる末尾値の最小値（昇順を保つ）
    let mut dp: Vec<T> = Vec::new();
    for &x in 配列 {
        let 位置 = if 狭義 {
            dp.partition_point(|&v| v < x)
        } else {
            dp.partition_point(|&v| v <= x)
        };
        if 位置 == dp.len() {
            dp.push(x);
        } else {
            dp[位置] = x;
        }
    }
    dp.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 狭義と広義() {
        let a = [1, 2, 2, 3];
        assert_eq!(最長増加部分列(&a, true), 3); // 1,2,3
        assert_eq!(最長増加部分列(&a, false), 4); // 1,2,2,3

        let b = [3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(最長増加部分列(&b, true), 4); // 1,4,5,9 など

        assert_eq!(最長増加部分列::<i64>(&[], true), 0);
    }
}
