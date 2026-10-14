// 数え上げ表（分割数・スターリング数・ベル数・モンモール数）— ModInt 前計算
//
// Comb（階乗・逆階乗の前計算）を必要としない数え上げ DP 表。
// mod は ac_library::ModInt のグローバル modulus（既定 998_244_353。
// 変更するときは Mint::set_modulus(m) か Comb::new(m, _) を先に呼ぶ）。

use cargo_snippet::snippet;

#[snippet("分割数")]
#[snippet("分割数_表")]
#[snippet("整数の_分割")]
#[snippet("五角数_定理")]
#[snippet("第二種_スターリング_数")]
#[snippet("スターリング_数")]
#[snippet("ベル_数")]
#[snippet("集合の_分割")]
#[snippet("第一種_スターリング_数")]
#[snippet("StirlingFirst")]
#[snippet("符号なし_第一種_スターリング_数")]
#[snippet("巡回_置換の_数")]
#[snippet("モンモール_数")]
#[snippet("Derangement")]
#[snippet("撹乱_順列")]
#[snippet("完全_順列")]
#[snippet("Montmort")]
#[snippet("分割数_k")]
#[snippet("分割数_k個_以下")]
#[snippet("k個_以下への_分割")]
use ac_library::ModInt as Mint;

#[snippet("分割数")]
#[snippet("分割数_表")]
#[snippet("整数の_分割")]
#[snippet("五角数_定理")]
/// 分割数 p(0..=n) を前計算して返す。`p(k)` は整数 `k` を順序を無視して正整数の和に
/// 分ける方法の総数。五角数定理を用いて O(n√n)。
///
/// mod は `Mint`（ac_library::ModInt）のグローバル modulus（既定 998_244_353、
/// 変更は `Mint::set_modulus`）。Comb は不要。
///
/// `p(0)=1, p(1)=1, p(2)=2, p(3)=3, p(4)=5, p(5)=7, ...`。u64 なら n ≥ 417 で溢れる。
fn 分割数表(n: usize) -> Vec<Mint> {
    let mut p = vec![Mint::new(0); n + 1];
    p[0] = Mint::new(1);
    for i in 1..=n {
        let mut 和 = Mint::new(0);
        let mut k = 1usize;
        loop {
            let g1 = k * (3 * k - 1) / 2;
            if g1 > i {
                break;
            }
            let mut 項 = p[i - g1];
            let g2 = k * (3 * k + 1) / 2;
            if g2 <= i {
                項 += p[i - g2];
            }
            if k % 2 == 1 {
                和 += 項;
            } else {
                和 -= 項;
            }
            k += 1;
        }
        p[i] = 和;
    }
    p
}

#[snippet("分割数_k")]
#[snippet("分割数_k個_以下")]
#[snippet("k個_以下への_分割")]
/// 個数制限つき分割数の表 `p[i][j]`（`0..=n` 行・`0..=k` 列）を返す。`p[i][j]` は `i` を
/// `j` 個の非負整数の和（順序は無視）に分ける方法の数。`p[i][j] = p[i][j-1] + p[i-j][j]`
/// で O(n·k)。共役をとれば「各部分が `j` 以下の分割数」でもある。
///
/// `p[4] = [0, 1, 3, 4, 5]`（4 = 4 / 3+1 / 2+2 / 2+1+1 / 1+1+1+1）。
/// u64 で持つなら `j` 次第（`j ≤ 10` は n=1000 でも安全、`j = 20` は n ≥ 658、
/// `j ≥ n` は [`分割数表`] と同じく n ≥ 417 で溢れる）。
fn 分割数表_k個以下(n: usize, k: usize) -> Vec<Vec<Mint>> {
    let mut p = vec![vec![Mint::new(0); k + 1]; n + 1];
    p[0] = vec![Mint::new(1); k + 1]; // 0 は「何も足さない」1 通り、i > 0 は 0 個では作れない
    for i in 1..=n {
        for j in 1..=k {
            p[i][j] = p[i][j - 1] + if i >= j { p[i - j][j] } else { Mint::new(0) };
        }
    }
    p
}

#[snippet("分割_列挙")]
#[snippet("整数_分割の_全列挙")]
#[snippet("k個_以下への_分割を_列挙")]
/// `n` を `k` 個以下の正整数の和に分ける組を、各組を降順にして全列挙する（個数は
/// [`分割数表_k個以下`]。ここだけ値ではなく組そのものを返すので usize）。
///
/// `分割列挙(4, 3) = [[4], [3, 1], [2, 2], [2, 1, 1]]`（辞書順の降順）。
/// 制限なし（`k >= n`）だと組数は p(n) で、n=50 は 20 万・27MB・11ms、n=60 は 97 万・143MB・0.13s、
/// n=70 は 409 万・660MB・0.5s。n ≤ 60 までが実用の目安（それ以上は列挙せず数えること）。
fn 分割列挙(n: usize, k: usize) -> Vec<Vec<usize>> {
    fn 再帰(
        残り: usize,
        最大: usize,
        残り個数: usize,
        組: &mut Vec<usize>,
        結果: &mut Vec<Vec<usize>>,
    ) {
        if 残り == 0 {
            結果.push(組.clone());
            return;
        }
        if 残り個数 == 0 {
            return;
        }
        for 部分 in (1..=最大.min(残り)).rev() {
            組.push(部分);
            再帰(残り - 部分, 部分, 残り個数 - 1, 組, 結果);
            組.pop();
        }
    }
    let mut 結果 = Vec::new();
    再帰(n, n, k, &mut Vec::new(), &mut 結果);
    結果
}

#[snippet("第二種_スターリング_数")]
#[snippet("スターリング_数")]
#[snippet("ベル_数")]
#[snippet("集合の_分割")]
/// 第二種スターリング数 S(n, k) の表（`0..=n` 行、`0..=n` 列）を前計算して返す。O(n^2)。
///
/// mod は `Mint`（ac_library::ModInt）のグローバル modulus。Comb は不要。
///
/// `S(n,k)` は `n` 要素を空でない `k` 個のグループに分割する方法の数。u64 なら n ≥ 27 で溢れる。
fn 第二種スターリング数表(n: usize) -> Vec<Vec<Mint>> {
    let mut s = vec![vec![Mint::new(0); n + 1]; n + 1];
    s[0][0] = Mint::new(1);
    for i in 1..=n {
        for k in 1..=i {
            s[i][k] = Mint::new(k) * s[i - 1][k] + s[i - 1][k - 1];
        }
    }
    s
}

#[snippet("ベル_数")]
/// ベル数 B(0..=n) を前計算して返す。`B(n)` は `n` 要素集合の分割の総数。O(n^2)。
/// u64 なら n ≥ 26 で溢れる。
///
/// mod は `Mint`（ac_library::ModInt）のグローバル modulus。Comb は不要。
fn ベル数表(n: usize) -> Vec<Mint> {
    let s = 第二種スターリング数表(n);
    s.iter()
        .map(|行| 行.iter().fold(Mint::new(0), |acc, &x| acc + x))
        .collect()
}

#[snippet("第一種_スターリング_数")]
#[snippet("StirlingFirst")]
#[snippet("符号なし_第一種_スターリング_数")]
#[snippet("巡回_置換の_数")]
/// 符号なし第一種スターリング数 `c(n, k)` の表（`0..=n` 行・`0..=n` 列）を前計算して返す。O(n^2)。
///
/// mod は `Mint`（ac_library::ModInt）のグローバル modulus。Comb は不要。
///
/// `c(n, k)` は `n` 要素の順列のうち巡回置換（サイクル）がちょうど `k` 個のものの数。
/// 第二種スターリング数（集合の分割）とは別物（こちらはサイクル数の分布）。u64 なら n ≥ 22 で溢れる。
fn 第一種スターリング数表(n: usize) -> Vec<Vec<Mint>> {
    let mut c = vec![vec![Mint::new(0); n + 1]; n + 1];
    c[0][0] = Mint::new(1);
    for i in 1..=n {
        for k in 1..=i {
            c[i][k] = c[i - 1][k - 1] + Mint::new(i - 1) * c[i - 1][k];
        }
    }
    c
}

#[snippet("モンモール_数")]
#[snippet("Derangement")]
#[snippet("撹乱_順列")]
#[snippet("完全_順列")]
#[snippet("Montmort")]
/// モンモール数（完全順列＝撹乱順列の数）`D(0..=n)` を前計算して返す。O(n)。
///
/// mod は `Mint`（ac_library::ModInt）のグローバル modulus。Comb は不要。
///
/// `D(n)` は `1..=n` のどの位置も自分自身を指さない順列の数。u64 なら n ≥ 21 で溢れる。
fn モンモール数(n: usize) -> Vec<Mint> {
    let mut d = vec![Mint::new(0); n + 1];
    d[0] = Mint::new(1);
    for i in 2..=n {
        d[i] = Mint::new(i - 1) * (d[i - 1] + d[i - 2]);
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    // 全順列を生成し、サイクル数の分布と無不動点（撹乱）順列数を数える。
    fn 全順列で集計(n: usize) -> (Vec<u32>, u32) {
        let mut サイクル分布 = vec![0u32; n + 1]; // サイクル分布[k] = サイクル数 k の順列数
        let mut 撹乱 = 0u32;
        let mut p: Vec<usize> = (0..n).collect();
        fn 再帰(i: usize, n: usize, p: &mut Vec<usize>, 分布: &mut [u32], 撹乱: &mut u32) {
            if i == n {
                let mut 見た = vec![false; n];
                let mut 個数 = 0;
                for s in 0..n {
                    if !見た[s] {
                        個数 += 1;
                        let mut j = s;
                        while !見た[j] {
                            見た[j] = true;
                            j = p[j];
                        }
                    }
                }
                分布[個数] += 1;
                if (0..n).all(|x| p[x] != x) {
                    *撹乱 += 1;
                }
                return;
            }
            for j in i..n {
                p.swap(i, j);
                再帰(i + 1, n, p, 分布, 撹乱);
                p.swap(i, j);
            }
        }
        if n == 0 {
            サイクル分布[0] = 1;
            return (サイクル分布, 1); // 空順列は撹乱とみなす
        }
        再帰(0, n, &mut p, &mut サイクル分布, &mut 撹乱);
        (サイクル分布, 撹乱)
    }

    // ModInt の mod はグローバルなので、comb のテストと直列化する（ロックは comb.rs 側）
    #[test]
    fn counting_cases() {
        let _guard = crate::comb::MODINT_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        Mint::set_modulus(998_244_353);

        // 分割数 1,1,2,3,5,7,11,15,22,30,42
        let p: Vec<u32> = 分割数表(10).iter().map(|x| x.val()).collect();
        assert_eq!(p, vec![1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42]);
        // 分割数は素朴漸化式（種類無制限ナップサック DP）と一致
        {
            let n = 60usize;
            let mut dp = vec![Mint::new(0); n + 1];
            dp[0] = Mint::new(1);
            for 部品 in 1..=n {
                for j in 部品..=n {
                    dp[j] = dp[j] + dp[j - 部品];
                }
            }
            assert_eq!(分割数表(n), dp);
        }

        // 個数制限つき分割数: 共役（各部分が j 以下のナップサック DP）と一致し、
        // j >= n の列は制限なしの分割数と一致。分割列挙の個数とも一致する。
        {
            let n = 15usize;
            let 表 = 分割数表_k個以下(n, n);
            for j in 0..=n {
                let mut dp = vec![Mint::new(0); n + 1];
                dp[0] = Mint::new(1);
                for 部品 in 1..=j {
                    for i in 部品..=n {
                        dp[i] = dp[i] + dp[i - 部品];
                    }
                }
                for i in 0..=n {
                    assert_eq!(表[i][j], dp[i], "p({i},{j})");
                    let 列挙 = 分割列挙(i, j);
                    assert_eq!(列挙.len() as u32, 表[i][j].val(), "列挙 p({i},{j})");
                    let 重複なし: std::collections::HashSet<_> = 列挙.iter().cloned().collect();
                    assert_eq!(重複なし.len(), 列挙.len(), "dup p({i},{j})");
                    for 組 in &列挙 {
                        assert!(組.len() <= j && 組.iter().all(|&x| x > 0), "{組:?}");
                        assert!(組.windows(2).all(|w| w[0] >= w[1]), "desc {組:?}");
                        assert_eq!(組.iter().sum::<usize>(), i, "sum {組:?}");
                    }
                }
            }
            assert_eq!(表[n][n], 分割数表(n)[n]);
            assert_eq!(
                分割列挙(4, 3),
                vec![vec![4], vec![3, 1], vec![2, 2], vec![2, 1, 1]]
            );
        }

        // 第二種スターリング数: S(n,1)=1, S(n,n)=1
        let s = 第二種スターリング数表(5);
        for (n, 行) in s.iter().enumerate().skip(1) {
            assert_eq!(行[1].val(), 1);
            assert_eq!(行[n].val(), 1);
        }
        assert_eq!(s[4][2].val(), 7);
        assert_eq!(s[5][2].val(), 15);
        assert_eq!(s[5][3].val(), 25);

        // ベル数 1,1,2,5,15,52,203,877,4140
        let b: Vec<u32> = ベル数表(8).iter().map(|x| x.val()).collect();
        assert_eq!(b, vec![1, 1, 2, 5, 15, 52, 203, 877, 4140]);

        // 第一種スターリング数: 全順列のサイクル数分布と一致、行和は n!
        let mut 階乗 = Mint::new(1);
        for n in 0..=7usize {
            if n > 0 {
                階乗 *= Mint::new(n);
            }
            let 表 = 第一種スターリング数表(n);
            let (分布, _) = 全順列で集計(n);
            for k in 0..=n {
                assert_eq!(表[n][k].val(), 分布[k], "c({n},{k})");
            }
            let 和 = 表[n].iter().fold(Mint::new(0), |acc, &x| acc + x);
            assert_eq!(和, 階乗, "row sum n={n}");
        }

        // モンモール数 1,0,1,2,9,44,265,1854、全順列の撹乱数と一致
        let d = モンモール数(7);
        let d_vals: Vec<u32> = d.iter().map(|x| x.val()).collect();
        assert_eq!(d_vals, vec![1, 0, 1, 2, 9, 44, 265, 1854]);
        for n in 1..=7usize {
            let (_, 撹乱) = 全順列で集計(n);
            assert_eq!(d[n].val(), 撹乱, "D({n})");
        }
    }
}

// https://qiita.com/drken/items/f2ea4b58b0d21621bd51
