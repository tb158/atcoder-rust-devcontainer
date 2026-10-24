use cargo_snippet::snippet;

#[snippet("リンドン_分解")]
#[snippet("Duval")]
#[snippet("LyndonFactorization")]
/// リンドン分解（Duval のアルゴリズム）。列 `s` を、辞書順で非増加に並ぶリンドン語
/// `w1 >= w2 >= ... >= wk` の連結に一意分解し、各因子の半開区間 `[開始, 終了)` を `O(n)` で返す。
///
/// リンドン語とは「自身が、その全ての真の接尾辞より辞書順で真に小さい」空でない列。最小回転や
/// suffix array の補助、文字列周期性の解析に使う。比較できる任意の要素列に使える。
fn リンドン分解<T: Ord>(s: &[T]) -> Vec<(usize, usize)> {
    let n = s.len();
    let mut 結果 = Vec::new();
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        let mut k = i;
        while j < n && s[k] <= s[j] {
            if s[k] < s[j] {
                k = i; // 等しくない箇所が来たら基準を先頭へ戻す
            } else {
                k += 1;
            }
            j += 1;
        }
        // [i, k] が周期、長さ j-k のリンドン語を切り出していく。
        while i <= k {
            結果.push((i, i + (j - k)));
            i += j - k;
        }
    }
    結果
}

#[snippet("最小_回転")]
#[snippet("MinimumRotation")]
#[snippet("Booth")]
#[snippet("最小_表現")]
/// 最小回転（Booth のアルゴリズム）。列 `s` の巡回シフトのうち辞書順最小になるものの開始位置を
/// `O(n)` で返す。`s[k..] ++ s[..k]` が最小の回転。空列なら `0`。
///
/// 文字列の「最小表現（canonical form）」を取って同型判定する用途で頻出。
/// 比較できる任意の要素列に使える。
fn 最小回転<T: Ord>(s: &[T]) -> usize {
    let n = s.len();
    if n == 0 {
        return 0;
    }
    let c = |idx: usize| &s[idx % n];
    let mut f: Vec<isize> = vec![-1; 2 * n]; // 失敗関数（長さ 2n の連結列上）
    let mut k: usize = 0; // 現時点の最小回転開始位置
    for j in 1..2 * n {
        let sj = c(j);
        let mut i = f[j - k - 1];
        while i != -1 && sj != c(k + i as usize + 1) {
            if sj < c(k + i as usize + 1) {
                k = j - i as usize - 1;
            }
            i = f[i as usize];
        }
        if sj != c(k + (i + 1) as usize) {
            if sj < c(k + (i + 1) as usize) {
                k = j;
            }
            f[j - k] = -1;
        } else {
            f[j - k] = i + 1;
        }
    }
    k
}

#[cfg(test)]
mod tests {
    use super::*;

    // w が真の接尾辞すべてより真に小さい（＝リンドン語）か。
    fn リンドン語か(w: &[u8]) -> bool {
        if w.is_empty() {
            return false;
        }
        for p in 1..w.len() {
            if w[p..] <= *w {
                return false;
            }
        }
        true
    }

    #[test]
    fn 既知例() {
        // "banana" -> b | an | an | a
        let s = b"banana";
        let f = リンドン分解(s);
        let 部分: Vec<&[u8]> = f.iter().map(|&(l, r)| &s[l..r]).collect();
        assert_eq!(部分, vec![&b"b"[..], &b"an"[..], &b"an"[..], &b"a"[..]]);
    }

    #[test]
    fn 分解の性質を乱数検証() {
        let mut seed = 0x9e37_79b9u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..500 {
            let n = (乱数() as usize % 20) + 1;
            let s: Vec<u8> = (0..n).map(|_| (乱数() % 3) as u8 + b'a').collect();
            let f = リンドン分解(&s);
            // 連結が元と一致
            let mut 連結 = Vec::new();
            for &(l, r) in &f {
                連結.extend_from_slice(&s[l..r]);
            }
            assert_eq!(連結, s, "s={s:?}");
            // 各因子がリンドン語、かつ非増加
            let 語: Vec<&[u8]> = f.iter().map(|&(l, r)| &s[l..r]).collect();
            for w in &語 {
                assert!(リンドン語か(w), "not lyndon {w:?} in s={s:?}");
            }
            for w in 語.windows(2) {
                assert!(w[0] >= w[1], "not non-increasing in s={s:?}");
            }
        }
    }

    #[test]
    fn 最小回転を乱数検証() {
        let mut seed = 0x2545_f491u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..500 {
            let n = (乱数() as usize % 20) + 1;
            let s: Vec<u8> = (0..n).map(|_| (乱数() % 3) as u8 + b'a').collect();
            let k = 最小回転(&s);
            let 回転 = |t: usize| -> Vec<u8> { (0..n).map(|x| s[(t + x) % n]).collect() };
            let 候補 = 回転(k);
            // 全回転の最小と一致すること
            let 最小 = (0..n).map(回転).min().unwrap();
            assert_eq!(候補, 最小, "s={s:?} k={k}");
        }
    }
}
