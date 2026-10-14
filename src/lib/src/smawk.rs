use cargo_snippet::snippet;

#[snippet("SMAWK")]
#[snippet("行_最小値")]
#[snippet("完全_単調")]
#[snippet("トータル_モノトーン")]
#[snippet("Monge")]
/// REDUCE 操作: どの行の最小にもなり得ない列を取り除き、残す列数を行数以下にする内部関数。
fn 列削減<T: PartialOrd, F: Fn(usize, usize) -> T>(
    行: &[usize],
    列: &[usize],
    コスト: &F,
) -> Vec<usize> {
    let mut 残す: Vec<usize> = Vec::new();
    for &c in 列 {
        while let Some(&上) = 残す.last() {
            let k = 残す.len() - 1;
            if k < 行.len() && コスト(行[k], 上) > コスト(行[k], c) {
                残す.pop();
            } else {
                break;
            }
        }
        if 残す.len() < 行.len() {
            残す.push(c);
        }
    }
    残す
}

#[snippet("SMAWK")]
#[snippet("行_最小値")]
#[snippet("完全_単調")]
#[snippet("トータル_モノトーン")]
#[snippet("Monge")]
/// SMAWK 本体（再帰）。偶数位置の行は奇数位置の行の答えで挟み込んで線形に解く。
fn 解く<T: PartialOrd, F: Fn(usize, usize) -> T>(
    行: &[usize],
    列: &[usize],
    コスト: &F,
    答え: &mut [usize],
) {
    if 行.is_empty() {
        return;
    }
    let 列 = 列削減(行, 列, コスト);
    let 奇: Vec<usize> = 行.iter().skip(1).step_by(2).copied().collect();
    解く(&奇, &列, コスト, 答え);
    let mut j = 0;
    let mut pos = 0;
    while pos < 行.len() {
        let r = 行[pos];
        // この行の最小列は、左隣の奇数行の答え以上・右隣の奇数行の答え以下にある。
        let 終列 = if pos + 1 < 行.len() {
            答え[行[pos + 1]]
        } else {
            *列.last().unwrap()
        };
        let mut 最良 = 列[j];
        while 列[j] != 終列 {
            j += 1;
            if コスト(r, 列[j]) < コスト(r, 最良) {
                最良 = 列[j];
            }
        }
        答え[r] = 最良;
        pos += 2;
    }
}

#[snippet("SMAWK")]
#[snippet("行_最小値")]
#[snippet("完全_単調")]
#[snippet("トータル_モノトーン")]
#[snippet("Monge")]
/// SMAWK 法。完全単調（totally monotone, Monge を含む）な `行数 × 列数` 行列について、
/// 各行の最小値をとる列番号を `O(行数 + 列数)` で求める。`コスト(i, j)` で要素を遅延参照する。
///
/// 行列が完全単調（任意の `i<i'`, `j<j'` で `A[i][j]>A[i][j']` ⇒ `A[i'][j]>A[i'][j']`）であることが前提。
/// 分割統治の単調最小値（[`divide_and_conquer_dp`](crate::divide_and_conquer_dp)、`O(n log n)`）より
/// 高速。最近点・DP 高速化などで使う。
///
/// 返り値 `argmin[i]` は行 `i` の最小値をとる列（同点は最左）。`列数 == 0` のときは空でない行に `0` が入る。
/// コストの型は比較できれば何でもよい（i64 / f64 など）。
fn 行ごとの最小列<T: PartialOrd, F: Fn(usize, usize) -> T>(
    行数: usize,
    列数: usize,
    コスト: F,
) -> Vec<usize> {
    let mut 答え = vec![0usize; 行数];
    if 行数 == 0 || 列数 == 0 {
        return 答え;
    }
    let 行: Vec<usize> = (0..行数).collect();
    let 列: Vec<usize> = (0..列数).collect();
    解く(&行, &列, &コスト, &mut 答え);
    答え
}

#[cfg(test)]
// f(i, j) は行番号 i を本質的に使うため、行ループは範囲ループが自然。
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;

    #[test]
    fn 小さな例() {
        // 各行で最小の列を持つ完全単調な行列（距離の2乗、a・b とも増加）。
        let a = [0i64, 1, 4, 9];
        let b = [0i64, 2, 3, 8];
        let f = |i: usize, j: usize| (a[i] - b[j]) * (a[i] - b[j]);
        let argmin = 行ごとの最小列(4, 4, f);
        for i in 0..4 {
            let 真の最小 = (0..4).map(|j| f(i, j)).min().unwrap();
            assert_eq!(f(i, argmin[i]), 真の最小, "行 {i}");
        }
    }

    #[test]
    fn 乱数モンジュ行列で最小値一致() {
        let mut seed = 0x5a2e_8e10u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..300 {
            let n = (乱数() as usize % 12) + 1;
            let m = (乱数() as usize % 12) + 1;
            // a, b を昇順に取れば A[i][j] = (a_i - b_j)^2 は Monge（完全単調）。
            let mut a: Vec<i64> = (0..n).map(|_| (乱数() % 100) as i64).collect();
            let mut b: Vec<i64> = (0..m).map(|_| (乱数() % 100) as i64).collect();
            a.sort_unstable();
            b.sort_unstable();
            let f = |i: usize, j: usize| {
                let d = a[i] - b[j];
                d * d
            };
            let argmin = 行ごとの最小列(n, m, f);
            assert_eq!(argmin.len(), n);
            for i in 0..n {
                let 真の最小 = (0..m).map(|j| f(i, j)).min().unwrap();
                assert_eq!(f(i, argmin[i]), 真の最小, "n={n} m={m} 行={i}");
                // 完全単調なら argmin は非減少。
                if i > 0 {
                    assert!(argmin[i] >= argmin[i - 1], "argmin 非単調 行={i}");
                }
            }
        }
    }
}
