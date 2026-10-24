use cargo_snippet::snippet;

#[snippet("長方形_和集合の_面積")]
#[snippet("矩形_和集合_面積")]
#[snippet("RectangleUnionArea")]
/// 軸平行長方形の**和集合の面積**（重なりを 1 回だけ数えた被覆面積）を返す。各長方形は
/// `(x1, y1, x2, y2)`（`x1<x2`, `y1<y2`、半開矩形 `[x1,x2)×[y1,y2)`）。`O(N log N)`。
///
/// x 座標圧縮 + y 方向の平面走査（Klee の長方形面積アルゴリズム）。
///
/// [`segment_intersection`](crate::segment_intersection) は線分交差、[`imos`](crate::imos) は格子の
/// 一括加算で、いずれも「連続座標の和集合面積」は扱わない。
fn 長方形和集合の面積(矩形: &[(isize, isize, isize, isize)]) -> isize {
    if 矩形.is_empty() {
        return 0;
    }
    // x 座標の圧縮。
    let mut xs: Vec<isize> = Vec::with_capacity(矩形.len() * 2);
    for &(x1, _, x2, _) in 矩形 {
        if x1 < x2 {
            xs.push(x1);
            xs.push(x2);
        }
    }
    xs.sort_unstable();
    xs.dedup();
    let m = xs.len();
    if m < 2 {
        return 0;
    }
    // 走査イベント (y, x区間左, x区間右, ±1)。
    let mut イベント: Vec<(isize, usize, usize, isize)> = Vec::new();
    for &(x1, y1, x2, y2) in 矩形 {
        if x1 >= x2 || y1 >= y2 {
            continue;
        }
        let i1 = xs.partition_point(|&v| v < x1);
        let i2 = xs.partition_point(|&v| v < x2);
        イベント.push((y1, i1, i2, 1));
        イベント.push((y2, i1, i2, -1));
    }
    if イベント.is_empty() {
        return 0;
    }
    イベント.sort_by_key(|&(y, _, _, _)| y);
    let mut seg = 面積セグ::new(xs);
    let mut 面積 = 0isize;
    let mut 前y = イベント[0].0;
    for (y, i1, i2, d) in イベント {
        面積 += seg.被覆長() * (y - 前y);
        seg.更新(1, 0, m - 1, i1, i2, d);
        前y = y;
    }
    面積
}

#[snippet("長方形_和集合の_面積")]
#[snippet("矩形_和集合_面積")]
#[snippet("RectangleUnionArea")]
// x 区間の被覆カウントと被覆長を持つセグメント木（葉 = 圧縮 x の隣接区間）。
struct 面積セグ {
    xs: Vec<isize>,
    count: Vec<isize>,
    len: Vec<isize>,
}

#[snippet("長方形_和集合の_面積")]
#[snippet("矩形_和集合_面積")]
#[snippet("RectangleUnionArea")]
impl 面積セグ {
    fn new(xs: Vec<isize>) -> Self {
        let size = 4 * xs.len().max(1);
        面積セグ {
            xs,
            count: vec![0; size],
            len: vec![0; size],
        }
    }

    fn 被覆長(&self) -> isize {
        self.len[1]
    }

    // [nl, nr) は xs の添字区間（実座標 [xs[nl], xs[nr]) を覆う）。
    fn 更新(&mut self, node: usize, nl: usize, nr: usize, ql: usize, qr: usize, d: isize) {
        if qr <= nl || nr <= ql {
            return;
        }
        if ql <= nl && nr <= qr {
            self.count[node] += d;
        } else {
            let mid = (nl + nr) / 2;
            self.更新(2 * node, nl, mid, ql, qr, d);
            self.更新(2 * node + 1, mid, nr, ql, qr, d);
        }
        self.len[node] = if self.count[node] > 0 {
            self.xs[nr] - self.xs[nl] // この区間は丸ごと覆われている
        } else if nr - nl <= 1 {
            0 // 葉でカウント 0 → 被覆なし
        } else {
            self.len[2 * node] + self.len[2 * node + 1]
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // 単位セルを塗りつぶす素朴オラクル（座標が小さい整数のときのみ有効）。
    fn 素朴(矩形: &[(isize, isize, isize, isize)]) -> isize {
        let mut セル = HashSet::new();
        for &(x1, y1, x2, y2) in 矩形 {
            for x in x1..x2 {
                for y in y1..y2 {
                    セル.insert((x, y));
                }
            }
        }
        セル.len() as isize
    }

    #[test]
    fn 既知例() {
        // 重ならない 2 つの 2×2 → 8。
        assert_eq!(長方形和集合の面積(&[(0, 0, 2, 2), (5, 5, 7, 7)]), 8);
        // 完全に重なる → 1 つぶん。
        assert_eq!(長方形和集合の面積(&[(0, 0, 3, 3), (0, 0, 3, 3)]), 9);
        // L 字に重なる：[0,0,2,2] と [1,1,3,3] → 4+4-1(重なり1×1) = 7。
        assert_eq!(長方形和集合の面積(&[(0, 0, 2, 2), (1, 1, 3, 3)]), 7);
        // 空・退化。
        assert_eq!(長方形和集合の面積(&[]), 0);
        assert_eq!(長方形和集合の面積(&[(0, 0, 0, 5)]), 0);
    }

    #[test]
    fn 乱数で素朴と一致() {
        let mut seed = 0x2f9a_71c0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..2000 {
            let n = (乱数() as usize % 8) + 1; // 1..=8
            let 矩形: Vec<(isize, isize, isize, isize)> = (0..n)
                .map(|_| {
                    let x1 = (乱数() % 12) as isize;
                    let y1 = (乱数() % 12) as isize;
                    let w = (乱数() % 6) as isize + 1;
                    let h = (乱数() % 6) as isize + 1;
                    (x1, y1, x1 + w, y1 + h)
                })
                .collect();
            assert_eq!(長方形和集合の面積(&矩形), 素朴(&矩形), "矩形={矩形:?}");
        }
    }
}
