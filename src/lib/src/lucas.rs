use crate::mod_pow::累乗_mod;
use cargo_snippet::snippet;

#[snippet("リュカの_定理", include = "累乗_mod")]
#[snippet("Lucas")]
#[snippet("二項_係数_mod_素数")]
#[snippet("LucasTheorem")]
#[snippet("素数_mod_の_組み合わせ")]
/// Lucas の定理による二項係数 `C(n, k) mod p`（`p` は素数）。`n`, `k` は最大 ~10^18 まで扱える。
///
/// `n`/`k` を p 進展開して桁ごとに掛け合わせるので、`n` が巨大でも `O(log_p n)`。
/// `0..p` の階乗を前計算するので `p` が小さめの素数のとき向き
/// （`p` が 10^9+7 級で `n <= max_n` に収まるなら [`crate::comb`]）。
struct リュカ {
    p: u64,
    階乗: Vec<u64>,
    逆階乗: Vec<u64>,
}

#[snippet("リュカの_定理")]
#[snippet("Lucas")]
#[snippet("二項_係数_mod_素数")]
#[snippet("LucasTheorem")]
#[snippet("素数_mod_の_組み合わせ")]
impl リュカ {
    /// 素数 `p` を法とする計算器を作る。`0..p` の階乗・逆階乗を前計算する（O(p)）。
    fn new(p: impl TryInto<u64>) -> Self {
        let p: u64 = p.try_into().ok().expect("リュカ: p が u64 に収まりません");
        assert!(p >= 2, "p must be a prime >= 2");
        let mut 階乗 = vec![1u64; p as usize];
        for i in 1..p as usize {
            階乗[i] = 階乗[i - 1] * i as u64 % p;
        }
        let mut 逆階乗 = vec![1u64; p as usize];
        // フェルマーの小定理で末尾の逆元を求め、後ろから埋める
        逆階乗[p as usize - 1] = 累乗_mod(階乗[p as usize - 1], p - 2, p);
        for i in (1..p as usize).rev() {
            逆階乗[i - 1] = 逆階乗[i] * i as u64 % p;
        }
        Self {
            p, 階乗, 逆階乗
        }
    }

    /// `0 <= a, b < p` での `C(a, b) mod p`。
    fn 小二項係数(&self, a: u64, b: u64) -> u64 {
        if b > a {
            return 0;
        }
        let a = a as usize;
        let b = b as usize;
        self.階乗[a] * self.逆階乗[b] % self.p * self.逆階乗[a - b] % self.p
    }

    /// `C(n, k) mod p`。`n`, `k` を p 進展開し、各桁の二項係数の積を取る。
    /// 引数は任意の整数型でよい（負の値は 0 を返す）。
    fn 二項係数(&self, n: impl TryInto<i128>, k: impl TryInto<i128>) -> u64 {
        let n: i128 = n.try_into().ok().expect("リュカ: n が i128 に収まりません");
        let k: i128 = k.try_into().ok().expect("リュカ: k が i128 に収まりません");
        if n < 0 || k < 0 {
            return 0;
        }
        let mut n = u64::try_from(n).expect("リュカ: n が u64 を超えています");
        let mut k = u64::try_from(k).expect("リュカ: k が u64 を超えています");
        let mut 結果 = 1u64;
        while n > 0 || k > 0 {
            let ni = n % self.p;
            let ki = k % self.p;
            if ki > ni {
                return 0;
            }
            結果 = 結果 * self.小二項係数(ni, ki) % self.p;
            n /= self.p;
            k /= self.p;
        }
        結果
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // パスカルの三角形で C(n,k) mod p を素朴計算（n <= 上限）
    fn パスカル(上限: usize, p: u64) -> Vec<Vec<u64>> {
        let mut c = vec![vec![0u64; 上限 + 1]; 上限 + 1];
        for n in 0..=上限 {
            c[n][0] = 1 % p;
            for k in 1..=n {
                c[n][k] = (c[n - 1][k - 1] + c[n - 1][k]) % p;
            }
        }
        c
    }

    #[test]
    fn 小さなnでパスカルと一致() {
        for &p in &[2u64, 3, 5, 7, 13] {
            let l = リュカ::new(p);
            let c = パスカル(60, p);
            for n in 0..=60u64 {
                for k in 0..=n {
                    assert_eq!(
                        l.二項係数(n, k),
                        c[n as usize][k as usize],
                        "p={p} n={n} k={k}"
                    );
                }
            }
        }
    }

    #[test]
    fn kがnを超えると0() {
        let l = リュカ::new(11);
        assert_eq!(l.二項係数(5, 7), 0);
    }

    #[test]
    fn 任意の整数型と負の値() {
        let l = リュカ::new(13);
        assert_eq!(l.二項係数(10usize, 3u32), l.二項係数(10u64, 3u64));
        assert_eq!(l.二項係数(-1, 2), 0);
        assert_eq!(l.二項係数(5, -2isize), 0);
    }

    #[test]
    fn 巨大なnでも計算できる() {
        // p=2 では C(n,k) mod 2 = 1 ⟺ k のビットが n のビットに含まれる（Lucas の帰結）
        let l = リュカ::new(2);
        let n = 0b1011_0100u64;
        for k in 0..=n.min(300) {
            let 期待 = if (k & n) == k { 1 } else { 0 };
            assert_eq!(l.二項係数(n, k), 期待, "n={n} k={k}");
        }
    }
}
