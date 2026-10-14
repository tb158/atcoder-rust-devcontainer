use cargo_snippet::snippet;

#[snippet("ナップサック")]
#[snippet("01_ナップサック")]
#[snippet("ナップサック_DP")]
#[snippet("ナップサック_問題")]
#[snippet("ザック_DP")]
/// 0/1 ナップサック問題。容量 `容量` のリュックに、品物 `(重さ, 価値)` を
/// 各 0 個か 1 個選び、重さ合計が容量以下になるときの価値合計の最大値を返す。O(N·容量)。
///
/// 重さが大きく価値が小さいとき（容量が巨大で価値合計が小さいとき）は
/// [`価値基準ナップサック`] を使うと O(N·Σ価値) で解ける。
fn ナップサック(品物: &[(usize, i64)], 容量: usize) -> i64 {
    let mut dp = vec![0i64; 容量 + 1];
    for &(重さ, 価値) in 品物 {
        for w in (重さ..=容量).rev() {
            dp[w] = dp[w].max(dp[w - 重さ] + 価値);
        }
    }
    dp[容量]
}

#[snippet("個数_制限_なし_ナップサック")]
#[snippet("ナップサック")]
#[snippet("無限_ナップサック")]
/// 個数制限なしナップサック。各品物を何個でも選べる版。O(N·容量)。
fn 個数制限なしナップサック(品物: &[(usize, i64)], 容量: usize) -> i64 {
    let mut dp = vec![0i64; 容量 + 1];
    for &(重さ, 価値) in 品物 {
        for w in 重さ..=容量 {
            dp[w] = dp[w].max(dp[w - 重さ] + 価値);
        }
    }
    dp[容量]
}

#[snippet("価値_基準_ナップサック")]
#[snippet("ナップサック")]
#[snippet("重さ_最小化_ナップサック")]
/// 価値を添字にする 0/1 ナップサック。容量が大きく（〜10^9）価値合計が小さいとき向け。
/// 「価値合計をちょうど v にする最小の重さ」を全 v について求め、
/// 重さが `容量` 以下で達成できる最大の価値 v を返す。O(N·Σ価値)。
fn 価値基準ナップサック(品物: &[(usize, i64)], 容量: usize) -> i64 {
    let 価値合計: i64 = 品物.iter().map(|&(_, v)| v).sum();
    const INF: usize = !0 / 2;
    // dp[v] = 価値合計をちょうど v にするための最小重さ
    let mut dp = vec![INF; 価値合計 as usize + 1];
    dp[0] = 0;
    for &(重さ, 価値) in 品物 {
        for v in (価値..=価値合計).rev() {
            let prev = dp[(v - 価値) as usize];
            if prev != INF {
                let cand = prev + 重さ;
                let cur = &mut dp[v as usize];
                if cand < *cur {
                    *cur = cand;
                }
            }
        }
    }
    (0..=価値合計)
        .rev()
        .find(|&v| dp[v as usize] <= 容量)
        .unwrap_or(0)
}

#[snippet("個数_制限付き_ナップサック")]
#[snippet("ナップサック")]
#[snippet("個数_制限_ナップサック")]
#[snippet("BoundedKnapsack")]
/// 個数制限付きナップサック。各品物 `(重さ, 価値, 個数)` を `0..=個数` 個まで使い、総重量が `容量`
/// 以下で価値和を最大化する。O(N·容量)。
///
/// 単調デックを使うので計算量は `個数` に依存しない。各 1 個までなら [`ナップサック`]、
/// 無制限なら [`個数制限なしナップサック`]。
fn 個数制限付きナップサック(品物: &[(usize, i64, usize)], 容量: usize) -> i64 {
    use std::collections::VecDeque;
    let mut dp = vec![0i64; 容量 + 1]; // dp[j] = 重量 j 以下での最大価値
    for &(w, v, c) in 品物 {
        if c == 0 {
            continue;
        }
        if w == 0 {
            // 重さ 0：価値が正なら個数ぶん全部取る。
            if v > 0 {
                let 加算 = v * c as i64;
                for x in dp.iter_mut() {
                    *x += 加算;
                }
            }
            continue;
        }
        let 旧 = dp.clone();
        // 剰余 r ごとに k = 0,1,2,… の列を単調デックで処理。
        for r in 0..w {
            if r > 容量 {
                break;
            }
            // h(k') = 旧[r + k'·w] − k'·v の窓 [k−c, k] 最大に k·v を足す。
            let mut デック: VecDeque<usize> = VecDeque::new();
            let mut k = 0usize;
            while r + k * w <= 容量 {
                let h = 旧[r + k * w] - (k as i64) * v;
                while let Some(&b) = デック.back() {
                    let hb = 旧[r + b * w] - (b as i64) * v;
                    if hb <= h {
                        デック.pop_back();
                    } else {
                        break;
                    }
                }
                デック.push_back(k);
                while let Some(&前) = デック.front() {
                    if 前 + c < k {
                        デック.pop_front();
                    } else {
                        break;
                    }
                }
                let &先頭 = デック.front().unwrap();
                let 最大h = 旧[r + 先頭 * w] - (先頭 as i64) * v;
                dp[r + k * w] = 最大h + (k as i64) * v;
                k += 1;
            }
        }
    }
    dp[容量]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 基本的な01ナップサック() {
        // 典型: (重さ,価値) = (2,3),(1,2),(3,6),(2,1),(1,3),(5,85)
        let 品物 = [(2, 3), (1, 2), (3, 6), (2, 1), (1, 3), (5, 85)];
        assert_eq!(ナップサック(&品物, 9), 94);
        assert_eq!(価値基準ナップサック(&品物, 9), 94);
    }

    #[test]
    fn 個数無制限() {
        // 重さ3価値4と重さ4価値5と重さ5価値6、容量8 → 3+3+? 重さ6価値8 か 4+4 価値10
        let 品物 = [(3, 4), (4, 5), (5, 6)];
        assert_eq!(個数制限なしナップサック(&品物, 8), 10); // (4,5)×2
    }

    #[test]
    fn 容量0は0() {
        let 品物 = [(1, 100)];
        assert_eq!(ナップサック(&品物, 0), 0);
    }

    // 各品物を個数ぶんのユニットに展開した素朴 0/1 ナップサック。
    fn 素朴個数制限(品物: &[(usize, i64, usize)], 容量: usize) -> i64 {
        let mut dp = vec![0i64; 容量 + 1];
        for &(w, v, c) in 品物 {
            for _ in 0..c {
                if w == 0 {
                    if v > 0 {
                        for x in dp.iter_mut() {
                            *x += v;
                        }
                    }
                    continue;
                }
                for j in (w..=容量).rev() {
                    dp[j] = dp[j].max(dp[j - w] + v);
                }
            }
        }
        dp[容量]
    }

    #[test]
    fn 個数制限付きの既知例() {
        // 重さ2価値3を2個まで、重さ3価値4を1個まで、容量5 → 7。
        assert_eq!(個数制限付きナップサック(&[(2, 3, 2), (3, 4, 1)], 5), 7);
        // 重さ1価値1を3個まで、容量10 → 3。
        assert_eq!(個数制限付きナップサック(&[(1, 1, 3)], 10), 3);
        // 重さ0価値5を4個 → 容量0でも 20。
        assert_eq!(個数制限付きナップサック(&[(0, 5, 4)], 0), 20);
        // 容量0、正の重さ → 0。
        assert_eq!(個数制限付きナップサック(&[(2, 100, 5)], 0), 0);
    }

    #[test]
    fn 個数制限付きが素朴と一致() {
        let mut seed = 0x4e71_a2c0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..3000 {
            let 品数 = (乱数() as usize % 6) + 1;
            let 容量 = 乱数() as usize % 25;
            let mut 品物 = Vec::new();
            for _ in 0..品数 {
                let w = 乱数() as usize % 6; // 0..=5
                let v = (乱数() % 12) as i64 - 1; // -1..=10
                let c = (乱数() as usize % 4) + 1; // 1..=4
                品物.push((w, v, c));
            }
            assert_eq!(
                個数制限付きナップサック(&品物, 容量),
                素朴個数制限(&品物, 容量),
                "品物={品物:?} 容量={容量}"
            );
        }
    }
}
