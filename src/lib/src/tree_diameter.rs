use crate::graph_build::{無向グラフ, 重み付き無向グラフ};
use cargo_snippet::snippet;

#[snippet("木の_直径", include = "重み付き_無向_グラフ")]
#[snippet("木の_最長_パス")]
#[snippet("直径")]
/// 木の直径（最も離れた2頂点間の距離）を求める。重み付き無向木に対応。O(N)。
///
/// `辺` は AtCoder の入力そのまま（0-indexed の `(u, v, 重み)`、重みは非負を想定）。
/// 重みなしの木なら重みをすべて 1 にして渡す。戻り値は `(直径の長さ, 端点1, 端点2)`。
///
/// 「任意の点から最も遠い点 u」「u から最も遠い点 v」を取ると u-v が直径になる、を利用。
fn 木の直径(n: usize, 辺: &[(usize, usize, isize)]) -> (isize, usize, usize) {
    // 始点から最も遠い頂点とその距離を返す（明示スタックで DFS）
    fn 最遠(グラフ: &[Vec<(usize, isize)>], 始点: usize) -> (isize, usize) {
        let mut 距離 = vec![-1isize; グラフ.len()];
        距離[始点] = 0;
        let mut 探索 = vec![始点];
        let (mut 最大距離, mut 最遠点) = (0isize, 始点);
        while let Some(now) = 探索.pop() {
            if 距離[now] > 最大距離 {
                最大距離 = 距離[now];
                最遠点 = now;
            }
            for &(next, w) in &グラフ[now] {
                if 距離[next] < 0 {
                    距離[next] = 距離[now] + w;
                    探索.push(next);
                }
            }
        }
        (最大距離, 最遠点)
    }

    let グラフ = 重み付き無向グラフ(n, 辺);
    let (_, u) = 最遠(&グラフ, 0);
    let (直径, v) = 最遠(&グラフ, u);
    (直径, u, v)
}

#[snippet("木の_中心", include = "無向_グラフ")]
#[snippet("tree_center")]
/// 木の中心を求める（再帰なし）。中心とは、そこから最も遠い頂点までの距離（離心数）が
/// 最小になる頂点。重みなしの木の中心は 1 個か 2 個あり、直径パスの真ん中に位置する。O(N)。
///
/// `辺` は AtCoder の入力そのまま（0-indexed の頂点対）。戻り値は中心の頂点番号（昇順、長さ 1 か 2）。
/// 中心の離心数（木の半径）は `⌈直径 / 2⌉`。2 個のときは隣接していて、その間の辺の中点が真の中心。
///
/// [`木の重心`](crate::tree_centroid::木の重心)（除去後の最大成分を最小化）とは別物。
/// 中心は「最遠距離の最小化」、重心は「部分木サイズの均衡」で、長い枝と多数の葉が混ざるとずれる。
fn 木の中心(n: usize, 辺: &[(usize, usize)]) -> Vec<usize> {
    // 始点からの距離（辺数）と、始点を根としたときの親を返す（明示スタックで DFS）
    fn 距離と親(グラフ: &[Vec<usize>], 始点: usize) -> (Vec<usize>, Vec<usize>) {
        let mut 距離 = vec![!0; グラフ.len()];
        let mut prev = vec![!0; グラフ.len()];
        距離[始点] = 0;
        let mut 探索 = vec![始点];
        while let Some(now) = 探索.pop() {
            for &next in &グラフ[now] {
                if 距離[next] == !0 {
                    距離[next] = 距離[now] + 1;
                    prev[next] = now;
                    探索.push(next);
                }
            }
        }
        (距離, prev)
    }

    if n <= 1 {
        return (0..n).collect();
    }
    let グラフ = 無向グラフ(n, 辺);
    let (距離, _) = 距離と親(&グラフ, 0);
    let u = (0..n).max_by_key(|&v| 距離[v]).unwrap();
    let (距離, prev) = 距離と親(&グラフ, u);
    let v = (0..n).max_by_key(|&w| 距離[w]).unwrap();
    let 直径 = 距離[v];

    // 直径パス v → u を ⌊直径/2⌋ 歩進んだ頂点が中心。奇数長ならその次の頂点も中心
    let mut now = v;
    for _ in 0..直径 / 2 {
        now = prev[now];
    }
    let mut 中心 = vec![now];
    if 直径 % 2 == 1 {
        中心.push(prev[now]);
    }
    中心.sort();
    中心
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 直線の木() {
        // 0-1-2-3-4（各辺重み1）
        let 辺: Vec<(usize, usize, isize)> = (0..4).map(|i| (i, i + 1, 1)).collect();
        let (d, _, _) = 木の直径(5, &辺);
        assert_eq!(d, 4);
    }

    #[test]
    fn 重み付きの木() {
        // 0-1(重み5), 1-2(重み3), 1-3(重み10)
        let (d, _, _) = 木の直径(4, &[(0, 1, 5), (1, 2, 3), (1, 3, 10)]);
        assert_eq!(d, 15); // 0 -> 1 -> 3
    }

    #[test]
    fn 中心_奇数長の鎖は中央() {
        // 0-1-2-3-4 → 2
        assert_eq!(木の中心(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]), vec![2]);
    }

    #[test]
    fn 中心_偶数長の鎖は2個() {
        // 0-1-2-3 → 1 と 2
        assert_eq!(木の中心(4, &[(0, 1), (1, 2), (2, 3)]), vec![1, 2]);
    }

    #[test]
    fn 中心_小さい木() {
        assert_eq!(木の中心(0, &[]), Vec::<usize>::new());
        assert_eq!(木の中心(1, &[]), vec![0]);
        assert_eq!(木の中心(2, &[(1, 0)]), vec![0, 1]);
    }

    #[test]
    fn 中心は重心と異なりうる() {
        // 鎖 0-1-2-3-4-5-6 の頂点 1 に葉 7..=12 をぶら下げる。
        // 中心は鎖の中央 3（離心数 3）、重心は葉が集まる 1。
        let mut 辺: Vec<(usize, usize)> = (0..6).map(|i| (i, i + 1)).collect();
        辺.extend((7..13).map(|i| (1, i)));
        assert_eq!(木の中心(13, &辺), vec![3]);
    }

    #[test]
    fn 中心_乱数で総当たりと一致() {
        let mut seed = 0x5ce7_7e40u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..2000 {
            let n = (乱数() as usize % 12) + 1; // 1..=12
            // ランダム木（頂点 i を 0..i の親に繋ぐ）を、ランダムな番号付けに置き換える
            let mut 番号: Vec<usize> = (0..n).collect();
            for i in (1..n).rev() {
                番号.swap(i, 乱数() as usize % (i + 1));
            }
            let 辺: Vec<(usize, usize)> = (1..n)
                .map(|i| (番号[乱数() as usize % i], 番号[i]))
                .collect();

            // 総当たり：全頂点の離心数を Floyd–Warshall で求め、最小のものを列挙
            let mut d = vec![vec![usize::MAX / 2; n]; n];
            for (i, 行) in d.iter_mut().enumerate() {
                行[i] = 0;
            }
            for &(a, b) in &辺 {
                d[a][b] = 1;
                d[b][a] = 1;
            }
            for k in 0..n {
                for i in 0..n {
                    for j in 0..n {
                        d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
                    }
                }
            }
            let 離心数: Vec<usize> = (0..n).map(|i| *d[i].iter().max().unwrap()).collect();
            let 最小 = *離心数.iter().min().unwrap();
            let 期待: Vec<usize> = (0..n).filter(|&i| 離心数[i] == 最小).collect();
            assert_eq!(木の中心(n, &辺), 期待, "n={n} 辺={辺:?}");
        }
    }
}
