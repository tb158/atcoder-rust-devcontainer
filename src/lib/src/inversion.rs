use cargo_snippet::snippet;

#[snippet]
/// `b` を基準順序としたときの `a` の転倒数（= `a` を `b` に並べ替えるのに必要な隣接交換の最小回数）。
/// `a`・`b` は同じ要素からなる順列。BIT で O(N log N)。
fn 転倒数(a: &[usize], b: &[usize]) -> usize {
    use ac_library::FenwickTree;

    let mut bの逆置換 = vec![0; a.len()];
    for (i, &値) in b.iter().enumerate() {
        bの逆置換[値] = i;
    }

    let mut bit = FenwickTree::new(a.len(), 0_usize);
    a.iter().fold(0, |acc, &値| {
        let pos = bの逆置換[値];
        bit.add(pos, 1);
        acc + bit.sum(pos + 1..)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 転倒数の基本テスト() {
        // Bを基準(0, 1, 2)としたとき、A(2, 1, 0)は完全に逆順
        // ペアは (2,1), (2,0), (1,0) の3つすべてが「前 > 後」になる
        let a = vec![2, 1, 0];
        let b = vec![0, 1, 2];

        let 結果 = 転倒数(&a, &b);

        assert_eq!(結果, 3);
    }
}
