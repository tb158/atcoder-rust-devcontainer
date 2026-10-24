use cargo_snippet::snippet;

#[snippet("ConvexContains")]
#[snippet("MinkowskiSum")]
#[snippet("ミンコフスキー_和")]
#[snippet("凸_多角形_内包")]
#[snippet("凸_多角形_点_判定")]
type 点 = (isize, isize);

#[snippet("凸_多角形_内包")]
#[snippet("凸_多角形_点_判定")]
#[snippet("ConvexContains")]
/// 反時計回りの凸多角形 `多角形` に点 `q` が含まれるかを `O(log N)` で判定する。
/// `2` = 内部、`1` = 辺・頂点上、`0` = 外部。座標は整数のまま厳密判定。
///
/// 多角形は**狭義凸**（連続 3 頂点が共線でない）かつ反時計回りであることを前提とする。
/// [`点の内外`](crate::polygon::点の内外) は任意の単純多角形を `O(N)` で判定する別物。
fn 凸多角形内包(多角形: &[点], q: 点) -> i32 {
    let p = 多角形;
    let n = p.len();
    let 外積 = |o: 点, a: 点, b: 点| -> i128 {
        (a.0 - o.0) as i128 * (b.1 - o.1) as i128 - (a.1 - o.1) as i128 * (b.0 - o.0) as i128
    };
    let 線分上 = |a: 点, b: 点| (q.0 - a.0) * (q.0 - b.0) <= 0 && (q.1 - a.1) * (q.1 - b.1) <= 0;
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return if q == p[0] { 1 } else { 0 };
    }
    if n == 2 {
        return if 外積(p[0], p[1], q) == 0 && 線分上(p[0], p[1]) {
            1
        } else {
            0
        };
    }
    let d1 = 外積(p[0], p[1], q);
    let d2 = 外積(p[0], p[n - 1], q);
    if d1 < 0 || d2 > 0 {
        return 0; // 扇の外
    }
    // p[0]→q を挟む隣接頂点 lo, lo+1 を二分探索。
    let mut lo = 1usize;
    let mut hi = n - 1;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if 外積(p[0], p[mid], q) >= 0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let c = 外積(p[lo], p[hi], q); // hi == lo + 1
    if c < 0 {
        return 0;
    }
    if c == 0 {
        return if 線分上(p[lo], p[hi]) { 1 } else { 0 };
    }
    if d1 == 0 {
        return if 線分上(p[0], p[1]) { 1 } else { 0 };
    }
    if d2 == 0 {
        return if 線分上(p[0], p[n - 1]) { 1 } else { 0 };
    }
    2
}

#[snippet("ミンコフスキー_和")]
#[snippet("MinkowskiSum")]
/// 2 つの反時計回り凸多角形 `a`, `b` のミンコフスキー和 `{p + q | p∈A, q∈B}` の輪郭を
/// 反時計回りで返す。`O(N + M)`。
fn ミンコフスキー和(a: &[点], b: &[点]) -> Vec<点> {
    let 整列 = |p: &[点]| -> Vec<点> {
        let n = p.len();
        let mut k = 0;
        for i in 1..n {
            if p[i].1 < p[k].1 || (p[i].1 == p[k].1 && p[i].0 < p[k].0) {
                k = i;
            }
        }
        (0..n).map(|i| p[(k + i) % n]).collect()
    };
    let a = 整列(a);
    let b = 整列(b);
    let n = a.len();
    let m = b.len();
    let mut 結果 = Vec::with_capacity(n + m);
    let (mut i, mut j) = (0usize, 0usize);
    while i < n || j < m {
        結果.push((a[i % n].0 + b[j % m].0, a[i % n].1 + b[j % m].1));
        let ae = (a[(i + 1) % n].0 - a[i % n].0, a[(i + 1) % n].1 - a[i % n].1);
        let be = (b[(j + 1) % m].0 - b[j % m].0, b[(j + 1) % m].1 - b[j % m].1);
        let cross = ae.0 as i128 * be.1 as i128 - ae.1 as i128 * be.0 as i128;
        // 偏角の小さい辺を採用。同偏角なら両方進める。
        let 進i = i < n && (j >= m || cross >= 0);
        let 進j = j < m && (i >= n || cross <= 0);
        if 進i {
            i += 1;
        }
        if 進j {
            j += 1;
        }
    }
    結果
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::polygon::点の内外;

    // 反時計回り凸包（共線中間点は除く）。テスト用の素朴版。
    fn 凸包(点群: &[点]) -> Vec<点> {
        let mut 点: Vec<点> = 点群.to_vec();
        点.sort();
        点.dedup();
        if 点.len() <= 2 {
            return 点;
        }
        let 外積 = |o: 点, a: 点, b: 点| -> i128 {
            (a.0 - o.0) as i128 * (b.1 - o.1) as i128 - (a.1 - o.1) as i128 * (b.0 - o.0) as i128
        };
        let mut 下: Vec<点> = Vec::new();
        for &p in &点 {
            while 下.len() >= 2 && 外積(下[下.len() - 2], 下[下.len() - 1], p) <= 0 {
                下.pop();
            }
            下.push(p);
        }
        let mut 上: Vec<点> = Vec::new();
        for &p in 点.iter().rev() {
            while 上.len() >= 2 && 外積(上[上.len() - 2], 上[上.len() - 1], p) <= 0 {
                上.pop();
            }
            上.push(p);
        }
        下.pop();
        上.pop();
        下.extend(上);
        下
    }

    #[test]
    fn 内包の既知例() {
        let 正方形 = vec![(0, 0), (4, 0), (4, 4), (0, 4)]; // CCW
        assert_eq!(凸多角形内包(&正方形, (2, 2)), 2); // 内部
        assert_eq!(凸多角形内包(&正方形, (4, 2)), 1); // 辺上
        assert_eq!(凸多角形内包(&正方形, (0, 0)), 1); // 頂点
        assert_eq!(凸多角形内包(&正方形, (5, 2)), 0); // 外部
        assert_eq!(凸多角形内包(&正方形, (2, 5)), 0);
    }

    #[test]
    fn 内包が点の内外と一致() {
        let mut seed = 0x4f12_a7c0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..2000 {
            // ランダム点群の凸包を狭義凸多角形として使う。
            let k = (乱数() as usize % 10) + 3;
            let 点群: Vec<点> = (0..k)
                .map(|_| ((乱数() % 13) as isize - 6, (乱数() % 13) as isize - 6))
                .collect();
            let 凸 = 凸包(&点群);
            if 凸.len() < 3 {
                continue; // 退化（共線・点不足）は除外
            }
            // 外接矩形内の格子点すべてでクエリ。
            for x in -7..=7 {
                for y in -7..=7 {
                    let q = (x, y);
                    let 速 = 凸多角形内包(&凸, q);
                    let 遅 = 点の内外(&凸, q);
                    assert_eq!(速, 遅, "凸={凸:?} q={q:?} 速={速} 遅={遅}");
                }
            }
        }
    }

    #[test]
    fn ミンコフスキー和が総当たり凸包と一致() {
        let mut seed = 0x2b88_d310u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..2000 {
            let mk = |乱: &mut dyn FnMut() -> u64| -> Vec<点> {
                let k = (乱() as usize % 8) + 3;
                let g: Vec<点> = (0..k)
                    .map(|_| ((乱() % 11) as isize - 5, (乱() % 11) as isize - 5))
                    .collect();
                凸包(&g)
            };
            let a = mk(&mut 乱数);
            let b = mk(&mut 乱数);
            if a.len() < 3 || b.len() < 3 {
                continue;
            }
            let 和 = ミンコフスキー和(&a, &b);
            // 総当たり: 全ペア和の凸包。
            let mut 全ペア = Vec::new();
            for &p in &a {
                for &q in &b {
                    全ペア.push((p.0 + q.0, p.1 + q.1));
                }
            }
            let mut 期待 = 凸包(&全ペア);
            let mut 実際 = 凸包(&和); // 共線点を落として正規化
            期待.sort();
            実際.sort();
            assert_eq!(実際, 期待, "a={a:?} b={b:?}");
        }
    }
}
