use cargo_snippet::snippet;

#[snippet("マナカー")]
#[snippet("Manacher")]
#[snippet("最長_回文")]
#[snippet("回文")]
/// Manacher 法。列に含まれる最長の回文部分列の長さを O(N) で返す。
///
/// 偶数長の回文にも対応するため、内部で文字の間に区切りを挟んだ列を作って処理する。
/// 要素型 `T` は等値比較できれば何でもよい（バイト列 `s.as_bytes()` のほか
/// `&[char]`・`&[isize]` などにも使える）。
fn マナカー最長回文<T: PartialEq>(s: &[T]) -> usize {
    if s.is_empty() {
        return 0;
    }
    // t = [区切り, s0, 区切り, s1, ..., s_{n-1}, 区切り]
    // 区切り(None)同士は等しく、各要素は Some(&c)。両端に番兵は置かない。
    let mut t: Vec<Option<&T>> = Vec::with_capacity(2 * s.len() + 1);
    for c in s {
        t.push(None); // 区切り
        t.push(Some(c));
    }
    t.push(None);

    let n = t.len();
    let mut 半径 = vec![0usize; n];
    let (mut 中心, mut 右端) = (0usize, 0usize);
    for i in 0..n {
        if i < 右端 {
            半径[i] = (右端 - i).min(半径[2 * 中心 - i]);
        }
        // 番兵が無いので、両端をはみ出さない範囲で回文を広げる。
        while i > 半径[i] && i + 半径[i] + 1 < n && t[i + 半径[i] + 1] == t[i - 半径[i] - 1]
        {
            半径[i] += 1;
        }
        if i + 半径[i] > 右端 {
            中心 = i;
            右端 = i + 半径[i];
        }
    }
    // 変換後の半径の最大値が、元の文字列での最長回文の長さに等しい
    半径.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 最長回文の長さ() {
        assert_eq!(マナカー最長回文(b"abacaba"), 7); // 全体が回文
        assert_eq!(マナカー最長回文(b"cbbd"), 2); // "bb"
        assert_eq!(マナカー最長回文(b"babad"), 3); // "bab" または "aba"
        assert_eq!(マナカー最長回文(b"a"), 1);
        assert_eq!(マナカー最長回文(b"abcde"), 1);
        assert_eq!(マナカー最長回文(b""), 0);
    }

    #[test]
    fn バイト以外の列でも動く() {
        // char 列でも isize 列でも同じ結果が得られる（PartialEq だけ要求）。
        let cs: Vec<char> = "abacaba".chars().collect();
        assert_eq!(マナカー最長回文(&cs), 7);
        assert_eq!(マナカー最長回文(&[1isize, 2, 3, 2, 1, 9]), 5); // [1,2,3,2,1]
    }
}
