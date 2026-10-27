use cargo_snippet::snippet;

#[snippet("スプラーグ_グランディ")]
#[snippet("グランディ_数")]
#[snippet("mex")]
#[snippet("Grundy")]
/// 非負整数の集合に含まれない最小の非負整数 **mex**（minimum excludant）を返す。`O(N)`。
/// 重複があってもよい（ソート不要）。
fn mex(集合: &[usize]) -> usize {
    let mut 出現 = vec![false; 集合.len() + 1];
    for &x in 集合 {
        if x < 出現.len() {
            出現[x] = true;
        }
    }
    // 出現[要素数] が true になるには相異なる要素が要素数+1 個必要なので、必ず未出現位置がある。
    出現.iter().position(|&出| !出).unwrap()
}

#[snippet("グランディ_数")]
#[snippet("Grundy")]
#[snippet("スプラーグ_グランディ")]
/// 状態 `0..=上限` それぞれの**グランディ数**（Sprague–Grundy 値）を表で返す。
/// `遷移(i)` は状態 `i` から 1 手で到達できる状態の集合で、要素はすべて `i` 未満であること
/// （有限・非循環な不偏ゲーム）。`grundy[i] = mex{ grundy[j] : j ∈ 遷移(i) }`。
///
/// `grundy[i] == 0` が後手必勝（手番側の負け）状態。独立した複数のゲームの合成局面は
/// 各ゲームのグランディ数の XOR（[`ニム和`]）で判定でき、XOR が 0 なら手番側の負け
/// （Sprague–Grundy の定理）。計算量は `O(Σ|遷移(i)|)`。
///
/// 状態を整数に符号化できる山取り・減算ゲームなどに使う。
fn グランディ数列<F: Fn(usize) -> Vec<usize>>(上限: usize, 遷移: F) -> Vec<usize> {
    let mut g = vec![0usize; 上限 + 1];
    for i in 0..=上限 {
        let next: Vec<usize> = 遷移(i).iter().map(|&j| g[j]).collect();
        g[i] = mex(&next);
    }
    g
}

#[snippet("ニム_和")]
#[snippet("Nim")]
/// 山（グランディ数）の XOR を取った**ニム和**を返す。`0` なら手番側の負け局面。
fn ニム和(山: &[usize]) -> usize {
    山.iter().fold(0, |a, &x| a ^ x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mexの既知例() {
        assert_eq!(mex(&[]), 0);
        assert_eq!(mex(&[0, 1, 2]), 3);
        assert_eq!(mex(&[1, 2]), 0);
        assert_eq!(mex(&[0, 2, 3]), 1);
        assert_eq!(mex(&[0, 0, 1, 1, 3]), 2); // 重複があっても可
    }

    #[test]
    fn 減算ゲームの閉じた形と一致() {
        // 1〜3 個取れる減算ゲームのグランディ数は i % 4。
        let g = グランディ数列(100, |i| {
            (1..=3).filter(|&s| s <= i).map(|s| i - s).collect()
        });
        for (i, &gi) in g.iter().enumerate() {
            assert_eq!(gi, i % 4, "i={i}");
        }
    }

    // 1 山ゲームで「到達先に負け状態があれば勝ち」を直接判定する勝敗 DP。
    fn 勝敗(上限: usize, 手: &[usize]) -> Vec<bool> {
        let mut 勝ち = vec![false; 上限 + 1];
        for i in 0..=上限 {
            勝ち[i] = 手.iter().any(|&s| s <= i && !勝ち[i - s]);
        }
        勝ち
    }

    #[test]
    fn グランディ0が負け状態と一致() {
        let mut seed = 0x33b1_92c0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..200 {
            // ランダムな着手集合（1..=6 のうちいくつか）。
            let 手: Vec<usize> = (1..=6).filter(|_| 乱数() % 2 == 0).collect();
            if 手.is_empty() {
                continue;
            }
            let 上限 = 60;
            let g = グランディ数列(上限, |i| {
                手.iter().filter(|&&s| s <= i).map(|&s| i - s).collect()
            });
            let 勝ち = 勝敗(上限, &手);
            for i in 0..=上限 {
                assert_eq!(g[i] == 0, !勝ち[i], "手={手:?} i={i} g={}", g[i]);
            }
        }
    }

    // 2 山の合成局面 (a,b) の勝敗を直接ゲーム木で求める素朴版。
    fn 二山勝敗(上限: usize, 手: &[usize]) -> Vec<Vec<bool>> {
        let mut 勝ち = vec![vec![false; 上限 + 1]; 上限 + 1];
        for a in 0..=上限 {
            for b in 0..=上限 {
                let 勝てる = 手
                    .iter()
                    .any(|&s| (s <= a && !勝ち[a - s][b]) || (s <= b && !勝ち[a][b - s]));
                勝ち[a][b] = 勝てる;
            }
        }
        勝ち
    }

    #[test]
    fn ニム和が二山ゲームの勝敗と一致() {
        let mut seed = 0x7c40_1a90u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..60 {
            let 手: Vec<usize> = (1..=4).filter(|_| 乱数() % 2 == 0).collect();
            if 手.is_empty() {
                continue;
            }
            let 上限 = 14;
            let g = グランディ数列(上限, |i| {
                手.iter().filter(|&&s| s <= i).map(|&s| i - s).collect()
            });
            let 勝ち = 二山勝敗(上限, &手);
            for a in 0..=上限 {
                for b in 0..=上限 {
                    // ニム和（XOR）が 0 ⇔ 手番側の負け。
                    assert_eq!(
                        ニム和(&[g[a], g[b]]) == 0,
                        !勝ち[a][b],
                        "手={手:?} a={a} b={b}"
                    );
                }
            }
        }
    }
}
