use cargo_snippet::snippet;

#[snippet("位相_順序_数え上げ")]
#[snippet("線形_拡大_数")]
#[snippet("トポロジカル_順序_数")]
#[snippet("LinearExtensionCount")]
/// DAG（半順序）の**位相順序（線形拡大）の総数**を `法` で数える。頂点数 `N ≤ 20` 程度向けの
/// 部分集合 DP。`O(2^N · N)`。
///
/// `辺` は有向辺 `(u, v)`（`u → v`、`u` が `v` より前）。閉路があれば 0。
/// 半順序集合の線形拡大数（ソートの仕方の数）を求めるのに使う。
#[allow(clippy::needless_range_loop)] // v をビット 1<<v と前提配列の添字に併用する
fn 位相順序数(頂点数: usize, 辺: &[(usize, usize)], 法: u64) -> u64 {
    let n = 頂点数;
    if n == 0 {
        return 1 % 法;
    }
    assert!(n <= 31, "頂点数が多すぎる（2^N が大きすぎる）: {n}");
    // 各頂点の前提集合（入ってくる辺の始点のビットマスク）。
    let mut 前提 = vec![0u32; n];
    for &(u, v) in 辺 {
        if u == v {
            return 0; // 自己ループは閉路なので線形拡大は存在しない
        }
        前提[v] |= 1 << u;
    }
    let 全体 = (1u32 << n) - 1;
    let mut dp = vec![0u64; 1 << n];
    dp[0] = 1 % 法;
    for s in 0..=全体 {
        if dp[s as usize] == 0 {
            continue;
        }
        let cur = dp[s as usize];
        for v in 0..n {
            let bit = 1u32 << v;
            // v が未配置で、前提がすべて配置済みなら追加できる。
            if s & bit == 0 && 前提[v] & s == 前提[v] {
                let ns = (s | bit) as usize;
                dp[ns] = (dp[ns] + cur) % 法;
            }
        }
    }
    dp[全体 as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: u64 = 998_244_353;

    // 全 N! 順列のうち、すべての辺 (u→v) で u が v より前に来るものを数える素朴版。
    fn 素朴(頂点数: usize, 辺: &[(usize, usize)]) -> u64 {
        let n = 頂点数;
        let mut 順: Vec<usize> = (0..n).collect();
        let mut 数 = 0u64;
        // ヒープのアルゴリズムでなく単純な next_permutation 相当を昇順生成で。
        順.sort_unstable();
        loop {
            // 位置を引く。
            let mut 位置 = vec![0usize; n];
            for (i, &x) in 順.iter().enumerate() {
                位置[x] = i;
            }
            if 辺.iter().all(|&(u, v)| 位置[u] < 位置[v]) {
                数 += 1;
            }
            if !次の順列(&mut 順) {
                break;
            }
        }
        数
    }

    // 辞書順で次の順列を作る（無ければ false）。
    fn 次の順列(a: &mut [usize]) -> bool {
        let n = a.len();
        if n <= 1 {
            return false;
        }
        let mut i = n - 1;
        while i > 0 && a[i - 1] >= a[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = n - 1;
        while a[j] <= a[i - 1] {
            j -= 1;
        }
        a.swap(i - 1, j);
        a[i..].reverse();
        true
    }

    #[test]
    fn 既知例() {
        // 辺なし N 頂点 → N!。
        assert_eq!(位相順序数(4, &[], P), 24);
        // 全順序 0<1<2<3 → 1 通り。
        assert_eq!(位相順序数(4, &[(0, 1), (1, 2), (2, 3)], P), 1);
        // 0 が 1,2 の前（残りは自由）：1 を先頭固定にできるのは… 0<1, 0<2 のとき順序は 0 が最初、
        // その後 1,2 は任意 → 2 通り。
        assert_eq!(位相順序数(3, &[(0, 1), (0, 2)], P), 2);
        // 閉路 → 0。
        assert_eq!(位相順序数(3, &[(0, 1), (1, 2), (2, 0)], P), 0);
        // 頂点 0 個 → 1（空の並べ方が 1 通り）。
        assert_eq!(位相順序数(0, &[], P), 1);
    }

    #[test]
    fn 乱数で素朴と一致() {
        let mut seed = 0x4ad9_31c0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..2000 {
            let n = (乱数() as usize % 6) + 1; // 1..=6
            let m = 乱数() as usize % 8;
            let 辺: Vec<(usize, usize)> = (0..m)
                .map(|_| {
                    let u = 乱数() as usize % n;
                    let v = 乱数() as usize % n;
                    (u, v)
                })
                .collect();
            assert_eq!(位相順序数(n, &辺, P), 素朴(n, &辺) % P, "n={n} 辺={辺:?}");
        }
    }
}
