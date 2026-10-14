use cargo_snippet::snippet;

#[snippet("素数_計数")]
#[snippet("LucyHedgehog")]
#[snippet("素数の_個数")]
#[snippet("素数_和")]
/// Lucy_Hedgehog 法の内部 DP。`n` 以下の各「`floor(n/i)` で表せる値 `m`」について
/// `Σ_{2<=k<=m, k は素数} 重み(k)` を計算し、最終的に `m = n` の値を返す。`O(n^(3/4))`。
///
/// `接頭辞(x)` は初期値 `Σ_{k=2}^{x} 重み(k)`（素数に限らない総和）を返すこと。
fn ルーシー篩<W: Fn(u64) -> i128, P: Fn(u64) -> i128>(
    n: u64, 接頭辞: P, 重み: W
) -> i128 {
    if n < 2 {
        return 0;
    }
    let mut v = (n as f64).sqrt() as u64;
    while (v + 1) * (v + 1) <= n {
        v += 1;
    }
    while v * v > n {
        v -= 1;
    }
    let mut small = vec![0i128; (v + 1) as usize]; // small[i] = S(i)
    let mut large = vec![0i128; (v + 1) as usize]; // large[i] = S(floor(n/i))
    for i in 1..=v {
        small[i as usize] = 接頭辞(i);
        large[i as usize] = 接頭辞(n / i);
    }
    for p in 2..=v {
        // S(p) が S(p-1) と同じなら p は合成数（既に篩い落とされた）。
        if small[p as usize] == small[(p - 1) as usize] {
            continue;
        }
        let sp = small[(p - 1) as usize]; // p 未満の素数の重み和
        let wp = 重み(p);
        let p2 = p * p;
        // large 側: floor(n/i) >= p^2 すなわち i <= n/p^2 を更新。
        let lim = (n / p2).min(v);
        for i in 1..=lim {
            let d = i * p; // floor(n/(i*p)) が S の引数（= floor(floor(n/i)/p)）
            let 値 = if d <= v {
                large[d as usize]
            } else {
                small[(n / d) as usize]
            };
            large[i as usize] -= wp * (値 - sp);
        }
        // small 側: i >= p^2 を降順に更新。
        let mut i = v;
        while i >= p2 {
            small[i as usize] -= wp * (small[(i / p) as usize] - sp);
            i -= 1;
        }
    }
    large[1] // S(floor(n/1)) = S(n)
}

#[snippet("素数_計数")]
#[snippet("LucyHedgehog")]
#[snippet("素数の_個数")]
/// `n` 以下の素数の個数 `π(n)` を Lucy_Hedgehog 法で `O(n^(3/4))` で求める。
///
/// 篩（[`prime_number`](crate::prime_number)・`primal` クレート）は `O(n)` 領域が必要で
/// `n` が `10^10` 規模だと回らないが、本法は `√n` 規模の配列だけで個数を数えられる。
fn 素数計数(n: impl TryInto<u64>) -> usize {
    let n: u64 = n
        .try_into()
        .ok()
        .expect("素数計数: 引数が u64 に収まりません");
    ルーシー篩(n, |x| x as i128 - 1, |_| 1) as usize
}

#[snippet("素数_計数")]
#[snippet("LucyHedgehog")]
#[snippet("素数_和")]
/// `n` 以下の素数の総和を Lucy_Hedgehog 法で `O(n^(3/4))` で求める（`i128` で返す）。
fn 素数和(n: impl TryInto<u64>) -> i128 {
    let n: u64 = n
        .try_into()
        .ok()
        .expect("素数和: 引数が u64 に収まりません");
    ルーシー篩(
        n,
        |x| {
            let x = x as i128;
            x * (x + 1) / 2 - 1
        },
        |p| p as i128,
    )
}

#[cfg(test)]
// n は索引と π(n)/素数和(n) の引数の両方に使うため範囲ループが自然。
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;

    // エラトステネスの篩で 0..=n の素数判定表を作る。
    fn 篩(n: usize) -> Vec<bool> {
        let mut 素数 = vec![true; n + 1];
        素数[0] = false;
        if n >= 1 {
            素数[1] = false;
        }
        let mut i = 2;
        while i * i <= n {
            if 素数[i] {
                let mut j = i * i;
                while j <= n {
                    素数[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
        素数
    }

    #[test]
    fn 既知値() {
        assert_eq!(素数計数(0), 0);
        assert_eq!(素数計数(1), 0);
        assert_eq!(素数計数(2), 1);
        assert_eq!(素数計数(10), 4);
        assert_eq!(素数計数(100), 25);
        assert_eq!(素数計数(1000), 168);
        assert_eq!(素数計数(1_000_000), 78498);
        assert_eq!(素数和(10), 17); // 2+3+5+7
        assert_eq!(素数和(2), 2);
        assert_eq!(素数和(1), 0);
    }

    #[test]
    fn 素数計数が篩と一致() {
        const M: usize = 3000;
        let 素数 = 篩(M);
        let mut 累計 = 0usize;
        for n in 0..=M {
            if 素数[n] {
                累計 += 1;
            }
            assert_eq!(素数計数(n as u64), 累計, "π({n})");
        }
    }

    #[test]
    fn 素数和が篩と一致() {
        const M: usize = 3000;
        let 素数 = 篩(M);
        let mut 累計 = 0i128;
        for n in 0..=M {
            if 素数[n] {
                累計 += n as i128;
            }
            assert_eq!(素数和(n as u64), 累計, "素数和({n})");
        }
    }
}
