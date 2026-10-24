use cargo_snippet::snippet;

#[snippet("行列_累乗")]
#[snippet("行列_積")]
#[snippet("matpow")]
/// 2つの行列の積を `mod 法` で計算する。`a` は `n×k`、`b` は `k×m` を仮定し `n×m` を返す。
///
/// 各成分は `0 <= 値 < 法` に正規化された前提（`行列累乗` 内部から呼ばれる）。
/// `法 < 約3×10^9` なら `isize` の乗算であふれない。
fn 行列積(a: &[Vec<isize>], b: &[Vec<isize>], 法: isize) -> Vec<Vec<isize>> {
    let m = if b.is_empty() { 0 } else { b[0].len() };
    let mut c = vec![vec![0isize; m]; a.len()];
    for (c行, a行) in c.iter_mut().zip(a.iter()) {
        for (&a成分, b行) in a行.iter().zip(b.iter()) {
            if a成分 == 0 {
                continue;
            }
            for (c成分, &b成分) in c行.iter_mut().zip(b行.iter()) {
                *c成分 = (*c成分 + a成分 * b成分) % 法;
            }
        }
    }
    c
}

#[snippet("行列_累乗")]
#[snippet("行列_積")]
#[snippet("matpow")]
/// 正方行列 `行列` の `指数` 乗を `mod 法` で計算する（繰り返し二乗法）。O(n^3 log 指数)。
///
/// 線形漸化式（フィボナッチ等）を行列で表して N 項目を高速に求めるのに使う。
/// `指数 == 0` のときは単位行列を返す。
fn 行列累乗(行列: &[Vec<isize>], mut 指数: usize, 法: isize) -> Vec<Vec<isize>> {
    let n = 行列.len();
    let mut 結果 = vec![vec![0isize; n]; n];
    for (i, 行) in 結果.iter_mut().enumerate() {
        行[i] = 1 % 法;
    }
    let mut 底: Vec<Vec<isize>> = 行列
        .iter()
        .map(|行| 行.iter().map(|&x| x.rem_euclid(法)).collect())
        .collect();
    while 指数 > 0 {
        if 指数 & 1 == 1 {
            結果 = 行列積(&結果, &底, 法);
        }
        底 = 行列積(&底, &底, 法);
        指数 >>= 1;
    }
    結果
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn フィボナッチを行列累乗() {
        let 法 = 1_000_000_007;
        // [[1,1],[1,0]]^n の左上が F(n+1)
        let a = vec![vec![1, 1], vec![1, 0]];
        let m = 行列累乗(&a, 10, 法);
        assert_eq!(m[0][0], 89); // F(11) = 89
        assert_eq!(m[0][1], 55); // F(10) = 55
    }

    #[test]
    fn 単位行列() {
        let a = vec![vec![2, 3], vec![4, 5]];
        let m = 行列累乗(&a, 0, 1_000_000_007);
        assert_eq!(m, vec![vec![1, 0], vec![0, 1]]);
    }
}
