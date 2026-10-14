use cargo_snippet::snippet;
#[snippet("GaleShapley")]
#[snippet("安定_マッチング")]
#[snippet("安定_マッチング_問題")]
#[snippet("安定_結婚_問題")]
use std::collections::VecDeque;

#[snippet("安定_マッチング")]
#[snippet("安定_結婚_問題")]
#[snippet("GaleShapley")]
#[snippet("安定_マッチング_問題")]
/// Gale–Shapley 法で安定マッチング（安定結婚問題の男性最適解）を求める。`O(n^2)`。
///
/// `男性選好[m]` は男性 `m` の女性の選好順（先頭が最も好む）、`女性選好[w]` は女性 `w` の
/// 男性の選好順。どちらも `0..n` の順列で、人数 `n` は両者で等しいこと。
/// 返り値 `妻[m]` は男性 `m` の相手の女性。得られる結果はブロッキングペアの無い安定マッチングで、
/// かつ全男性にとって最良（男性最適）。
fn 安定マッチング(
    男性選好: &[Vec<usize>], 女性選好: &[Vec<usize>]
) -> Vec<usize> {
    let n = 男性選好.len();
    assert_eq!(n, 女性選好.len());
    if n == 0 {
        return Vec::new();
    }
    // 女性順位[w][m] = 女性 w から見た男性 m の順位（小さいほど好む）。
    let mut 女性順位 = vec![vec![0usize; n]; n];
    for w in 0..n {
        for (r, &m) in 女性選好[w].iter().enumerate() {
            女性順位[w][m] = r;
        }
    }
    let mut 次提案 = vec![0usize; n]; // 男性 m が次に提案する選好リストの位置
    let mut 夫: Vec<Option<usize>> = vec![None; n]; // 夫[w] = 現在の婚約者
    let mut 自由: VecDeque<usize> = (0..n).collect();
    while let Some(m) = 自由.pop_front() {
        let w = 男性選好[m][次提案[m]];
        次提案[m] += 1;
        match 夫[w] {
            None => 夫[w] = Some(m),
            Some(m2) => {
                if 女性順位[w][m] < 女性順位[w][m2] {
                    夫[w] = Some(m); // w は m の方を好む
                    自由.push_back(m2); // 振られた m2 は再び自由
                } else {
                    自由.push_back(m); // m は振られ、次の相手へ
                }
            }
        }
    }
    let mut 妻 = vec![0usize; n];
    for w in 0..n {
        妻[夫[w].unwrap()] = w;
    }
    妻
}

#[cfg(test)]
mod tests {
    use super::*;

    // ブロッキングペアが無い（安定）かどうかを定義通り確認する。
    fn 安定か(男性選好: &[Vec<usize>], 女性選好: &[Vec<usize>], 妻: &[usize]) -> bool {
        let n = 妻.len();
        let mut 夫 = vec![0usize; n];
        for m in 0..n {
            夫[妻[m]] = m;
        }
        let 順位 = |選好: &[usize], x: usize| 選好.iter().position(|&y| y == x).unwrap();
        for m in 0..n {
            for &w in &男性選好[m] {
                if w == 妻[m] {
                    break; // これ以降は現在の相手より好まない
                }
                // m は w を現在の妻より好む。w も m を現在の夫より好めばブロッキング。
                if 順位(&女性選好[w], m) < 順位(&女性選好[w], 夫[w]) {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn 既知例() {
        // 古典例（3人）。
        let 男 = vec![vec![0, 1, 2], vec![1, 0, 2], vec![0, 1, 2]];
        let 女 = vec![vec![1, 0, 2], vec![0, 1, 2], vec![0, 1, 2]];
        let 妻 = 安定マッチング(&男, &女);
        assert!(安定か(&男, &女, &妻));
        // 全単射であること。
        let mut s = 妻.clone();
        s.sort_unstable();
        assert_eq!(s, vec![0, 1, 2]);
    }

    #[test]
    fn 乱数で安定かつ全単射() {
        let mut seed = 0x57ab_1e00u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        // ランダム順列を作る。
        let 順列 = |n: usize, 乱数: &mut dyn FnMut() -> u64| {
            let mut p: Vec<usize> = (0..n).collect();
            for i in (1..n).rev() {
                let j = (乱数() as usize) % (i + 1);
                p.swap(i, j);
            }
            p
        };
        for _ in 0..3000 {
            let n = (乱数() as usize % 8) + 1;
            let 男: Vec<Vec<usize>> = (0..n).map(|_| 順列(n, &mut 乱数)).collect();
            let 女: Vec<Vec<usize>> = (0..n).map(|_| 順列(n, &mut 乱数)).collect();
            let 妻 = 安定マッチング(&男, &女);
            // 全単射。
            let mut s = 妻.clone();
            s.sort_unstable();
            assert_eq!(
                s,
                (0..n).collect::<Vec<_>>(),
                "全単射でない 男={男:?} 女={女:?}"
            );
            // 安定。
            assert!(
                安定か(&男, &女, &妻),
                "安定でない 男={男:?} 女={女:?} 妻={妻:?}"
            );
        }
    }
}
