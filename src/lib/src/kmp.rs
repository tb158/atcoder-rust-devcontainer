use cargo_snippet::snippet;

#[snippet("KMP_法")]
#[snippet("KMP")]
#[snippet("接頭辞_関数")]
#[snippet("最小_周期")]
#[snippet("ボーダー")]
/// 接頭辞関数（KMP の failure 配列）。`pi[i]` は `s[0..=i]` の真の接頭辞かつ接尾辞となる
/// 最長文字列（ボーダー）の長さ。O(n)。`s` の要素は `PartialEq` であればよい。
///
/// ボーダー・周期性の解析や [`kmp検索`] の前処理に使う。`ac_library::z_algorithm` は Z 配列を
/// 返す別アルゴリズムで、用途（周期・ボーダーの直接取得）が異なる。
fn 接頭辞関数<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut pi = vec![0usize; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}

#[snippet("KMP_法")]
#[snippet("KMP")]
#[snippet("接頭辞_関数")]
#[snippet("最小_周期")]
#[snippet("ボーダー")]
/// `テキスト` 中に `パターン` が出現する開始位置（0-indexed）を全て昇順で返す。O(n + m)。
///
/// 重なり合う出現も全て列挙する。空パターンは「マッチなし」として空の `Vec` を返す。
fn kmp検索<T: PartialEq>(テキスト: &[T], パターン: &[T]) -> Vec<usize> {
    let m = パターン.len();
    let mut 結果 = Vec::new();
    if m == 0 {
        return 結果;
    }
    let pi = 接頭辞関数(パターン);
    let mut j = 0usize;
    for (i, c) in テキスト.iter().enumerate() {
        while j > 0 && *c != パターン[j] {
            j = pi[j - 1];
        }
        if *c == パターン[j] {
            j += 1;
        }
        if j == m {
            結果.push(i + 1 - m);
            j = pi[j - 1];
        }
    }
    結果
}

#[snippet("KMP_法")]
#[snippet("KMP")]
#[snippet("接頭辞_関数")]
#[snippet("最小_周期")]
#[snippet("ボーダー")]
/// `s` の最小周期長。`s` は長さ `周期` のブロックの繰り返しの接頭辞になる
/// （`s[i] == s[i + 周期]` が全ての有効な `i` で成り立つ最小の `周期`）。空列なら 0。
///
/// 例: `"aabaab"` → 3、`"aabaa"` → 3、`"abcabc"` → 3、`"abcd"` → 4。
/// `s.len() % 周期 == 0` のときだけ `s` はそのブロックの「ちょうど整数回」の繰り返し。
fn 最小周期<T: PartialEq>(s: &[T]) -> usize {
    let n = s.len();
    if n == 0 {
        return 0;
    }
    let pi = 接頭辞関数(s);
    n - pi[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 素朴検索(t: &[u8], p: &[u8]) -> Vec<usize> {
        if p.is_empty() || p.len() > t.len() {
            return Vec::new();
        }
        (0..=t.len() - p.len())
            .filter(|&i| &t[i..i + p.len()] == p)
            .collect()
    }

    #[test]
    fn 接頭辞関数の既知値() {
        // "abcabcd" → [0,0,0,1,2,3,0]
        let pi = 接頭辞関数(b"abcabcd");
        assert_eq!(pi, vec![0, 0, 0, 1, 2, 3, 0]);
        // "aabaaab" → [0,1,0,1,2,2,3]
        let pi2 = 接頭辞関数(b"aabaaab");
        assert_eq!(pi2, vec![0, 1, 0, 1, 2, 2, 3]);
    }

    #[test]
    fn 検索が素朴解と一致() {
        let テキスト = b"abababcababcabc";
        for パターン in [
            &b"ab"[..],
            b"abc",
            b"abcabc",
            b"a",
            b"c",
            b"xyz",
            b"abababcababcabc",
        ] {
            assert_eq!(
                kmp検索(テキスト, パターン),
                素朴検索(テキスト, パターン),
                "パターン={:?}",
                std::str::from_utf8(パターン).unwrap()
            );
        }
    }

    #[test]
    fn 重なる出現も列挙() {
        // "aaaa" 中の "aa" は 0,1,2
        assert_eq!(kmp検索(b"aaaa", b"aa"), vec![0, 1, 2]);
    }

    #[test]
    fn 空パターンは空() {
        assert_eq!(kmp検索(b"abc", b""), Vec::<usize>::new());
    }

    #[test]
    fn 最小周期の検証() {
        assert_eq!(最小周期(b"aabaab"), 3);
        assert_eq!(最小周期(b"aabaa"), 3);
        assert_eq!(最小周期(b"abcabc"), 3);
        assert_eq!(最小周期(b"abcd"), 4);
        assert_eq!(最小周期(b"aaaa"), 1);
        assert_eq!(最小周期(b""), 0);
    }
}
