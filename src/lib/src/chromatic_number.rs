use cargo_snippet::snippet;

#[snippet("彩色数")]
#[snippet("ChromaticNumber")]
#[snippet("グラフ_彩色")]
#[snippet("クロマチック_数")]
/// 無向グラフの彩色数（隣接頂点を異色で塗り分けるのに必要な最小色数）を O(2^n · n) で求める。頂点数 `n <= 20`。
///
/// 包除原理: 「`k` 個の独立集合（順序付き・重複可）で全頂点を覆う場合の数」
/// `f(k) = Σ_S (-1)^{n-|S|} · g(S)^k`（`g(S)` は `S` に含まれる独立集合の個数）が正になる最小の `k` が彩色数。
fn 彩色数(n: usize, 辺: &[(usize, usize)]) -> usize {
    assert!(n <= 20, "n must be <= 20");
    if n == 0 {
        return 0;
    }
    let mut グラフ = vec![0u32; n];
    for &(u, v) in 辺 {
        if u != v {
            グラフ[u] |= 1 << v;
            グラフ[v] |= 1 << u;
        }
    }
    let 全 = 1usize << n;
    // g(S): S に含まれる独立集合の個数。最下位ビット v について
    // g(S) = g(S\{v}) + g(S \ ({v} ∪ N(v)))。
    let mut g = vec![0u64; 全];
    g[0] = 1;
    for s in 1..全 {
        let v = s.trailing_zeros() as usize;
        let 無v = s & !(1 << v);
        g[s] = g[無v] + g[無v & !(グラフ[v] as usize)];
    }
    // 大きな素数 2 つの法で f(k) の非零を判定する。
    const 素数: [u64; 2] = [1_000_000_007, 1_000_000_009];
    let mut 現在 = [vec![1u64; 全], vec![1u64; 全]]; // g(S)^k mod p（k=0 で 1）
    for k in 1..=n {
        let mut 非零 = false;
        for (idx, &p) in 素数.iter().enumerate() {
            let mut 和: i64 = 0;
            // `s` is the subset bitmask itself: used both as an index and via popcount for the sign.
            #[allow(clippy::needless_range_loop)]
            for s in 0..全 {
                現在[idx][s] = 現在[idx][s] * (g[s] % p) % p;
                let 符号 = if (n - s.count_ones() as usize) % 2 == 0 {
                    1i64
                } else {
                    -1
                };
                和 += 符号 * 現在[idx][s] as i64;
            }
            if 和 % p as i64 != 0 {
                非零 = true;
            }
        }
        if 非零 {
            return k;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    // 総当たり: k 色での彩色可能性をバックトラックで判定し、最小 k を返す。
    fn 総当たり(n: usize, 辺: &[(usize, usize)]) -> usize {
        if n == 0 {
            return 0;
        }
        let mut 隣接 = vec![vec![false; n]; n];
        for &(u, v) in 辺 {
            if u != v {
                隣接[u][v] = true;
                隣接[v][u] = true;
            }
        }
        fn 塗れる(
            v: usize,
            n: usize,
            k: usize,
            色: &mut Vec<usize>,
            隣接: &[Vec<bool>],
        ) -> bool {
            if v == n {
                return true;
            }
            for c in 0..k {
                let 衝突 = (0..v).any(|u| 隣接[v][u] && 色[u] == c);
                if !衝突 {
                    色[v] = c;
                    if 塗れる(v + 1, n, k, 色, 隣接) {
                        return true;
                    }
                }
            }
            false
        }
        for k in 1..=n {
            let mut 色 = vec![usize::MAX; n];
            if 塗れる(0, n, k, &mut 色, &隣接) {
                return k;
            }
        }
        n
    }

    #[test]
    fn 三角形は3() {
        assert_eq!(彩色数(3, &[(0, 1), (1, 2), (2, 0)]), 3);
    }

    #[test]
    fn 二部グラフは2() {
        // 4 頂点の経路 0-1-2-3 は 2 彩色
        assert_eq!(彩色数(4, &[(0, 1), (1, 2), (2, 3)]), 2);
    }

    #[test]
    fn 辺なしは1() {
        assert_eq!(彩色数(5, &[]), 1);
        assert_eq!(彩色数(0, &[]), 0);
    }

    #[test]
    fn 偶数長閉路は2_奇数長は3() {
        // C4
        assert_eq!(彩色数(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]), 2);
        // C5
        assert_eq!(彩色数(5, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]), 3);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x55aa_33ccu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..150 {
            let n = (乱数() as usize % 7) + 1; // 1..=7
            let mut 辺 = Vec::new();
            for u in 0..n {
                for v in u + 1..n {
                    if 乱数() % 2 == 0 {
                        辺.push((u, v));
                    }
                }
            }
            assert_eq!(彩色数(n, &辺), 総当たり(n, &辺), "n={n} 辺={辺:?}");
        }
    }
}
