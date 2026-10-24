use cargo_snippet::snippet;

#[snippet("回文_分割の_最小_カット_数")]
#[snippet("回文_分割")]
#[snippet("PalindromePartition")]
/// 文字列を**すべて回文になるように分割する最小カット数**を返す（分割数 − 1）。`O(N²)`。
/// 文字列全体がすでに回文なら 0。空文字列も 0。
///
/// [`manacher`](crate::manacher) は最長回文「部分文字列」、[`palindromic_tree`](crate::palindromic_tree) は
/// 回文の数え上げで、いずれも分割の最小化は扱わない。
fn 回文分割の最小カット数(s: &[u8]) -> usize {
    let n = s.len();
    if n == 0 {
        return 0;
    }
    // 回文[i][j] = s[i..=j] が回文。
    let mut 回文 = vec![vec![false; n]; n];
    for i in (0..n).rev() {
        for j in i..n {
            回文[i][j] = s[i] == s[j] && (j - i < 2 || 回文[i + 1][j - 1]);
        }
    }
    const INF: isize = 1 << 50;
    let mut カット = vec![INF; n + 1];
    カット[0] = -1; // 番兵：全体が回文なら 0 カットになる
    for i in 1..=n {
        for j in 0..i {
            if 回文[j][i - 1] && カット[j] + 1 < カット[i] {
                カット[i] = カット[j] + 1;
            }
        }
    }
    カット[n] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    // 全分割を再帰列挙して最小分割数（パート数）を返す素朴オラクル。
    fn 素朴最小パート数(s: &[u8]) -> usize {
        let n = s.len();
        if n == 0 {
            return 0;
        }
        let 回文 = |i: usize, j: usize| {
            let (mut a, mut b) = (i, j);
            while a < b {
                if s[a] != s[b] {
                    return false;
                }
                a += 1;
                b -= 1;
            }
            true
        };
        // dp[i] = 先頭 i 文字の最小パート数。
        let mut dp = vec![usize::MAX; n + 1];
        dp[0] = 0;
        for i in 1..=n {
            for j in 0..i {
                if dp[j] != usize::MAX && 回文(j, i - 1) {
                    dp[i] = dp[i].min(dp[j] + 1);
                }
            }
        }
        dp[n]
    }

    #[test]
    fn 既知例() {
        // "aab" → "aa"|"b" でカット 1。
        assert_eq!(回文分割の最小カット数(b"aab"), 1);
        // 全体が回文 → 0。
        assert_eq!(回文分割の最小カット数(b"racecar"), 0);
        assert_eq!(回文分割の最小カット数(b"a"), 0);
        // すべて異なる → n-1 カット。
        assert_eq!(回文分割の最小カット数(b"abcd"), 3);
        // 空。
        assert_eq!(回文分割の最小カット数(b""), 0);
        // "ababbbabbababa"（よく知られた例）→ 3。
        assert_eq!(回文分割の最小カット数(b"ababbbabbababa"), 3);
    }

    #[test]
    fn 乱数で素朴と一致() {
        let mut seed = 0x7c14_9ae0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..3000 {
            let len = (乱数() as usize % 11) + 1; // 1..=11
            let 字種 = (乱数() as usize % 3) + 2; // 2..=4 文字種
            let s: Vec<u8> = (0..len)
                .map(|_| b'a' + (乱数() as u8 % 字種 as u8))
                .collect();
            let カット = 回文分割の最小カット数(&s);
            let パート = 素朴最小パート数(&s);
            assert_eq!(カット + 1, パート, "s={:?}", String::from_utf8_lossy(&s));
        }
    }
}
