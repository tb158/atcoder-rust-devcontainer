use cargo_snippet::snippet;

#[snippet("全域_最小_カット")]
#[snippet("StoerWagner")]
#[snippet("無向_最小_カット")]
#[snippet("GlobalMinCut")]
/// 無向重み付きグラフの**全域最小カット**（global minimum cut）を Stoer–Wagner 法で求める。
/// 始点・終点を指定せず「頂点集合を空でない 2 つに分ける切断のうち、またぐ辺の重み和が最小のもの」を返す。
///
/// `n` 頂点、`辺` は `(u, v, 重み)` の無向辺（重み非負、多重辺は加算、自己ループは無視）。計算量 `O(n^3)`。
/// `ac_library` の `maxflow` は s–t 間の最小カット専用で、全ペアを試さずに大域最小カットを出すのはこちら。
/// `n <= 1` のときは `0` を返す。
fn 全域最小カット(n: usize, 辺: &[(usize, usize, isize)]) -> isize {
    if n <= 1 {
        return 0;
    }
    let mut グラフ = vec![vec![0isize; n]; n];
    for &(u, v, w) in 辺 {
        if u != v {
            グラフ[u][v] += w;
            グラフ[v][u] += w;
        }
    }
    let mut 最良 = isize::MAX;
    let mut 頂点: Vec<usize> = (0..n).collect();
    while 頂点.len() > 1 {
        let m = 頂点.len();
        let mut 接続重み = vec![0isize; n]; // A への接続重み
        let mut 取込済 = vec![false; n];
        let mut 順序 = Vec::with_capacity(m);
        for _ in 0..m {
            // A に最も強く接続する未取り込み頂点を選ぶ。
            let mut 選 = !0;
            for &v in &頂点 {
                if !取込済[v] && (選 == !0 || 接続重み[v] > 接続重み[選]) {
                    選 = v;
                }
            }
            取込済[選] = true;
            順序.push(選);
            for &v in &頂点 {
                if !取込済[v] {
                    接続重み[v] += グラフ[選][v];
                }
            }
        }
        let t = 順序[m - 1];
        let s = 順序[m - 2];
        最良 = 最良.min(接続重み[t]); // フェーズのカット = t の A への接続重み
        // s に t を縮約する。
        for &v in &頂点 {
            グラフ[s][v] += グラフ[t][v];
            グラフ[v][s] += グラフ[v][t];
        }
        頂点.retain(|&x| x != t);
    }
    最良
}

#[cfg(test)]
mod tests {
    use super::*;

    // 総当たり: 頂点 0 を含む側の部分集合を全列挙してまたぐ重みの最小を取る。
    fn 総当たり(n: usize, 辺: &[(usize, usize, isize)]) -> isize {
        if n <= 1 {
            return 0;
        }
        let mut 最良 = isize::MAX;
        // 0 は常に S 側に固定（対称性）。残り n-1 頂点を全列挙（全部 S 側＝T 空の自明分割は除く）。
        for マスク in 0..(1u32 << (n - 1)) - 1 {
            let 側 = |v: usize| -> bool {
                if v == 0 {
                    true
                } else {
                    (マスク >> (v - 1)) & 1 == 1
                }
            };
            let mut 切断 = 0isize;
            for &(u, v, w) in 辺 {
                if u != v && 側(u) != 側(v) {
                    切断 += w;
                }
            }
            最良 = 最良.min(切断);
        }
        最良
    }

    #[test]
    fn 小さな例() {
        // 4 頂点の正方形 + 対角、最小カットは頂点 1 個を切り離す重み。
        let 辺 = [(0, 1, 3), (1, 2, 1), (2, 3, 4), (3, 0, 2), (0, 2, 5)];
        assert_eq!(全域最小カット(4, &辺), 総当たり(4, &辺));
    }

    #[test]
    fn 非連結は0() {
        let 辺 = [(0, 1, 7), (2, 3, 9)];
        assert_eq!(全域最小カット(4, &辺), 0);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x3141_5926u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..400 {
            let n = (乱数() as usize % 7) + 2; // 2..=8
            let 本数 = 乱数() as usize % (n * 2 + 1);
            let mut 辺 = Vec::new();
            for _ in 0..本数 {
                let u = 乱数() as usize % n;
                let v = 乱数() as usize % n;
                if u != v {
                    let w = (乱数() % 9) as isize + 1;
                    辺.push((u, v, w));
                }
            }
            assert_eq!(全域最小カット(n, &辺), 総当たり(n, &辺), "n={n} 辺={辺:?}");
        }
    }
}
