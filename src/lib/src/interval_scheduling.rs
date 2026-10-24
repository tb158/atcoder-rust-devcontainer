use cargo_snippet::snippet;

#[snippet("区間_スケジューリング")]
#[snippet("区間_スケジューリング_問題")]
#[snippet("最大_区間_数")]
#[snippet("重ならない_区間")]
/// 区間スケジューリング問題。半開区間 `[開始, 終了)` の集合から、
/// 互いに重ならないように選べる区間の最大個数を返す貪欲法。O(N log N)。
///
/// `区間` は `(開始, 終了)` の列。終端 `終了` が早い順に貪欲に取れば最適になる。
/// 端点が接するだけ（前の `終了 == 次の 開始`）は重ならないとみなす。
///
/// 「最大個数」ではなく実際に選んだ区間が欲しいときは [`区間スケジューリング_選択`] を使う。
fn 区間スケジューリング(区間: &[(isize, isize)]) -> usize {
    let mut 並び: Vec<(isize, isize)> = 区間.to_vec();
    並び.sort_unstable_by_key(|&(_, 終)| 終);
    let mut 個数 = 0;
    let mut 直前の終了 = isize::MIN;
    for &(開始, 終了) in &並び {
        if 開始 >= 直前の終了 {
            個数 += 1;
            直前の終了 = 終了;
        }
    }
    個数
}

#[snippet("区間_スケジューリング")]
#[snippet("区間_スケジューリング_選択")]
#[snippet("重ならない_区間")]
/// 区間スケジューリングで実際に採用した区間を、選んだ順（終端の昇順）で返す。
///
/// 戻り値は `(開始, 終了)` の列。個数だけ欲しいなら [`区間スケジューリング`]。
fn 区間スケジューリング_選択(区間: &[(isize, isize)]) -> Vec<(isize, isize)> {
    let mut 並び: Vec<(isize, isize)> = 区間.to_vec();
    並び.sort_unstable_by_key(|&(_, 終)| 終);
    let mut 採用 = Vec::new();
    let mut 直前の終了 = isize::MIN;
    for &(開始, 終了) in &並び {
        if 開始 >= 直前の終了 {
            採用.push((開始, 終了));
            直前の終了 = 終了;
        }
    }
    採用
}

#[snippet("重み付き_区間_スケジューリング")]
#[snippet("重み付き_区間")]
#[snippet("WeightedIntervalScheduling")]
/// 重み付き区間スケジューリング。区間 `(開始, 終了, 重み)` から、互いに重ならないように選んだときの
/// 重み和の最大値と、選んだ区間の元の添字を返す。端点の接触（`終了 == 次の開始`）は許す。O(N log N)。
///
/// 重みなし（個数最大化）なら [`区間スケジューリング`]。重みが付くと貪欲は最適でないので DP になる。
fn 重み付き区間スケジューリング(
    区間: &[(isize, isize, isize)],
) -> (isize, Vec<usize>) {
    let n = 区間.len();
    if n == 0 {
        return (0, Vec::new());
    }
    // (終了, 開始, 重み, 元添字) を終了昇順に並べる。
    let mut 並び: Vec<(isize, isize, isize, usize)> = 区間
        .iter()
        .enumerate()
        .map(|(i, &(s, e, w))| (e, s, w, i))
        .collect();
    並び.sort_by_key(|&(e, _, _, _)| e);
    let 終了: Vec<isize> = 並び.iter().map(|&(e, _, _, _)| e).collect();
    let mut dp = vec![0isize; n + 1];
    let mut 取る = vec![false; n + 1]; // dp[i] で i 番目（並び）を取ったか
    let mut 戻り = vec![0usize; n + 1]; // 取ったときの p(i)
    for i in 1..=n {
        let (_, s, w, _) = 並び[i - 1];
        // 終了 <= s となる最後の位置 p（0..i-1 の範囲で二分探索）。
        let p = 終了[..i - 1].partition_point(|&e| e <= s);
        let 取った値 = dp[p] + w;
        if 取った値 >= dp[i - 1] {
            dp[i] = 取った値;
            取る[i] = true;
            戻り[i] = p;
        } else {
            dp[i] = dp[i - 1];
        }
    }
    // 復元。
    let mut 採用 = Vec::new();
    let mut i = n;
    while i > 0 {
        if 取る[i] {
            採用.push(並び[i - 1].3);
            i = 戻り[i];
        } else {
            i -= 1;
        }
    }
    採用.sort_unstable();
    (dp[n], 採用)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 基本的な選択() {
        // [1,3) [2,5) [4,7) [6,8) → (4,7) と (6,8) は重なるので最大2個
        let v = vec![(1, 3), (2, 5), (4, 7), (6, 8)];
        assert_eq!(区間スケジューリング(&v), 2);
        assert_eq!(区間スケジューリング_選択(&v), vec![(1, 3), (4, 7)]);
    }

    #[test]
    fn 端点が接する区間は両方取れる() {
        let v = vec![(0, 1), (1, 2), (2, 3)];
        assert_eq!(区間スケジューリング(&v), 3);
    }

    #[test]
    fn 全部重なる() {
        let v = vec![(0, 10), (1, 9), (2, 8)];
        assert_eq!(区間スケジューリング(&v), 1);
    }

    fn 重なる(a: (isize, isize, isize), b: (isize, isize, isize)) -> bool {
        a.0.max(b.0) < a.1.min(b.1) // 接触は重なりとしない
    }

    // 全 2^n 部分集合で互いに重ならないものの最大重み。
    fn 重み総当たり(区間: &[(isize, isize, isize)]) -> isize {
        let n = 区間.len();
        let mut 最良 = 0isize;
        for s in 0u32..(1 << n) {
            let 選: Vec<usize> = (0..n).filter(|&i| s & (1 << i) != 0).collect();
            let 整合 = 選
                .iter()
                .enumerate()
                .all(|(x, &i)| 選[x + 1..].iter().all(|&j| !重なる(区間[i], 区間[j])));
            if 整合 {
                let w: isize = 選.iter().map(|&i| 区間[i].2).sum();
                最良 = 最良.max(w);
            }
        }
        最良
    }

    fn 重み採用検証(区間: &[(isize, isize, isize)], 値: isize, 採用: &[usize]) {
        for x in 0..採用.len() {
            for y in x + 1..採用.len() {
                assert!(
                    !重なる(区間[採用[x]], 区間[採用[y]]),
                    "採用が重なる {:?}",
                    採用
                );
            }
        }
        let w: isize = 採用.iter().map(|&i| 区間[i].2).sum();
        assert_eq!(w, 値, "採用重み和が値と不一致");
    }

    #[test]
    fn 重み付きの既知例() {
        // [0,3]w5, [2,5]w6, [3,8]w4 → [0,3]+[3,8]=9。
        let v = vec![(0, 3, 5), (2, 5, 6), (3, 8, 4)];
        let (値, 採用) = 重み付き区間スケジューリング(&v);
        assert_eq!(値, 9);
        重み採用検証(&v, 値, &採用);
        // 全部重なる → 最大重み 1 つ。
        assert_eq!(
            重み付き区間スケジューリング(&[(0, 10, 3), (1, 9, 8), (2, 8, 5)]).0,
            8
        );
        // 空。
        assert_eq!(重み付き区間スケジューリング(&[]), (0, vec![]));
    }

    #[test]
    fn 重み付きが総当たりと一致() {
        let mut seed = 0x9ad2_44c0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..3000 {
            let n = (乱数() as usize % 10) + 1; // 1..=10
            let 区間: Vec<(isize, isize, isize)> = (0..n)
                .map(|_| {
                    let a = (乱数() % 10) as isize;
                    let b = (乱数() % 10) as isize;
                    let (s, e) = if a <= b { (a, b + 1) } else { (b, a + 1) };
                    let w = (乱数() % 20) as isize + 1;
                    (s, e, w)
                })
                .collect();
            let (値, 採用) = 重み付き区間スケジューリング(&区間);
            assert_eq!(値, 重み総当たり(&区間), "区間={区間:?}");
            重み採用検証(&区間, 値, &採用);
        }
    }
}
