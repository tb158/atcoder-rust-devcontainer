use cargo_snippet::snippet;
#[snippet("BSGS")]
#[snippet("ベビーステップ_ジャイアントステップ")]
#[snippet("離散_対数")]
#[snippet("離散_対数_問題")]
use std::collections::HashMap;

#[snippet("離散_対数")]
#[snippet("BSGS")]
#[snippet("ベビーステップ_ジャイアントステップ")]
#[snippet("離散_対数_問題")]
/// 離散対数：`a^x ≡ b (mod m)` を満たす最小の非負整数 `x` を返す（無ければ `None`）。O(√m)。
///
/// Baby-step Giant-step 法。`gcd(a, m) ≠ 1`（`m` が合成数で `a` と互いに素でない）でも動く
/// 一般版。`a`・`b` は `mod m` 前で渡してよい。`m < 2^31` 程度を想定（乗算は u64 内で完結）。
///
/// 例：`a^x ≡ b (mod m)` の位数計算や、巡回的な遷移の到達ステップ数を求めるのに使う。
fn 離散対数(a: u64, b: u64, m: u64) -> Option<u64> {
    if m == 1 {
        return Some(0);
    }
    let a = a % m;
    let mut b = b % m;
    let mut m = m;
    let mut k = 1u64 % m; // ここまでに掛けた a の累積（縮約後の mod での値）
    let mut 加算 = 0u64;
    // gcd(a, m) > 1 の間、共通因数を割って正規化する
    loop {
        let g = num::integer::gcd(a, m);
        if g <= 1 {
            break;
        }
        if k == b {
            return Some(加算);
        }
        if b % g != 0 {
            return None;
        }
        b /= g;
        m /= g;
        k = k * (a / g) % m;
        加算 += 1;
    }
    let n = (m as f64).sqrt() as u64 + 1;
    // a^n mod m
    let mut an = 1u64 % m;
    for _ in 0..n {
        an = an * (a % m) % m;
    }
    // baby step: b·a^q (q = 0..=n) を記録（同じ値は大きい q で上書き）
    let mut テーブル: HashMap<u64, u64> = HashMap::new();
    let mut cur = b % m;
    for q in 0..=n {
        テーブル.insert(cur, q);
        cur = cur * (a % m) % m;
    }
    // giant step: k·(a^n)^p を走査
    let mut cur = k;
    for p in 1..=n {
        cur = cur * an % m;
        if let Some(&q) = テーブル.get(&cur) {
            return Some(n * p - q + 加算);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // a^x mod m を素朴に前から探索した最小解
    fn 素朴離散対数(a: u64, b: u64, m: u64) -> Option<u64> {
        let a = a % m;
        let b = b % m;
        let mut cur = 1u64 % m;
        // 前周期 + 周期 <= m + 64 で必ず巡回する
        for x in 0..(m + 64) {
            if cur == b {
                return Some(x);
            }
            cur = cur * a % m;
        }
        None
    }

    #[test]
    fn 総当たりと一致() {
        for m in 2..=40u64 {
            for a in 0..m {
                for b in 0..m {
                    let 期待 = 素朴離散対数(a, b, m);
                    let 実際 = 離散対数(a, b, m);
                    // 解の存在と最小性まで一致すること
                    assert_eq!(実際, 期待, "a={a}, b={b}, m={m}");
                }
            }
        }
    }

    #[test]
    fn 既知の例() {
        // 3^x ≡ 13 (mod 17) → x = 4 (3^4=81≡13)
        assert_eq!(離散対数(3, 13, 17), Some(4));
        // 解なし: 2^x ≡ 3 (mod 4)
        assert_eq!(離散対数(2, 3, 4), None);
        // x=0: a^0 = 1
        assert_eq!(離散対数(5, 1, 13), Some(0));
    }
}
