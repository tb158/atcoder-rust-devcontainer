use cargo_snippet::snippet;
#[snippet("L1_距離の_最小_全域木")]
#[snippet("ManhattanMST")]
#[snippet("マンハッタン_MST")]
#[snippet("マンハッタン_最小_全域木")]
use std::collections::BTreeMap;

#[snippet("マンハッタン_最小_全域木")]
#[snippet("ManhattanMST")]
#[snippet("マンハッタン_MST")]
#[snippet("L1_距離の_最小_全域木")]
/// マンハッタン距離 `|dx| + |dy|` を辺重みとする完全グラフの最小全域木（の総重み）を O(N log N) で求める。
///
/// 完全グラフは辺が O(N^2) 本あるが、各点について「8 方向（octant）ごとの最近点」だけが MST 候補になる
/// という性質を使い、候補辺を O(N) 本に絞ってから Kruskal する。点が 1 個以下なら 0。
fn マンハッタン最小全域木(点群: &[(i64, i64)]) -> i64 {
    let n = 点群.len();
    if n <= 1 {
        return 0;
    }
    let mut 点 = 点群.to_vec();
    let mut 辺: Vec<(i64, usize, usize)> = Vec::new();
    let mut id: Vec<usize> = (0..n).collect();
    for k in 0..4 {
        // x + y 昇順に並べ、(-y) をキーにしたスイープで octant {dx >= dy >= 0} の最近点を拾う。
        id.sort_by_key(|&i| 点[i].0 + 点[i].1);
        let mut スイープ: BTreeMap<i64, usize> = BTreeMap::new();
        for &i in &id {
            let mut 削除 = Vec::new();
            for (&キー, &j) in スイープ.range(-点[i].1..) {
                let dx = 点[i].0 - 点[j].0;
                let dy = 点[i].1 - 点[j].1;
                if dy > dx {
                    break;
                }
                辺.push((dx + dy, i, j));
                削除.push(キー);
            }
            for キー in 削除 {
                スイープ.remove(&キー);
            }
            スイープ.insert(-点[i].1, i);
        }
        // 座標を変換して残り 3 個の octant 対を順に処理する。
        for p in 点.iter_mut() {
            if k % 2 == 1 {
                p.0 = -p.0;
            } else {
                std::mem::swap(&mut p.0, &mut p.1);
            }
        }
    }
    // Kruskal
    辺.sort_by_key(|e| e.0);
    let mut 親: Vec<usize> = (0..n).collect();
    fn 根(親: &mut [usize], x: usize) -> usize {
        let mut x = x;
        while 親[x] != x {
            親[x] = 親[親[x]];
            x = 親[x];
        }
        x
    }
    let mut 合計 = 0;
    let mut 本数 = 0;
    for &(w, u, v) in &辺 {
        let a = 根(&mut 親, u);
        let b = 根(&mut 親, v);
        if a != b {
            親[a] = b;
            合計 += w;
            本数 += 1;
            if 本数 == n - 1 {
                break;
            }
        }
    }
    合計
}

#[cfg(test)]
mod tests {
    use super::*;

    // 総当たり: L1 完全グラフを Prim で MST にする。
    fn 総当たり(点群: &[(i64, i64)]) -> i64 {
        let n = 点群.len();
        if n <= 1 {
            return 0;
        }
        let 重み =
            |i: usize, j: usize| (点群[i].0 - 点群[j].0).abs() + (点群[i].1 - 点群[j].1).abs();
        let mut 連結 = vec![false; n];
        let mut 距離 = vec![i64::MAX; n];
        距離[0] = 0;
        let mut 合計 = 0;
        for _ in 0..n {
            let mut 選 = usize::MAX;
            for i in 0..n {
                if !連結[i] && (選 == usize::MAX || 距離[i] < 距離[選]) {
                    選 = i;
                }
            }
            連結[選] = true;
            合計 += 距離[選];
            for i in 0..n {
                if !連結[i] {
                    距離[i] = 距離[i].min(重み(選, i));
                }
            }
        }
        合計
    }

    #[test]
    fn 正方形() {
        // 単位正方形の 4 頂点。MST は 3 辺 = 3。
        let p = vec![(0, 0), (0, 1), (1, 0), (1, 1)];
        assert_eq!(マンハッタン最小全域木(&p), 3);
    }

    #[test]
    fn 点が1個以下は0() {
        assert_eq!(マンハッタン最小全域木(&[]), 0);
        assert_eq!(マンハッタン最小全域木(&[(3, 7)]), 0);
    }

    #[test]
    fn 一直線() {
        let p = vec![(0, 0), (5, 0), (2, 0), (9, 0)];
        assert_eq!(マンハッタン最小全域木(&p), 9);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0xfeed_face_u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..200 {
            let n = (乱数() as usize % 30) + 1;
            let p: Vec<(i64, i64)> = (0..n)
                .map(|_| ((乱数() % 100) as i64 - 50, (乱数() % 100) as i64 - 50))
                .collect();
            assert_eq!(マンハッタン最小全域木(&p), 総当たり(&p), "p={p:?}");
        }
    }
}
