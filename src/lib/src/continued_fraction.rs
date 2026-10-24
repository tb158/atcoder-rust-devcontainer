use cargo_snippet::snippet;
#[snippet("SternBrocot")]
#[snippet("最良_近似")]
#[snippet("有理数_近似")]
#[snippet("連分数")]
use std::cmp::Ordering;

#[snippet("連分数")]
#[snippet("ContinuedFraction")]
#[snippet("最良_近似")]
#[snippet("有理数_近似")]
#[snippet("SternBrocot")]
/// 非負の有理数 `分子 / 分母`（`分母 > 0`）の正則連分数展開 `[a0; a1, a2, …]` を返す。
/// ユークリッドの互除法そのもの。`O(log 分母)`。
fn 連分数展開(分子: impl TryInto<isize>, 分母: impl TryInto<isize>) -> Vec<isize> {
    let mut 分子 = 分子
        .try_into()
        .ok()
        .expect("連分数展開: 引数が isize に収まりません");
    let mut 分母 = 分母
        .try_into()
        .ok()
        .expect("連分数展開: 引数が isize に収まりません");
    assert!(分母 > 0 && 分子 >= 0, "分子 >= 0, 分母 > 0");
    let mut 係数 = Vec::new();
    while 分母 != 0 {
        係数.push(分子 / 分母);
        let r = 分子 % 分母;
        分子 = 分母;
        分母 = r;
    }
    係数
}

#[snippet("連分数")]
#[snippet("ContinuedFraction")]
#[snippet("最良_近似")]
#[snippet("有理数_近似")]
#[snippet("SternBrocot")]
/// 連分数係数列から収束分数（convergent）`(分子, 分母)` の列を返す。
/// `h_i = a_i·h_{i-1} + h_{i-2}`, `k_i = a_i·k_{i-1} + k_{i-2}`。各収束分数は既約。
fn 収束分数(係数: &[isize]) -> Vec<(isize, isize)> {
    let mut 列 = Vec::with_capacity(係数.len());
    let (mut h2, mut h1) = (0isize, 1isize); // h_{-2}, h_{-1}
    let (mut k2, mut k1) = (1isize, 0isize); // k_{-2}, k_{-1}
    for &a in 係数 {
        let h = a * h1 + h2;
        let k = a * k1 + k2;
        列.push((h, k));
        h2 = h1;
        h1 = h;
        k2 = k1;
        k1 = k;
    }
    列
}

#[snippet("連分数")]
#[snippet("最良_近似")]
#[snippet("有理数_近似")]
#[snippet("SternBrocot")]
/// `分子/分母` への近さ `|分子/分母 - a/b|` を厳密に比較する内部関数（小さいほど近い）。
fn 距離cmp(分子: isize, 分母: isize, x: (isize, isize), y: (isize, isize)) -> Ordering {
    let (p, q) = (分子 as i128, 分母 as i128);
    let dx = (p * x.1 as i128 - x.0 as i128 * q).abs() * y.1 as i128;
    let dy = (p * y.1 as i128 - y.0 as i128 * q).abs() * x.1 as i128;
    dx.cmp(&dy)
}

#[snippet("連分数")]
#[snippet("最良_近似")]
#[snippet("有理数_近似")]
#[snippet("SternBrocot")]
/// `x = 分子/分母` に最も近い有理数 `a/b`（`1 <= b <= 分母上限`）を `O(log 分母)` で求める。
///
/// 連分数の収束分数と半収束分数（semiconvergent）のうち、分母が上限以下で最も近いものを返す。
/// 距離が等しいときは分母が小さい方。`x` が `分母上限` 以下の分母で厳密表現できればそれを返す。
/// Stern–Brocot 木上の最良近似。
fn 最良有理近似(
    分子: impl TryInto<isize>,
    分母: impl TryInto<isize>,
    分母上限: impl TryInto<isize>,
) -> (isize, isize) {
    let 分子 = 分子
        .try_into()
        .ok()
        .expect("最良有理近似: 引数が isize に収まりません");
    let 分母 = 分母
        .try_into()
        .ok()
        .expect("最良有理近似: 引数が isize に収まりません");
    let 分母上限 = 分母上限
        .try_into()
        .ok()
        .expect("最良有理近似: 引数が isize に収まりません");
    assert!(分母 > 0 && 分子 >= 0 && 分母上限 >= 1);
    let (mut a, mut b) = (分子, 分母); // 走査中の剰余分数
    let (mut h2, mut h1) = (0isize, 1isize);
    let (mut k2, mut k1) = (1isize, 0isize);
    loop {
        if b == 0 {
            return (h1, k1); // 厳密表現に到達
        }
        let ai = a / b;
        let r = a % b;
        let k = ai * k1 + k2;
        if k > 分母上限 {
            // a' = (分母上限 - k2)/k1 を係数とする半収束分数と、直前の収束分数を比較。
            let ap = (分母上限 - k2) / k1;
            let 半 = (ap * h1 + h2, ap * k1 + k2);
            let 前 = (h1, k1);
            return match 距離cmp(分子, 分母, 半, 前) {
                Ordering::Less => 半,
                Ordering::Greater => 前,
                Ordering::Equal => {
                    if 半.1 <= 前.1 {
                        半
                    } else {
                        前
                    }
                }
            };
        }
        let h = ai * h1 + h2;
        h2 = h1;
        h1 = h;
        k2 = k1;
        k1 = k;
        a = b;
        b = r;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 連分数と収束分数の整合() {
        // 415/93 = [4;2,6,7]（既知例）。
        assert_eq!(連分数展開(415, 93), vec![4, 2, 6, 7]);
        let c = 収束分数(&連分数展開(415, 93));
        assert_eq!(*c.last().unwrap(), (415, 93)); // 最後の収束分数は元の既約分数
        // 各収束分数は既約。
        fn gcd(a: isize, b: isize) -> isize {
            if b == 0 { a } else { gcd(b, a % b) }
        }
        for &(h, k) in &c {
            assert_eq!(gcd(h.abs(), k.abs()), 1);
        }
    }

    // 総当たり: 1<=b<=N の全分母で最近の a/b を求める。
    fn 素朴最良(分子: isize, 分母: isize, n: isize) -> (isize, isize) {
        let mut best = (分子 / 分母, 1isize);
        for b in 1..=n {
            let a0 = 分子 * b / 分母; // floor
            for a in [a0, a0 + 1] {
                let cand = (a, b);
                best = match 距離cmp(分子, 分母, cand, best) {
                    Ordering::Less => cand,
                    Ordering::Greater => best,
                    Ordering::Equal => {
                        if cand.1 < best.1 {
                            cand
                        } else {
                            best
                        }
                    }
                };
            }
        }
        best
    }

    #[test]
    fn 最良近似が総当たりと同距離() {
        let mut seed = 0xc0a7_f7a0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..2000 {
            let 分子 = (乱数() % 200) as isize;
            let 分母 = (乱数() % 200) as isize + 1;
            let n = (乱数() % 30) as isize + 1;
            let 得 = 最良有理近似(分子, 分母, n);
            assert!(得.1 >= 1 && 得.1 <= n, "分母範囲外 {得:?}");
            let 期待 = 素朴最良(分子, 分母, n);
            // 距離が等しいことを確認（同距離なら別の表現でも可）。
            assert_eq!(
                距離cmp(分子, 分母, 得, 期待),
                Ordering::Equal,
                "分子/分母={分子}/{分母} n={n} 得={得:?} 期待={期待:?}"
            );
        }
    }
}
