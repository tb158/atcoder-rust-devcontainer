use cargo_snippet::snippet;

#[snippet("TriangleCenters")]
#[snippet("三角形の_中心")]
#[snippet("内心")]
#[snippet("内接円")]
#[snippet("垂心")]
#[snippet("外心")]
#[snippet("外接円")]
type 点 = (f64, f64);

#[snippet("三角形の_中心")]
#[snippet("外接円")]
#[snippet("外心")]
#[snippet("垂心")]
#[snippet("TriangleCenters")]
/// 三角形 `a, b, c` の外接円 `(中心, 半径)` を返す。3 点が共線なら `None`。`O(1)`。
///
/// [`min_enclosing_circle`](crate::min_enclosing_circle) は点群の最小包含円で、外接円とは別。
fn 外接円(a: 点, b: 点, c: 点) -> Option<(点, f64)> {
    let d = 2.0 * (a.0 * (b.1 - c.1) + b.0 * (c.1 - a.1) + c.0 * (a.1 - b.1));
    if d.abs() < 1e-12 {
        return None; // 共線
    }
    let a2 = a.0 * a.0 + a.1 * a.1;
    let b2 = b.0 * b.0 + b.1 * b.1;
    let c2 = c.0 * c.0 + c.1 * c.1;
    let ux = (a2 * (b.1 - c.1) + b2 * (c.1 - a.1) + c2 * (a.1 - b.1)) / d;
    let uy = (a2 * (c.0 - b.0) + b2 * (a.0 - c.0) + c2 * (b.0 - a.0)) / d;
    let 中心 = (ux, uy);
    let r = (中心.0 - a.0).hypot(中心.1 - a.1);
    Some((中心, r))
}

#[snippet("三角形の_中心")]
#[snippet("内接円")]
#[snippet("内心")]
#[snippet("TriangleCenters")]
/// 三角形 `a, b, c` の内接円 `(中心, 半径)` を返す。退化（周長 0）なら `None`。`O(1)`。
fn 内接円(a: 点, b: 点, c: 点) -> Option<(点, f64)> {
    let la = (b.0 - c.0).hypot(b.1 - c.1); // 頂点 a の対辺
    let lb = (c.0 - a.0).hypot(c.1 - a.1);
    let lc = (a.0 - b.0).hypot(a.1 - b.1);
    let 周 = la + lb + lc;
    if 周 < 1e-12 {
        return None;
    }
    let cx = (la * a.0 + lb * b.0 + lc * c.0) / 周;
    let cy = (la * a.1 + lb * b.1 + lc * c.1) / 周;
    let 面積 = ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)).abs() / 2.0;
    let r = 2.0 * 面積 / 周; // = 面積 / 半周長
    Some(((cx, cy), r))
}

#[snippet("三角形の_中心")]
#[snippet("TriangleCenters")]
/// 三角形（任意の 3 点）の重心 `(a + b + c) / 3` を返す。`O(1)`。
fn 重心(a: 点, b: 点, c: 点) -> 点 {
    ((a.0 + b.0 + c.0) / 3.0, (a.1 + b.1 + c.1) / 3.0)
}

#[snippet("三角形の_中心")]
#[snippet("垂心")]
#[snippet("TriangleCenters")]
/// 三角形 `a, b, c` の垂心（各頂点から対辺への垂線の交点）を返す。共線なら `None`。`O(1)`。
fn 垂心(a: 点, b: 点, c: 点) -> Option<点> {
    let (o, _) = 外接円(a, b, c)?;
    Some((a.0 + b.0 + c.0 - 2.0 * o.0, a.1 + b.1 + c.1 - 2.0 * o.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 点と直線の距離(p: 点, a: 点, b: 点) -> f64 {
        let dx = b.0 - a.0;
        let dy = b.1 - a.1;
        (dx * (p.1 - a.1) - dy * (p.0 - a.0)).abs() / dx.hypot(dy)
    }

    #[test]
    fn 既知例() {
        // 直角三角形 (0,0),(4,0),(0,3): 外接円中心は斜辺中点 (2,1.5)、半径 2.5。
        let (o, r) = 外接円((0.0, 0.0), (4.0, 0.0), (0.0, 3.0)).unwrap();
        assert!((o.0 - 2.0).abs() < 1e-9 && (o.1 - 1.5).abs() < 1e-9);
        assert!((r - 2.5).abs() < 1e-9);
        // 内接円半径 = (a+b-c)/2 = (3+4-5)/2 = 1。
        let (_, ir) = 内接円((0.0, 0.0), (4.0, 0.0), (0.0, 3.0)).unwrap();
        assert!((ir - 1.0).abs() < 1e-9);
        // 共線は外接円なし。
        assert!(外接円((0.0, 0.0), (1.0, 1.0), (2.0, 2.0)).is_none());
        // 重心。
        let g = 重心((0.0, 0.0), (3.0, 0.0), (0.0, 3.0));
        assert!((g.0 - 1.0).abs() < 1e-9 && (g.1 - 1.0).abs() < 1e-9);
    }

    #[test]
    fn 乱数で各中心の定義を満たす() {
        let mut seed = 0x71a4_c1e0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        let mut 座標 = || (乱数() % 41) as f64 / 2.0 - 10.0;
        for _ in 0..20000 {
            let a = (座標(), 座標());
            let b = (座標(), 座標());
            let c = (座標(), 座標());
            let 面積2 = (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
            if 面積2.abs() < 1.0 {
                continue; // ほぼ共線は除外（数値誤差回避）
            }
            // 外心は 3 頂点から等距離。
            let (o, r) = 外接円(a, b, c).unwrap();
            for &p in &[a, b, c] {
                assert!(
                    ((o.0 - p.0).hypot(o.1 - p.1) - r).abs() < 1e-6,
                    "外心不一致"
                );
            }
            // 内心は 3 辺から等距離（= 内接円半径）。
            let (ic, ir) = 内接円(a, b, c).unwrap();
            assert!((点と直線の距離(ic, a, b) - ir).abs() < 1e-6, "内心-辺ab");
            assert!((点と直線の距離(ic, b, c) - ir).abs() < 1e-6, "内心-辺bc");
            assert!((点と直線の距離(ic, c, a) - ir).abs() < 1e-6, "内心-辺ca");
            // 垂心: (H-a)·(c-b) = 0（a からの垂線は対辺 bc に直交）。
            let h = 垂心(a, b, c).unwrap();
            let 直交 = (h.0 - a.0) * (c.0 - b.0) + (h.1 - a.1) * (c.1 - b.1);
            assert!(直交.abs() < 1e-4, "垂心の直交性 {直交}");
        }
    }
}
