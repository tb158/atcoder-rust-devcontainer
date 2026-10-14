use crate::mod_pow::累乗_mod;
use cargo_snippet::snippet;

#[snippet("有向_全域木_数")]
#[snippet("内向き_全域木")]
#[snippet("mod_行列式", include = "累乗_mod")]
#[snippet("行列式_mod")]
#[snippet("行列木_定理")]
#[snippet("全域木_数え上げ")]
#[snippet("キルヒホッフ")]
#[snippet("有向_全域木_数え上げ")]
#[snippet("ArborescenceCount")]
/// 素数 `法` 上での正方行列の行列式を掃き出し（前進消去）で求める。O(n^3)。
///
/// `法` は**素数**であること（フェルマーの小定理で逆元を取るため）。
/// 行列はムーブで受け取り破壊的に消去する。`法 < 2^31` 程度を想定。
fn 行列式_mod(mut 行列: Vec<Vec<u64>>, 法: u64) -> u64 {
    let n = 行列.len();
    for row in &mut 行列 {
        for x in row.iter_mut() {
            *x %= 法;
        }
    }
    let mut 結果 = 1u64 % 法;
    for 列 in 0..n {
        // この列で 0 でない行をピボットに選ぶ
        let ピボット = (列..n).find(|&r| 行列[r][列] != 0);
        let Some(p) = ピボット else {
            return 0;
        };
        if p != 列 {
            行列.swap(p, 列);
            結果 = (法 - 結果) % 法; // 行の入れ替えで符号反転
        }
        let 逆 = 累乗_mod(行列[列][列], 法 - 2, 法);
        結果 = 結果 * 行列[列][列] % 法;
        for r in 列 + 1..n {
            let 係数 = 行列[r][列] * 逆 % 法;
            if 係数 != 0 {
                for c in 列..n {
                    行列[r][c] = (行列[r][c] + 法 - 係数 * 行列[列][c] % 法) % 法;
                }
            }
        }
    }
    結果
}

#[snippet("mod_行列式")]
#[snippet("行列式_mod")]
#[snippet("行列木_定理")]
#[snippet("全域木_数え上げ")]
#[snippet("キルヒホッフ")]
/// 行列木定理（キルヒホッフの定理）で、無向グラフの全域木の個数を `法`（素数）で求める。O(V^3)。
///
/// `辺` は無向辺 `(u, v)`（`0..頂点数`、多重辺可・自己ループは無視推奨）。ラプラシアン行列
/// から 1 行 1 列を除いた小行列の行列式が全域木数に等しい。連結でなければ 0。
/// 頂点 1 個のときは（辺のない木が 1 通りとして）1 を返す。
fn 全域木数_mod(頂点数: usize, 辺: &[(usize, usize)], 法: u64) -> u64 {
    if 頂点数 <= 1 {
        return 1 % 法;
    }
    let n = 頂点数;
    let mut l = vec![vec![0u64; n]; n];
    for &(u, v) in 辺 {
        if u == v {
            continue;
        }
        l[u][u] += 1;
        l[v][v] += 1;
        l[u][v] = (l[u][v] + 法 - 1) % 法;
        l[v][u] = (l[v][u] + 法 - 1) % 法;
    }
    for (i, row) in l.iter_mut().enumerate() {
        row[i] %= 法;
    }
    // 末尾の行・列を除いた小行列
    let 小行列: Vec<Vec<u64>> = (0..n - 1).map(|i| l[i][0..n - 1].to_vec()).collect();
    行列式_mod(小行列, 法)
}

#[snippet("有向_全域木_数え上げ")]
#[snippet("ArborescenceCount")]
#[snippet("有向_全域木_数")]
#[snippet("内向き_全域木")]
/// 有向版の行列木定理（Tutte の定理）。有向グラフで、根 `根` から**葉に向かって**辺が伸びる
/// 全域有向木（arborescence。根以外の各頂点の入次数がちょうど 1）の個数を `法`（素数）で数える。
/// O(V^3)。
///
/// `辺` は有向辺 `(u, v)`（`u → v`、`0..頂点数`、多重辺可・自己ループは無視）。
/// 無向版 [`全域木数_mod`] と違って辺の向きを区別する。
/// 「根に向かう（in-arborescence）」全域木を数えたいときは全辺を逆向きにして渡す。
fn 有向全域木数_mod(頂点数: usize, 辺: &[(usize, usize)], 根: usize, 法: u64) -> u64 {
    if 頂点数 == 0 {
        return 0;
    }
    if 頂点数 == 1 {
        return 1 % 法;
    }
    let n = 頂点数;
    let mut l = vec![vec![0u64; n]; n];
    for &(u, v) in 辺 {
        if u == v {
            continue;
        }
        l[v][v] = (l[v][v] + 1) % 法; // v の入次数
        l[u][v] = (l[u][v] + 法 - 1) % 法; // i→j の本数を引く
    }
    // 根の行・列を除いた小行列。
    let 残: Vec<usize> = (0..n).filter(|&i| i != 根).collect();
    let 小行列: Vec<Vec<u64>> = 残
        .iter()
        .map(|&i| 残.iter().map(|&j| l[i][j]).collect())
        .collect();
    行列式_mod(小行列, 法)
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: u64 = 998_244_353;

    #[test]
    fn 単位行列の行列式は1() {
        let m = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(行列式_mod(m, P), 1);
    }

    #[test]
    fn 既知の行列式と一致() {
        // |1 2; 3 4| = -2 ≡ P-2
        let m = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(行列式_mod(m, P), P - 2);
        // |2 0 0; 0 3 0; 0 0 5| = 30
        let d = vec![vec![2, 0, 0], vec![0, 3, 0], vec![0, 0, 5]];
        assert_eq!(行列式_mod(d, P), 30);
    }

    #[test]
    fn 特異行列は0() {
        let m = vec![vec![1, 2, 3], vec![2, 4, 6], vec![1, 0, 1]];
        assert_eq!(行列式_mod(m, P), 0);
    }

    #[test]
    fn ケイリーの公式() {
        // 完全グラフ K_n の全域木数は n^(n-2)
        for n in 2..=6usize {
            let mut 辺 = Vec::new();
            for u in 0..n {
                for v in u + 1..n {
                    辺.push((u, v));
                }
            }
            let 期待 = if n == 2 {
                1
            } else {
                (n as u64).pow(n as u32 - 2)
            };
            assert_eq!(全域木数_mod(n, &辺, P), 期待 % P, "K_{n}");
        }
    }

    #[test]
    fn 木は全域木1個_非連結は0() {
        // パス 0-1-2-3 はそれ自身が唯一の全域木
        assert_eq!(全域木数_mod(4, &[(0, 1), (1, 2), (2, 3)], P), 1);
        // 非連結
        assert_eq!(全域木数_mod(4, &[(0, 1), (2, 3)], P), 0);
    }

    // 各非根頂点に入辺を 1 本ずつ割り当て、全頂点が根に到達する（サイクル無し）組合せ数を数える素朴版。
    fn 素朴有向全域木(n: usize, 辺: &[(usize, usize)], 根: usize) -> u64 {
        let mut 入: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(u, v) in 辺 {
            if u != v {
                入[v].push(u);
            }
        }
        let 非根: Vec<usize> = (0..n).filter(|&i| i != 根).collect();
        if 非根.iter().any(|&v| 入[v].is_empty()) {
            return 0; // どれかが入辺を持たない → 不可能
        }
        let mut count = 0u64;
        let mut idx = vec![0usize; 非根.len()];
        loop {
            let mut 親 = vec![usize::MAX; n];
            for (k, &v) in 非根.iter().enumerate() {
                親[v] = 入[v][idx[k]];
            }
            let 妥当 = 非根.iter().all(|&start| {
                let mut cur = start;
                let mut steps = 0;
                while cur != 根 {
                    if 親[cur] == usize::MAX || steps > n {
                        return false;
                    }
                    cur = 親[cur];
                    steps += 1;
                }
                true
            });
            if 妥当 {
                count += 1;
            }
            // 桁上げ式に次の組合せへ。
            let mut k = 0;
            loop {
                if k == 非根.len() {
                    return count;
                }
                idx[k] += 1;
                if idx[k] < 入[非根[k]].len() {
                    break;
                }
                idx[k] = 0;
                k += 1;
            }
        }
    }

    #[test]
    fn 有向全域木の既知例() {
        // 有向三角形 0→1→2→0。根 0 から葉へ向かう全域木は {0→1,1→2} の 1 通り。
        let 環 = [(0, 1), (1, 2), (2, 0)];
        assert_eq!(有向全域木数_mod(3, &環, 0, P), 1);
        assert_eq!(有向全域木数_mod(3, &環, 1, P), 1);
        // スター 0→1,0→2,0→3（根 0）→ それ自身のみ 1。
        assert_eq!(有向全域木数_mod(4, &[(0, 1), (0, 2), (0, 3)], 0, P), 1);
        // 0→1,0→2,1→2（根 0）→ 頂点2の親は 0 か 1 の 2 通り。
        assert_eq!(有向全域木数_mod(3, &[(0, 1), (0, 2), (1, 2)], 0, P), 2);
        // 完全有向グラフ K_4（全 12 有向辺）の根固定 arborescence は n^(n-2)=16。
        let mut k4 = Vec::new();
        for u in 0..4 {
            for v in 0..4 {
                if u != v {
                    k4.push((u, v));
                }
            }
        }
        assert_eq!(有向全域木数_mod(4, &k4, 0, P), 16);
        // どこからも根へ行けない → 0。
        assert_eq!(有向全域木数_mod(3, &[(0, 1), (0, 2)], 1, P), 0);
        // 頂点 1 個。
        assert_eq!(有向全域木数_mod(1, &[], 0, P), 1);
    }

    #[test]
    fn 有向全域木が素朴と一致() {
        let mut seed = 0x6c1d_a930u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..3000 {
            let n = (乱数() as usize % 5) + 2; // 2..=6
            let m = (乱数() as usize % 8) + 1; // 辺数（多重辺あり得る）
            let 辺: Vec<(usize, usize)> = (0..m)
                .map(|_| {
                    let u = 乱数() as usize % n;
                    let v = 乱数() as usize % n;
                    (u, v)
                })
                .collect();
            let 根 = 乱数() as usize % n;
            assert_eq!(
                有向全域木数_mod(n, &辺, 根, P),
                素朴有向全域木(n, &辺, 根) % P,
                "n={n} 辺={辺:?} 根={根}"
            );
        }
    }
}
