use cargo_snippet::snippet;

#[snippet]
/// 部分列オートマトン（部分列判定用の遷移表）。
/// 文字列（0~25 の usize に変換済みのスライス）を受け取り、
/// next[l][a] = l 文字目以降で最初に文字 a が出現する位置 + 1（存在しなければ None）を返す。O(26N)。
fn 部分列オートマトン(文字列: &[usize]) -> Vec<Vec<Option<usize>>> {
    // (列の長さ + 1) × 26 の二次元配列を None で初期化
    let mut next = vec![vec![None; 26]; 文字列.len() + 1];

    // 後ろから前に向かって更新していく
    for (idx, &文字) in 文字列.iter().enumerate().rev() {
        for j in 0..26 {
            next[idx][j] = next[idx + 1][j];
        }
        next[idx][文字] = Some(idx + 1);
    }

    next
}
