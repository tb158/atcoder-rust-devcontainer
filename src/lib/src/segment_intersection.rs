use crate::geometry::外積3点;
use cargo_snippet::snippet;

#[snippet("SegmentIntersection")]
#[snippet("直線_交点")]
#[snippet("線分_交差")]
#[snippet("線分_交差_判定")]
type 点 = (isize, isize);

#[snippet("線分_交差", include = "外積_3点")]
#[snippet("線分_交差_判定")]
#[snippet("SegmentIntersection")]
#[snippet("直線_交点")]
/// 点 `p` が線分 `[a, b]`（端点を含む）の上にあるかを整数演算で厳密に判定する。
///
/// `a == b`（点）にも対応。
fn 点は線分上(p: 点, a: 点, b: 点) -> bool {
    外積3点(a, b, p) == 0
        && p.0 >= a.0.min(b.0)
        && p.0 <= a.0.max(b.0)
        && p.1 >= a.1.min(b.1)
        && p.1 <= a.1.max(b.1)
}

#[snippet("線分_交差")]
#[snippet("線分_交差_判定")]
#[snippet("SegmentIntersection")]
/// 線分 `[a, b]` と `[c, d]` が交わる（端点接触・共線重なりを含む）かを整数演算で厳密に判定する。`O(1)`。
///
/// 退化した線分（点）にも正しく対応。
fn 線分交差(a: 点, b: 点, c: 点, d: 点) -> bool {
    let d1 = 外積3点(c, d, a).signum();
    let d2 = 外積3点(c, d, b).signum();
    let d3 = 外積3点(a, b, c).signum();
    let d4 = 外積3点(a, b, d).signum();
    if d1 != d2 && d3 != d4 {
        return true; // 互いにまたいでいる（真の交差）
    }
    // 端点接触・共線重なりの特例。
    点は線分上(a, c, d) || 点は線分上(b, c, d) || 点は線分上(c, a, b) || 点は線分上(d, a, b)
}

#[snippet("線分_交差")]
#[snippet("直線_交点")]
#[snippet("SegmentIntersection")]
/// 2 直線（`a` と `b` を通る直線、`c` と `d` を通る直線）の交点を `f64` で返す。平行なら `None`。
///
/// 線分ではなく直線として扱う。厳密な交差「有無」は [`線分交差`]、座標が必要なときに本関数。
fn 直線交点(a: 点, b: 点, c: 点, d: 点) -> Option<(f64, f64)> {
    let r = (b.0 - a.0, b.1 - a.1);
    let s = (d.0 - c.0, d.1 - c.1);
    let rxs = r.0 as i128 * s.1 as i128 - r.1 as i128 * s.0 as i128;
    if rxs == 0 {
        return None; // 平行（共線含む）
    }
    let qp = (c.0 - a.0, c.1 - a.1);
    let t = (qp.0 as f64 * s.1 as f64 - qp.1 as f64 * s.0 as f64) / rxs as f64;
    Some((a.0 as f64 + t * r.0 as f64, a.1 as f64 + t * r.1 as f64))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 外積2(a: 点, b: 点) -> i128 {
        a.0 as i128 * b.1 as i128 - a.1 as i128 * b.0 as i128
    }
    fn 内積2(a: 点, b: 点) -> i128 {
        a.0 as i128 * b.0 as i128 + a.1 as i128 * b.1 as i128
    }

    // 媒介変数法による独立なオラクル（向き符号法とは別の導出）。
    fn 交差オラクル(a: 点, b: 点, c: 点, d: 点) -> bool {
        // 退化（点）は線分上判定に帰着。
        if a == b {
            return 点は線分上(a, c, d);
        }
        if c == d {
            return 点は線分上(c, a, b);
        }
        let r = (b.0 - a.0, b.1 - a.1);
        let s = (d.0 - c.0, d.1 - c.1);
        let qp = (c.0 - a.0, c.1 - a.1);
        let rxs = 外積2(r, s);
        let qps = 外積2(qp, s); // (c-a) × s
        let qpr = 外積2(qp, r); // (c-a) × r
        if rxs != 0 {
            // t = qps/rxs, u = qpr/rxs がともに [0,1]。
            let (lo, hi) = if rxs > 0 { (0, rxs) } else { (rxs, 0) };
            (lo <= qps && qps <= hi) && (lo <= qpr && qpr <= hi)
        } else {
            // 平行。共線でなければ交わらない。
            if qpr != 0 {
                return false;
            }
            // 共線: r 方向に射影して 1 次元の重なりを見る。
            let rr = 内積2(r, r); // a→b を [0, rr]
            let tc = 内積2((c.0 - a.0, c.1 - a.1), r);
            let td = 内積2((d.0 - a.0, d.1 - a.1), r);
            let (clo, chi) = (tc.min(td), tc.max(td));
            clo <= rr && 0 <= chi
        }
    }

    #[test]
    fn 既知例() {
        // 十字に交わる。
        assert!(線分交差((0, 0), (2, 2), (0, 2), (2, 0)));
        // 端点で接する。
        assert!(線分交差((0, 0), (1, 1), (1, 1), (2, 0)));
        // 共線で重なる。
        assert!(線分交差((0, 0), (2, 0), (1, 0), (3, 0)));
        // 共線で離れている。
        assert!(!線分交差((0, 0), (1, 0), (2, 0), (3, 0)));
        // 平行で交わらない。
        assert!(!線分交差((0, 0), (2, 0), (0, 1), (2, 1)));
        // 直線交点。
        let p = 直線交点((0, 0), (2, 2), (0, 2), (2, 0)).unwrap();
        assert!((p.0 - 1.0).abs() < 1e-9 && (p.1 - 1.0).abs() < 1e-9);
        assert!(直線交点((0, 0), (1, 0), (0, 1), (1, 1)).is_none()); // 平行
    }

    #[test]
    fn 乱数でオラクルと一致() {
        let mut seed = 0x5e90_17a0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        let mut 座標 = || (乱数() % 7) as isize - 3; // -3..=3
        for _ in 0..200_000 {
            let a = (座標(), 座標());
            let b = (座標(), 座標());
            let c = (座標(), 座標());
            let d = (座標(), 座標());
            assert_eq!(
                線分交差(a, b, c, d),
                交差オラクル(a, b, c, d),
                "a={a:?} b={b:?} c={c:?} d={d:?}"
            );
        }
    }

    #[test]
    fn 直線交点は両直線上にある() {
        let mut seed = 0x11ee_22a0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        let mut 座標 = || (乱数() % 21) as isize - 10;
        for _ in 0..5000 {
            let a = (座標(), 座標());
            let b = (座標(), 座標());
            let c = (座標(), 座標());
            let d = (座標(), 座標());
            if a == b || c == d {
                continue;
            }
            if let Some(p) = 直線交点(a, b, c, d) {
                // 外積3点(a, b, p) ≈ 0 かつ 外積3点(c, d, p) ≈ 0 を浮動小数で確認。
                let f = |o: 点, q: 点, r: (f64, f64)| {
                    (q.0 - o.0) as f64 * (r.1 - o.1 as f64)
                        - (q.1 - o.1) as f64 * (r.0 - o.0 as f64)
                };
                assert!(f(a, b, p).abs() < 1e-6, "ab 上でない {p:?}");
                assert!(f(c, d, p).abs() < 1e-6, "cd 上でない {p:?}");
            }
        }
    }
}
