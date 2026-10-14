use crate::to_int::to_u64;
use cargo_snippet::snippet;

#[snippet("線形_基底")]
#[snippet("線型_基底")]
#[snippet("XOR_基底")]
#[snippet("xor_基底")]
/// XOR の線形基底。非負整数の集合に対し、その部分集合の XOR で作れる値全体を
/// 少数の基底ベクトルで表す。最大 XOR・ある値が作れるかの判定・作れる値の個数に使える。
/// 追加・判定は基底の次元（最大 60 程度）に比例。
struct 線形基底 {
    基底: Vec<u64>,
}

#[snippet("線形_基底")]
#[snippet("線型_基底")]
#[snippet("XOR_基底")]
#[snippet("xor_基底")]
impl Default for 線形基底 {
    fn default() -> Self {
        Self::new()
    }
}

#[snippet("線形_基底", include = "to_u64")]
#[snippet("線型_基底")]
#[snippet("XOR_基底")]
#[snippet("xor_基底")]
impl 線形基底 {
    /// 空の基底を作る。
    fn new() -> Self {
        Self { 基底: Vec::new() }
    }

    /// 値 `x` を追加する。基底の次元が増えたら `true`、既存基底で表現可能なら `false`。
    fn 追加(&mut self, x: impl TryInto<u64>) -> bool {
        let mut x = to_u64(x);
        for &b in &self.基底 {
            let t = x ^ b;
            if t < x {
                x = t;
            }
        }
        if x == 0 {
            return false;
        }
        self.基底.push(x);
        // 最上位ビットの降順を保つよう、追加した要素を前へ移動
        let mut i = self.基底.len() - 1;
        while i > 0 && self.基底[i] > self.基底[i - 1] {
            self.基底.swap(i, i - 1);
            i -= 1;
        }
        true
    }

    /// 部分集合 XOR で作れる最大値。
    fn 最大値(&self) -> usize {
        let mut res = 0u64;
        for &b in &self.基底 {
            res = res.max(res ^ b);
        }
        res as usize
    }

    /// `x` が部分集合 XOR で作れるか。
    fn 表現可能(&self, x: impl TryInto<u64>) -> bool {
        let mut x = to_u64(x);
        for &b in &self.基底 {
            let t = x ^ b;
            if t < x {
                x = t;
            }
        }
        x == 0
    }

    /// 基底の次元。作れる相異なる値は `2^次元` 通り。
    fn 次元(&self) -> usize {
        self.基底.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 最大xorと判定() {
        let mut b = 線形基底::new();
        for &x in &[3u64, 10, 5] {
            b.追加(x);
        }
        // {3,10,5} の部分集合 XOR の最大は 3^10^5? を含む候補から
        // 実際の最大は 3^10=9, 3^5=6, 10^5=15, 3^10^5=12, 単体 10 → 15
        assert_eq!(b.最大値(), 15);
        assert!(b.表現可能(0)); // 空集合
        assert!(b.表現可能(15));
        assert!(b.表現可能(3 ^ 10));
        assert_eq!(b.次元(), 3);
    }

    #[test]
    fn 従属な値は次元を増やさない() {
        let mut b = 線形基底::new();
        assert!(b.追加(6));
        assert!(b.追加(3));
        // 6 ^ 3 = 5 は既存基底で作れる
        assert!(!b.追加(5));
        assert_eq!(b.次元(), 2);
    }
}
