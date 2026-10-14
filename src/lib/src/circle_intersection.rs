use cargo_snippet::snippet;

#[snippet("CircleIntersection")]
#[snippet("円と_円の_交点")]
#[snippet("円と_直線の_交点")]
#[snippet("円の_交点")]
type 点 = (f64, f64);

#[snippet("円の_交点")]
#[snippet("円と_円の_交点")]
#[snippet("円と_直線の_交点")]
#[snippet("CircleIntersection")]
/// 2 つの円（中心 `c1` 半径 `r1`、中心 `c2` 半径 `r2`）の交点を返す。`0`/`1`（接する）/`2` 個。
///
/// 中心間距離 `d` が `r1+r2` より大きい・`|r1-r2|` より小さい・同心（`d≈0`）のときは交わらない。
/// [`segment_intersection`](crate::segment_intersection) は直線・線分用で円は扱えない。
fn 円と円の交点(c1: 点, r1: f64, c2: 点, r2: f64) -> Vec<点> {
    const EPS: f64 = 1e-9;
    let dx = c2.0 - c1.0;
    let dy = c2.1 - c1.1;
    let d2 = dx * dx + dy * dy;
    let d = d2.sqrt();
    if d < EPS {
        return Vec::new(); // 同心（無限 or 0）。交点列としては空を返す。
    }
    if d > r1 + r2 + EPS || d < (r1 - r2).abs() - EPS {
        return Vec::new(); // 離れすぎ／内包
    }
    let a = (r1 * r1 - r2 * r2 + d2) / (2.0 * d); // c1 から交線への符号付き距離
    let h2 = r1 * r1 - a * a;
    let h = if h2 < 0.0 { 0.0 } else { h2.sqrt() };
    // 交線の足。
    let px = c1.0 + a * dx / d;
    let py = c1.1 + a * dy / d;
    if h < EPS {
        return vec![(px, py)]; // 接する
    }
    // (dx, dy) に垂直な単位ベクトル × h。
    let ox = -dy / d * h;
    let oy = dx / d * h;
    vec![(px + ox, py + oy), (px - ox, py - oy)]
}

#[snippet("円の_交点")]
#[snippet("円と_直線の_交点")]
#[snippet("CircleIntersection")]
/// 円（中心 `c` 半径 `r`）と直線（`a`, `b` を通る）の交点を返す。`0`/`1`（接する）/`2` 個。
fn 円と直線の交点(c: 点, r: f64, a: 点, b: 点) -> Vec<点> {
    const EPS: f64 = 1e-9;
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let 長2 = dx * dx + dy * dy;
    if 長2 < EPS {
        return Vec::new(); // a == b は直線にならない
    }
    // c を直線に射影した足 f。
    let t = ((c.0 - a.0) * dx + (c.1 - a.1) * dy) / 長2;
    let fx = a.0 + t * dx;
    let fy = a.1 + t * dy;
    let dist2 = (c.0 - fx) * (c.0 - fx) + (c.1 - fy) * (c.1 - fy);
    if dist2 > r * r + EPS {
        return Vec::new(); // 交わらない
    }
    let k2 = r * r - dist2;
    let k = if k2 < 0.0 { 0.0 } else { k2.sqrt() };
    let 長 = 長2.sqrt();
    if k < EPS {
        return vec![(fx, fy)]; // 接する
    }
    let ux = dx / 長 * k;
    let uy = dy / 長 * k;
    vec![(fx + ux, fy + uy), (fx - ux, fy - uy)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 近い(a: 点, b: 点) -> bool {
        (a.0 - b.0).hypot(a.1 - b.1) < 1e-6
    }
    fn 円上(c: 点, r: f64, p: 点) -> bool {
        ((p.0 - c.0).hypot(p.1 - c.1) - r).abs() < 1e-6
    }
    fn 直線上(a: 点, b: 点, p: 点) -> bool {
        let cross = (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
        cross.abs() < 1e-6
    }

    #[test]
    fn 既知例() {
        // 2 円が 2 点で交わる。
        let v = 円と円の交点((0.0, 0.0), 5.0, (8.0, 0.0), 5.0);
        assert_eq!(v.len(), 2);
        for p in &v {
            assert!(円上((0.0, 0.0), 5.0, *p) && 円上((8.0, 0.0), 5.0, *p));
        }
        // 外接（接する）。
        let v = 円と円の交点((0.0, 0.0), 2.0, (5.0, 0.0), 3.0);
        assert_eq!(v.len(), 1);
        assert!(近い(v[0], (2.0, 0.0)));
        // 離れている。
        assert!(円と円の交点((0.0, 0.0), 1.0, (10.0, 0.0), 1.0).is_empty());
        // 円と直線。
        let v = 円と直線の交点((0.0, 0.0), 1.0, (-2.0, 0.0), (2.0, 0.0));
        assert_eq!(v.len(), 2);
        assert!(v.iter().any(|p| 近い(*p, (1.0, 0.0))) && v.iter().any(|p| 近い(*p, (-1.0, 0.0))));
        // 接線。
        let v = 円と直線の交点((0.0, 0.0), 1.0, (-2.0, 1.0), (2.0, 1.0));
        assert_eq!(v.len(), 1);
        assert!(近い(v[0], (0.0, 1.0)));
    }

    #[test]
    fn 乱数で交点が両方に乗る() {
        let mut seed = 0xc14c_1e00u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        let mut 座標 = || (乱数() % 2001) as f64 / 100.0 - 10.0; // -10..10
        for _ in 0..20000 {
            // 既知の交点 P を作り、それを通る 2 円を構成 → 必ず交わる。
            let p = (座標(), 座標());
            let c1 = (座標(), 座標());
            let c2 = (座標(), 座標());
            let r1 = (p.0 - c1.0).hypot(p.1 - c1.1);
            let r2 = (p.0 - c2.0).hypot(p.1 - c2.1);
            if (c1.0 - c2.0).hypot(c1.1 - c2.1) < 1e-6 {
                continue; // 同心は除外
            }
            let v = 円と円の交点(c1, r1, c2, r2);
            assert!(!v.is_empty(), "交点が無い c1={c1:?} c2={c2:?}");
            for q in &v {
                assert!(円上(c1, r1, *q) && 円上(c2, r2, *q), "円上でない {q:?}");
            }
            assert!(v.iter().any(|q| 近い(*q, p)), "既知交点 P が見つからない");

            // 円と直線も同様に。中心 c1・P を通る円と、P・b を通る直線。
            let b = (座標(), 座標());
            if (p.0 - b.0).hypot(p.1 - b.1) < 1e-6 {
                continue;
            }
            let v = 円と直線の交点(c1, r1, p, b);
            assert!(!v.is_empty());
            for q in &v {
                assert!(円上(c1, r1, *q) && 直線上(p, b, *q), "直線交点が不正 {q:?}");
            }
            assert!(v.iter().any(|q| 近い(*q, p)));
        }
    }
}
