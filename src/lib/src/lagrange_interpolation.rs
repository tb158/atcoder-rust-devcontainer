use crate::mod_pow::逆元;
use cargo_snippet::snippet;

#[snippet("ラグランジュ_補間", include = "mod_逆元")]
#[snippet("LagrangeInterpolation")]
#[snippet("多項式_補間")]
#[snippet("連続点_補間")]
/// 相異なる点 `(xs[i], ys[i])` を通る次数 `< n` の多項式を素数 `法` 上で求め、`t` での値を返す。O(n^2)。
///
/// `法` は素数で、`xs` は互いに異なること。点が等間隔（0,1,2,...）なら [`連続点補間`] が O(n) で速い。
fn ラグランジュ補間(
    xs: &[impl TryInto<u64> + Copy],
    ys: &[impl TryInto<u64> + Copy],
    t: impl TryInto<u64>,
    法: impl TryInto<u64>,
) -> usize {
    let xs: Vec<u64> = xs
        .iter()
        .map(|&x| {
            x.try_into()
                .ok()
                .expect("ラグランジュ補間: 引数が u64 に収まりません")
        })
        .collect();
    let ys: Vec<u64> = ys
        .iter()
        .map(|&y| {
            y.try_into()
                .ok()
                .expect("ラグランジュ補間: 引数が u64 に収まりません")
        })
        .collect();
    let t: u64 = t
        .try_into()
        .ok()
        .expect("ラグランジュ補間: 引数が u64 に収まりません");
    let 法: u64 = 法
        .try_into()
        .ok()
        .expect("ラグランジュ補間: 引数が u64 に収まりません");
    let n = xs.len();
    let t = t % 法;
    let mut 結果 = 0u64;
    for i in 0..n {
        let mut 分子 = 1u64;
        let mut 分母 = 1u64;
        for j in 0..n {
            if i != j {
                分子 = 分子 * ((t + 法 - xs[j] % 法) % 法) % 法;
                分母 = 分母 * ((xs[i] % 法 + 法 - xs[j] % 法) % 法) % 法;
            }
        }
        結果 = (結果 + ys[i] % 法 * 分子 % 法 * 逆元(分母, 法)) % 法;
    }
    結果 as usize
}

#[snippet("ラグランジュ_補間")]
#[snippet("LagrangeInterpolation")]
#[snippet("多項式_補間")]
#[snippet("連続点_補間")]
/// `ys[i] = f(i)`（`i = 0..n`）から次数 `< n` の多項式 `f` を定め、`f(t)` を素数 `法` 上で O(n) で返す。
///
/// `Σ i^k` のような多項式和の閉じた式を点から外挿する用途で頻出。
/// `t` が `0..n` の範囲なら `ys[t]` をそのまま返す。
fn 連続点補間(
    ys: &[impl TryInto<u64> + Copy],
    t: impl TryInto<u64>,
    法: impl TryInto<u64>,
) -> usize {
    let ys: Vec<u64> = ys
        .iter()
        .map(|&y| {
            y.try_into()
                .ok()
                .expect("連続点補間: 引数が u64 に収まりません")
        })
        .collect();
    let t: u64 = t
        .try_into()
        .ok()
        .expect("連続点補間: 引数が u64 に収まりません");
    let 法: u64 = 法
        .try_into()
        .ok()
        .expect("連続点補間: 引数が u64 に収まりません");
    let n = ys.len();
    if n == 0 {
        return 0;
    }
    if t < n as u64 {
        return (ys[t as usize] % 法) as usize;
    }
    let d = n - 1;
    let tm = t % 法;
    let mut 階乗 = vec![1u64; n];
    for i in 1..n {
        階乗[i] = 階乗[i - 1] * i as u64 % 法;
    }
    // prev[i] = Π_{j<i} (t-j), 後[i] = Π_{j>=i} (t-j)
    let mut prev = vec![1u64; n + 1];
    for i in 0..n {
        prev[i + 1] = prev[i] * ((tm + 法 - i as u64 % 法) % 法) % 法;
    }
    let mut 後 = vec![1u64; n + 1];
    for i in (0..n).rev() {
        後[i] = 後[i + 1] * ((tm + 法 - i as u64 % 法) % 法) % 法;
    }
    let mut 結果 = 0u64;
    for i in 0..n {
        let 分子 = prev[i] * 後[i + 1] % 法;
        let 分母 = 階乗[i] * 階乗[d - i] % 法;
        let mut 項 = ys[i] % 法 * 分子 % 法 * 逆元(分母, 法) % 法;
        if (d - i) % 2 == 1 {
            項 = (法 - 項) % 法; // 符号 (-1)^{d-i}
        }
        結果 = (結果 + 項) % 法;
    }
    結果 as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    // 多項式（係数昇順）を法 p 上で x で評価する。
    fn 評価(係数: &[u64], x: u64, 法: u64) -> u64 {
        let mut r = 0u64;
        for &c in 係数.iter().rev() {
            r = (r * (x % 法) + c % 法) % 法;
        }
        r
    }

    #[test]
    fn 二次式() {
        // f(x) = x^2 + 3x + 2
        let 法 = 1_000_000_007u64;
        let xs = [0u64, 1, 2, 3];
        let ys: Vec<u64> = xs.iter().map(|&x| 評価(&[2, 3, 1], x, 法)).collect();
        assert_eq!(ラグランジュ補間(&xs, &ys, 10, 法), 132);
        assert_eq!(連続点補間(&ys, 10, 法), 132);
    }

    #[test]
    fn 範囲内はそのまま() {
        let 法 = 998_244_353u64;
        let ys = [5u64, 7, 11, 2];
        for (t, &y) in ys.iter().enumerate() {
            assert_eq!(連続点補間(&ys, t as u64, 法), y as usize);
        }
    }

    #[test]
    fn 乱数多項式で一致() {
        let mut seed = 0x9bad_f00du64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        let 法 = 998_244_353u64;
        for _ in 0..200 {
            let d = (乱数() as usize % 8) + 1; // 次数 1..=8
            let 係数: Vec<u64> = (0..=d).map(|_| 乱数() % 法).collect();
            // 連続点 f(0..=d)
            let ys: Vec<u64> = (0..=d as u64).map(|x| 評価(&係数, x, 法)).collect();
            let t = 乱数() % 法;
            let 期待 = 評価(&係数, t, 法);
            assert_eq!(連続点補間(&ys, t, 法), 期待 as usize, "d={d} t={t}");
            // 一般補間（任意の x 点）
            let xs: Vec<u64> = (0..=d as u64).map(|i| i * 2 + 3).collect();
            let ys2: Vec<u64> = xs.iter().map(|&x| 評価(&係数, x, 法)).collect();
            assert_eq!(
                ラグランジュ補間(&xs, &ys2, t, 法),
                期待 as usize,
                "d={d} t={t}"
            );
        }
    }
}
