use crate::hopcroft_karp::ホップクロフトカープ;
use cargo_snippet::snippet;

#[snippet("DAG_最小_パス_被覆", include = "ホップクロフト_カープ")]
#[snippet("最小_パス_被覆")]
#[snippet("MinimumPathCover")]
/// DAG（閉路のない有向グラフ）の**頂点素な最小パス被覆**を求める。すべての頂点をちょうど 1 本の
/// 有向パスに含めるとき、必要なパスの最小本数と、その実際のパス列を返す。`O(E√V)`（二部マッチング）。
///
/// 半順序の比較可能対を辺にすれば、最小パス被覆数は**最大反鎖**の大きさに等しい（Dilworth の定理）。
///
/// [`hopcroft_karp`](crate::hopcroft_karp) は素のマッチング、本関数はその DAG パス被覆への応用。
/// 入力に閉路があると正しい結果を保証しない（事前にトポロジカル性を仮定）。
fn dag最小パス被覆(頂点数: usize, 辺: &[(usize, usize)]) -> (usize, Vec<Vec<usize>>) {
    let mut hk = ホップクロフトカープ::new(頂点数, 頂点数);
    for &(u, v) in 辺 {
        hk.辺追加(u, v);
    }
    let m = hk.最大マッチング();
    let mut next = vec![!0; 頂点数];
    let mut prev = vec![!0; 頂点数];
    for (u, v) in hk.マッチング() {
        next[u] = v;
        prev[v] = u;
    }
    let mut パス = Vec::new();
    for (s, &prev_s) in prev.iter().enumerate() {
        if prev_s == !0 {
            let mut p = Vec::new();
            let mut now = s;
            while now != !0 {
                p.push(now);
                now = next[now];
            }
            パス.push(p);
        }
    }
    (頂点数 - m, パス)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 辺集合のうち「各頂点の出次数・入次数 ≤ 1」を満たす最大本数を全探索（= 最大マッチング）。
    // 最小パス被覆数 = 頂点数 − その本数。
    fn 素朴最小被覆(頂点数: usize, 辺: &[(usize, usize)]) -> usize {
        let e = 辺.len();
        let mut 最大辺 = 0usize;
        for mask in 0u32..(1 << e) {
            let mut 出 = vec![0u8; 頂点数];
            let mut 入 = vec![0u8; 頂点数];
            let mut 良 = true;
            let mut 本 = 0;
            for (i, &(u, v)) in 辺.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    出[u] += 1;
                    入[v] += 1;
                    本 += 1;
                    if 出[u] > 1 || 入[v] > 1 {
                        良 = false;
                        break;
                    }
                }
            }
            if 良 {
                最大辺 = 最大辺.max(本);
            }
        }
        頂点数 - 最大辺
    }

    fn パス検証(
        頂点数: usize, 辺: &[(usize, usize)], 本数: usize, パス: &[Vec<usize>]
    ) {
        let 辺集合: std::collections::HashSet<(usize, usize)> = 辺.iter().copied().collect();
        let mut 出現 = vec![0u32; 頂点数];
        for p in パス {
            for w in p.windows(2) {
                assert!(
                    辺集合.contains(&(w[0], w[1])),
                    "パス辺 {:?} が存在しない",
                    w
                );
            }
            for &v in p {
                出現[v] += 1;
            }
        }
        assert!(出現.iter().all(|&c| c == 1), "全頂点ちょうど1回でない");
        assert_eq!(パス.len(), 本数, "パス本数が値と不一致");
    }

    #[test]
    fn 既知例() {
        // 0→1→2 の鎖 → 1 本。
        let (本, パス) = dag最小パス被覆(3, &[(0, 1), (1, 2)]);
        assert_eq!(本, 1);
        パス検証(3, &[(0, 1), (1, 2)], 本, &パス);
        // 辺なし → 各頂点が 1 本 → 頂点数本。
        let (本, _) = dag最小パス被覆(4, &[]);
        assert_eq!(本, 4);
        // 二分木 0→1,0→2 → 0-1 と 2、または 0-2 と 1 → 2 本。
        let (本, パス) = dag最小パス被覆(3, &[(0, 1), (0, 2)]);
        assert_eq!(本, 2);
        パス検証(3, &[(0, 1), (0, 2)], 本, &パス);
    }

    #[test]
    fn 乱数で素朴と一致() {
        let mut seed = 0x5b8c_31a0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..1500 {
            let n = (乱数() as usize % 6) + 2; // 2..=7
            // i<j の向きだけ辺を張れば必ず DAG。
            let mut 辺 = Vec::new();
            for i in 0..n {
                for j in i + 1..n {
                    if 乱数() % 3 == 0 {
                        辺.push((i, j));
                    }
                }
            }
            if 辺.len() > 18 {
                continue; // 総当たり 2^e を抑える
            }
            let (本, パス) = dag最小パス被覆(n, &辺);
            assert_eq!(本, 素朴最小被覆(n, &辺), "n={n} 辺={辺:?}");
            パス検証(n, &辺, 本, &パス);
        }
    }
}
