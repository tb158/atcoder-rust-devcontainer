use cargo_snippet::snippet;

#[snippet("GeometryDistance")]
#[snippet("点と_直線の_距離")]
#[snippet("点と_線分の_距離")]
#[snippet("線分間_距離")]
type 点 = (f64, f64);

#[snippet("点と_線分の_距離")]
#[snippet("線分間_距離")]
#[snippet("点と_直線の_距離")]
#[snippet("GeometryDistance")]
/// 点 `p` と、点 `a`・`b` を通る直線との距離。`O(1)`。`a == b` のときは点 `a` との距離。
fn 点と直線の距離(p: 点, a: 点, b: 点) -> f64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let 長2 = dx * dx + dy * dy;
    if 長2 == 0.0 {
        return (p.0 - a.0).hypot(p.1 - a.1);
    }
    ((dx * (p.1 - a.1) - dy * (p.0 - a.0)).abs()) / 長2.sqrt()
}

#[snippet("点と_線分の_距離")]
#[snippet("線分間_距離")]
#[snippet("点と_直線の_距離")]
#[snippet("GeometryDistance")]
/// 点 `p` と線分 `[a, b]` との最短距離。`O(1)`。
///
/// `p` の射影が線分内に落ちれば垂線の長さ、外れれば近い方の端点との距離。
fn 点と線分の距離(p: 点, a: 点, b: 点) -> f64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let 長2 = dx * dx + dy * dy;
    if 長2 == 0.0 {
        return (p.0 - a.0).hypot(p.1 - a.1);
    }
    let t = ((p.0 - a.0) * dx + (p.1 - a.1) * dy) / 長2;
    let tc = t.clamp(0.0, 1.0); // 線分内に制限
    let fx = a.0 + tc * dx;
    let fy = a.1 + tc * dy;
    (p.0 - fx).hypot(p.1 - fy)
}

#[snippet("線分間_距離")]
#[snippet("点と_線分の_距離")]
#[snippet("GeometryDistance")]
/// 線分 `[a, b]` と線分 `[c, d]` の最短距離。`O(1)`。交差していれば `0`。
fn 線分と線分の距離(a: 点, b: 点, c: 点, d: 点) -> f64 {
    // 向きの符号で真の交差を判定（境界は端点距離 0 で拾えるので厳密一致は不要）。
    let 外積 = |o: 点, x: 点, y: 点| (x.0 - o.0) * (y.1 - o.1) - (x.1 - o.1) * (y.0 - o.0);
    let d1 = 外積(c, d, a);
    let d2 = 外積(c, d, b);
    let d3 = 外積(a, b, c);
    let d4 = 外積(a, b, d);
    if d1 * d2 < 0.0 && d3 * d4 < 0.0 {
        return 0.0; // 内部で交差
    }
    点と線分の距離(a, c, d)
        .min(点と線分の距離(b, c, d))
        .min(点と線分の距離(c, a, b))
        .min(点と線分の距離(d, a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 既知例() {
        // 点 (0,2) と x 軸線分 [(-1,0),(1,0)]: 垂線で距離 2。
        assert!((点と線分の距離((0.0, 2.0), (-1.0, 0.0), (1.0, 0.0)) - 2.0).abs() < 1e-9);
        // 射影が外れる場合は端点距離。
        assert!((点と線分の距離((3.0, 0.0), (-1.0, 0.0), (1.0, 0.0)) - 2.0).abs() < 1e-9);
        // 交差する線分は距離 0。
        assert!(線分と線分の距離((0.0, -1.0), (0.0, 1.0), (-1.0, 0.0), (1.0, 0.0)) < 1e-9);
        // 点と直線は線分と違い延長線まで含む。
        assert!((点と直線の距離((3.0, 2.0), (-1.0, 0.0), (1.0, 0.0)) - 2.0).abs() < 1e-9);
    }

    #[test]
    fn 点と線分が標本最小と一致() {
        let mut seed = 0x9e21_dd00u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        let mut 座標 = || (乱数() % 21) as f64 - 10.0;
        for _ in 0..20000 {
            let p = (座標(), 座標());
            let a = (座標(), 座標());
            let b = (座標(), 座標());
            let 閉 = 点と線分の距離(p, a, b);
            // 線分上を細かく標本して最小距離を取る（閉形式は標本最小以下のはず）。
            let mut 標本 = f64::INFINITY;
            for i in 0..=2000 {
                let t = i as f64 / 2000.0;
                let x = a.0 + t * (b.0 - a.0);
                let y = a.1 + t * (b.1 - a.1);
                標本 = 標本.min((p.0 - x).hypot(p.1 - y));
            }
            assert!(
                閉 <= 標本 + 1e-9,
                "閉形式が標本最小を超過 閉={閉} 標本={標本}"
            );
            assert!(標本 - 閉 < 0.05, "標本最小と乖離 閉={閉} 標本={標本}");
        }
    }

    #[test]
    fn 線分間が標本最小と一致() {
        let mut seed = 0x4ab2_99c0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        let mut 座標 = || (乱数() % 13) as f64 - 6.0;
        for _ in 0..3000 {
            let a = (座標(), 座標());
            let b = (座標(), 座標());
            let c = (座標(), 座標());
            let d = (座標(), 座標());
            let 閉 = 線分と線分の距離(a, b, c, d);
            let mut 標本 = f64::INFINITY;
            for i in 0..=120 {
                let t = i as f64 / 120.0;
                let x1 = a.0 + t * (b.0 - a.0);
                let y1 = a.1 + t * (b.1 - a.1);
                for j in 0..=120 {
                    let s = j as f64 / 120.0;
                    let x2 = c.0 + s * (d.0 - c.0);
                    let y2 = c.1 + s * (d.1 - c.1);
                    標本 = 標本.min((x1 - x2).hypot(y1 - y2));
                }
            }
            assert!(閉 <= 標本 + 1e-9, "閉={閉} 標本={標本}");
            assert!(標本 - 閉 < 0.1, "閉={閉} 標本={標本}");
        }
    }
}
