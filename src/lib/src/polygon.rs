use cargo_snippet::snippet;

#[snippet("ピックの_定理")]
#[snippet("Pick")]
#[snippet("格子点")]
#[snippet("多角形")]
#[snippet("符号付き_面積")]
#[snippet("Polygon")]
/// 多角形（頂点を順に並べた整数座標列）の符号付き面積の 2 倍を返す。O(N)。
/// 反時計回りなら正、時計回りなら負。
///
/// 整数座標のまま厳密に計算するため、実面積は `面積2倍(p).abs() as f64 / 2.0`。
fn 符号付き面積2倍(多角形: &[(i64, i64)]) -> i64 {
    let n = 多角形.len();
    let mut s = 0i64;
    for i in 0..n {
        let (ax, ay) = 多角形[i];
        let (bx, by) = 多角形[(i + 1) % n];
        s += ax * by - bx * ay;
    }
    s
}

#[snippet("多角形")]
#[snippet("内外_判定")]
#[snippet("PointInPolygon")]
#[snippet("点の_内外")]
/// 点が多角形の内部・境界・外部のどれにあるかを整数演算のみで判定する。O(N)。
/// `2` = 内部、`1` = 境界上、`0` = 外部。凸でない単純多角形にも使える（射影法／レイキャスティング）。
pub fn 点の内外(多角形: &[(i64, i64)], 点: (i64, i64)) -> i32 {
    let n = 多角形.len();
    let (px, py) = 点;
    let mut 内 = false;
    for i in 0..n {
        let (xi, yi) = 多角形[i];
        let (xj, yj) = 多角形[(i + 1) % n];
        // 境界判定: 線分 (i,j) 上に点があるか（共線かつ外接矩形内）。
        let 外積 = (xj - xi) * (py - yi) - (yj - yi) * (px - xi);
        if 外積 == 0 && (px - xi) * (px - xj) <= 0 && (py - yi) * (py - yj) <= 0 {
            return 1;
        }
        // 右向きレイとの交差数を数える。
        if (yi > py) != (yj > py) {
            // px < 交点x ⇔ 符号付き判定（除算を避ける）
            let 左 = if yj - yi > 0 { 外積 > 0 } else { 外積 < 0 };
            if 左 {
                内 = !内;
            }
        }
    }
    if 内 { 2 } else { 0 }
}

#[snippet("多角形")]
#[snippet("凸_判定")]
#[snippet("ConvexPolygon")]
/// 頂点列が凸多角形をなすか（同一向きに曲がり続けるか）を判定する。O(N)。
/// 共線な 3 点は許容する。`n < 3` は `false`。
fn 凸多角形か(多角形: &[(i64, i64)]) -> bool {
    let n = 多角形.len();
    if n < 3 {
        return false;
    }
    let mut 符号 = 0i64;
    for i in 0..n {
        let (ax, ay) = 多角形[i];
        let (bx, by) = 多角形[(i + 1) % n];
        let (cx, cy) = 多角形[(i + 2) % n];
        let 外積 = (bx - ax) * (cy - by) - (by - ay) * (cx - bx);
        if 外積 != 0 {
            let s = 外積.signum();
            if 符号 == 0 {
                符号 = s;
            } else if 符号 != s {
                return false;
            }
        }
    }
    true
}

#[snippet("ピックの_定理")]
#[snippet("Pick")]
#[snippet("格子点")]
#[snippet("多角形")]
/// 多角形の辺上にある格子点の個数を返す（各辺の `gcd(|dx|, |dy|)` の総和）。O(N log)。
fn 境界格子点数(多角形: &[(i64, i64)]) -> i64 {
    let n = 多角形.len();
    let mut b = 0i64;
    for i in 0..n {
        let (ax, ay) = 多角形[i];
        let (bx, by) = 多角形[(i + 1) % n];
        b += gcd((ax - bx).abs(), (ay - by).abs());
    }
    b
}

#[snippet("ピックの_定理")]
#[snippet("Pick")]
#[snippet("格子点")]
#[snippet("多角形")]
/// ユークリッドの互除法（[`内部格子点数`] / [`境界格子点数`] 用の内部関数）。
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[snippet("ピックの_定理")]
#[snippet("Pick")]
#[snippet("格子点")]
#[snippet("多角形")]
/// ピックの定理で多角形**内部**の格子点数 `i = A - b/2 + 1` を返す（`A` は面積、`b` は[`境界格子点数`]）。
/// 整数座標の単純多角形に対して厳密。`A = 符号付き面積2倍.abs()/2`。
fn 内部格子点数(多角形: &[(i64, i64)]) -> i64 {
    let 面積2 = 符号付き面積2倍(多角形).abs();
    let b = 境界格子点数(多角形);
    // i = (2A - b + 2) / 2 = (面積2 - b + 2) / 2
    (面積2 - b + 2) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 面積と内外と凸() {
        // 単位正方形 (0,0)-(2,0)-(2,2)-(0,2)
        let sq = [(0, 0), (2, 0), (2, 2), (0, 2)];
        assert_eq!(符号付き面積2倍(&sq), 8); // 面積 4
        assert_eq!(点の内外(&sq, (1, 1)), 2); // 内部
        assert_eq!(点の内外(&sq, (2, 1)), 1); // 境界
        assert_eq!(点の内外(&sq, (0, 0)), 1); // 頂点
        assert_eq!(点の内外(&sq, (3, 1)), 0); // 外部
        assert!(凸多角形か(&sq));
        // L 字（凹）
        let l = [(0, 0), (4, 0), (4, 2), (2, 2), (2, 4), (0, 4)];
        assert!(!凸多角形か(&l));
    }

    // 反時計回り凸包（Andrew's monotone chain）。
    fn 凸包(mut p: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
        p.sort_unstable();
        p.dedup();
        let n = p.len();
        if n < 3 {
            return p;
        }
        let 外積 = |o: (i64, i64), a: (i64, i64), b: (i64, i64)| {
            (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
        };
        let mut 下: Vec<(i64, i64)> = Vec::new();
        for &pt in &p {
            while 下.len() >= 2 && 外積(下[下.len() - 2], 下[下.len() - 1], pt) <= 0 {
                下.pop();
            }
            下.push(pt);
        }
        let mut 上: Vec<(i64, i64)> = Vec::new();
        for &pt in p.iter().rev() {
            while 上.len() >= 2 && 外積(上[上.len() - 2], 上[上.len() - 1], pt) <= 0 {
                上.pop();
            }
            上.push(pt);
        }
        下.pop();
        上.pop();
        下.extend(上);
        下
    }

    #[test]
    fn ピックと内外を凸包で乱数検証() {
        let mut seed = 0x6b1d_77efu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..300 {
            let 個数 = (乱数() as usize % 12) + 3;
            let p: Vec<(i64, i64)> = (0..個数)
                .map(|_| ((乱数() % 13) as i64, (乱数() % 13) as i64))
                .collect();
            let poly = 凸包(p);
            if poly.len() < 3 {
                continue;
            }
            assert!(凸多角形か(&poly), "hull not convex: {poly:?}");
            // 反時計回りなので面積は正。
            assert!(符号付き面積2倍(&poly) > 0);
            // CCW 凸多角形に対する独立な内外オラクル: 全辺で左側(外積>=0)。
            let n = poly.len();
            let オラクル = |pt: (i64, i64)| -> i32 {
                let mut 最小 = i64::MAX;
                for i in 0..n {
                    let (ax, ay) = poly[i];
                    let (bx, by) = poly[(i + 1) % n];
                    let cr = (bx - ax) * (pt.1 - ay) - (by - ay) * (pt.0 - ax);
                    最小 = 最小.min(cr);
                }
                match 最小.cmp(&0) {
                    std::cmp::Ordering::Greater => 2,
                    std::cmp::Ordering::Equal => 1,
                    std::cmp::Ordering::Less => 0,
                }
            };
            let mut 内 = 0i64;
            let mut 境界 = 0i64;
            for x in -1..=14 {
                for y in -1..=14 {
                    let o = オラクル((x, y));
                    assert_eq!(点の内外(&poly, (x, y)), o, "pt=({x},{y}) poly={poly:?}");
                    match o {
                        2 => 内 += 1,
                        1 => 境界 += 1,
                        _ => {}
                    }
                }
            }
            assert_eq!(境界格子点数(&poly), 境界, "boundary poly={poly:?}");
            assert_eq!(内部格子点数(&poly), 内, "interior poly={poly:?}");
        }
    }
}
