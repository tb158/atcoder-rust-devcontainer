use cargo_snippet::snippet;

#[snippet("分割_統治_DP_最適化")]
#[snippet("単調_最小値")]
#[snippet("MonotoneMinima")]
#[snippet("DivideAndConquerDP")]
/// 単調最小値（monotone minima）。各行 `i` の最小値をとる列 `argmin(i)` が `i` について単調非減少
/// であるような暗黙の行列に対し、全行の `(最小値, argmin)` を `O((行数 + 列数) log 行数)` で求める。
///
/// `コスト(i, j)` が行 `i`・列 `j` の値を返す。Monge（凸四角不等式）な行列で成り立つ。
///
/// 同点のときはより小さい列を採用する。`列数 == 0` のときは全行 `(isize::MAX, 0)`。
fn 単調最小値<F: Fn(usize, usize) -> isize>(
    行数: usize,
    列数: usize,
    コスト: F,
) -> Vec<(isize, usize)> {
    let mut 結果 = vec![(isize::MAX, 0usize); 行数];
    if 行数 == 0 || 列数 == 0 {
        return 結果;
    }
    // (行下限, 行上限(排他), 列下限, 列上限(包含)) を反復的に分割統治する。
    let mut スタック = vec![(0usize, 行数, 0usize, 列数 - 1)];
    while let Some((行下, 行上, 列下, 列上)) = スタック.pop() {
        if 行下 >= 行上 {
            continue;
        }
        let 中 = (行下 + 行上) / 2;
        let mut 最良値 = isize::MAX;
        let mut 最良列 = 列下;
        for j in 列下..=列上 {
            let v = コスト(中, j);
            if v < 最良値 {
                最良値 = v;
                最良列 = j;
            }
        }
        結果[中] = (最良値, 最良列);
        // 上半分の最適列は [列下, 最良列]、下半分は [最良列, 列上] に限定される。
        スタック.push((行下, 中, 列下, 最良列));
        スタック.push((中 + 1, 行上, 最良列, 列上));
    }
    結果
}

#[snippet("分割_統治_DP_最適化")]
#[snippet("単調_最小値")]
#[snippet("MonotoneMinima")]
#[snippet("DivideAndConquerDP")]
/// 1 層の DP 遷移 `新[j] = min_{0 <= k <= j} ( 旧[k] + コスト(k, j) )` を、最適 `k` が `j` について
/// 単調と仮定して `O(N log N)` で計算する（[`単調最小値`] を三角制約付きで適用）。
///
/// `コスト(k, j)` は `k <= j` のときに呼ばれ、Monge 性（`コスト(a,c)+コスト(b,d) <= コスト(a,d)+コスト(b,c)`、
/// `a<=b<=c<=d`）を満たすときに正しい。`旧[k] == isize::MAX` は到達不能として扱う（飽和加算）。
/// 返り値の長さは `旧.len()`。
fn 分割統治dp層<F: Fn(usize, usize) -> isize>(旧: &[isize], コスト: F) -> Vec<isize> {
    let n = 旧.len();
    let 行列 = |j: usize, k: usize| -> isize {
        if k <= j && 旧[k] != isize::MAX {
            旧[k].saturating_add(コスト(k, j))
        } else {
            isize::MAX
        }
    };
    単調最小値(n, n, 行列).into_iter().map(|(v, _)| v).collect()
}

#[cfg(test)]
// Indices double as matrix coordinates / prefix-sum offsets (`s[i+1]`, `コスト(i, j)`), so range
// loops are the natural form here.
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;

    #[test]
    fn 単調最小値が総当たりと一致() {
        let mut seed = 0x0bad_beefu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..200 {
            let 行数 = (乱数() as usize % 30) + 1;
            let 列数 = (乱数() as usize % 30) + 1;
            // 凸（Monge）な行列を作る: cost(i,j) = (x[i] - y[j])^2、x・y は昇順。
            let mut x: Vec<isize> = (0..行数).map(|_| (乱数() % 50) as isize).collect();
            let mut y: Vec<isize> = (0..列数).map(|_| (乱数() % 50) as isize).collect();
            x.sort_unstable();
            y.sort_unstable();
            let コスト = |i: usize, j: usize| {
                let d = x[i] - y[j];
                d * d
            };
            let 結果 = 単調最小値(行数, 列数, コスト);
            for i in 0..行数 {
                let mut 期待 = isize::MAX;
                for j in 0..列数 {
                    期待 = 期待.min(コスト(i, j));
                }
                assert_eq!(結果[i].0, 期待, "i={i}");
                // argmin の値も最小であること
                assert_eq!(コスト(i, 結果[i].1), 期待, "argmin i={i}");
            }
        }
    }

    #[test]
    fn 分割統治dpが総当たりと一致() {
        let mut seed = 0xcafe_d00du64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..200 {
            let n = (乱数() as usize % 25) + 1;
            // prefix sum S[0..=n]、コスト(k,j) = (S[j+1]-S[k])^2（Monge）。
            let a: Vec<isize> = (0..n).map(|_| (乱数() % 20) as isize).collect();
            let mut s = vec![0isize; n + 1];
            for i in 0..n {
                s[i + 1] = s[i] + a[i];
            }
            // 旧[k] を適当な単調量にする（到達不能 MAX も混ぜる）。
            let 旧: Vec<isize> = (0..n)
                .map(|_| {
                    if 乱数() % 7 == 0 {
                        isize::MAX
                    } else {
                        (乱数() % 30) as isize
                    }
                })
                .collect();
            let コスト = |k: usize, j: usize| {
                let d = s[j + 1] - s[k];
                d * d
            };
            let 新 = 分割統治dp層(&旧, コスト);
            // 総当たり O(n^2)
            for j in 0..n {
                let mut 期待 = isize::MAX;
                for k in 0..=j {
                    if 旧[k] == isize::MAX {
                        continue;
                    }
                    期待 = 期待.min(旧[k] + コスト(k, j));
                }
                assert_eq!(新[j], 期待, "j={j} 旧={旧:?}");
            }
        }
    }
}
