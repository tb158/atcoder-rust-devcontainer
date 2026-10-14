use cargo_snippet::snippet;

#[snippet("CircleArea")]
#[snippet("CirclePolygonArea")]
#[snippet("CommonTangents")]
#[snippet("レンズ_面積")]
#[snippet("二円の_交差_面積")]
#[snippet("共通_接線_本数")]
#[snippet("円と_多角形の_交差_面積")]
#[snippet("円の_共通_接線")]
#[snippet("円_多角形_面積")]
type 点 = (f64, f64);

#[snippet("二円の_交差_面積")]
#[snippet("レンズ_面積")]
#[snippet("CircleArea")]
/// 2 つの円（中心 `c1` 半径 `r1`、中心 `c2` 半径 `r2`）が重なる部分の面積を返す。`O(1)`。
///
/// 離れていれば `0`、一方が他方を含めば小さい円の面積。
/// [`円と円の交点`](crate::circle_intersection::円と円の交点) は交点座標で、こちらは面積。
fn 二円の交差面積(c1: 点, r1: f64, c2: 点, r2: f64) -> f64 {
    let d = (c1.0 - c2.0).hypot(c1.1 - c2.1);
    if d >= r1 + r2 {
        return 0.0; // 交わらない
    }
    if d <= (r1 - r2).abs() {
        let r = r1.min(r2);
        return std::f64::consts::PI * r * r; // 一方が他方を内包
    }
    let a1 = ((d * d + r1 * r1 - r2 * r2) / (2.0 * d * r1))
        .clamp(-1.0, 1.0)
        .acos();
    let a2 = ((d * d + r2 * r2 - r1 * r1) / (2.0 * d * r2))
        .clamp(-1.0, 1.0)
        .acos();
    let 三角 = 0.5
        * ((-d + r1 + r2) * (d + r1 - r2) * (d - r1 + r2) * (d + r1 + r2))
            .max(0.0)
            .sqrt();
    r1 * r1 * a1 + r2 * r2 * a2 - 三角
}

#[snippet("共通_接線_本数")]
#[snippet("円の_共通_接線")]
#[snippet("CommonTangents")]
/// 2 つの円の共通接線の本数を返す。同心で半径が等しい場合のみ無数（`!0`）。`O(1)`。
///
/// 中心間距離 `d` と半径から分類する：`d>r1+r2`→4、`=r1+r2`→3、`|r1-r2|<d<r1+r2`→2、
/// `d=|r1-r2|`→1、`d<|r1-r2|`→0（一方が他方の内部）。半径 0（点）も同じ枠組みで扱える。
fn 共通接線本数(c1: 点, r1: f64, c2: 点, r2: f64) -> usize {
    const EPS: f64 = 1e-9;
    let d = (c1.0 - c2.0).hypot(c1.1 - c2.1);
    let 和 = r1 + r2;
    let 差 = (r1 - r2).abs();
    if d < EPS && 差 < EPS {
        return !0; // 完全に一致する円（接線無数）
    }
    if d > 和 + EPS {
        4
    } else if (d - 和).abs() <= EPS {
        3
    } else if d > 差 + EPS {
        2
    } else if (d - 差).abs() <= EPS {
        1
    } else {
        0
    }
}

#[snippet("円と_多角形の_交差_面積")]
#[snippet("円_多角形_面積")]
#[snippet("CirclePolygonArea")]
/// 円（中心 `中心`・半径 `r`）と単純多角形（頂点を順に並べた列）が重なる部分の面積を返す。O(N)。
///
/// 頂点列の向きは問わず、凸でなくてもよい。
fn 円と多角形の交差面積(中心: 点, r: f64, 多角形: &[点]) -> f64 {
    let n = 多角形.len();
    if n < 3 || r <= 0.0 {
        return 0.0;
    }
    let 内積 = |p: 点, q: 点| p.0 * q.0 + p.1 * q.1;
    let 外積 = |p: 点, q: 点| p.0 * q.1 - p.1 * q.0;
    // 中心から見た 2 ベクトル間の符号付き角度。
    let 偏角 = |p: 点, q: 点| 外積(p, q).atan2(内積(p, q));
    let r2 = r * r / 2.0;
    // 三角形 (原点, p, q)（p,q は中心を原点に平行移動済み）と円の交差面積（符号付き）。
    let 三角 = |p: 点, q: 点| -> f64 {
        let d = (q.0 - p.0, q.1 - p.1);
        let d2 = 内積(d, d);
        if d2 < 1e-18 {
            return 0.0; // 退化辺
        }
        let a = 内積(d, p) / d2;
        let b = (内積(p, p) - r * r) / d2;
        let det = a * a - b;
        if det <= 0.0 {
            return 偏角(p, q) * r2; // 辺は円の外側 → 弧の扇形だけ
        }
        let sq = det.sqrt();
        let s = (-a - sq).max(0.0);
        let t = (-a + sq).min(1.0);
        if t < 0.0 || 1.0 <= s {
            return 偏角(p, q) * r2; // 交点が辺の外
        }
        let u = (p.0 + d.0 * s, p.1 + d.1 * s);
        let v = (p.0 + d.0 * t, p.1 + d.1 * t);
        偏角(p, u) * r2 + 外積(u, v) / 2.0 + 偏角(v, q) * r2
    };
    let mut 和 = 0.0f64;
    for i in 0..n {
        let p = (多角形[i].0 - 中心.0, 多角形[i].1 - 中心.1);
        let q = (
            多角形[(i + 1) % n].0 - 中心.0,
            多角形[(i + 1) % n].1 - 中心.1,
        );
        和 += 三角(p, q);
    }
    和.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    // 円周上に等間隔で k 点を取った凸多角形（必ず凸・反時計回り）。
    fn 円多角形(中心: 点, r: f64, k: usize, 回転: f64) -> Vec<点> {
        (0..k)
            .map(|i| {
                let t = 回転 + 2.0 * PI * i as f64 / k as f64;
                (中心.0 + r * t.cos(), 中心.1 + r * t.sin())
            })
            .collect()
    }

    #[test]
    fn 交差面積の既知例() {
        // 半径 1 の同心円 → 面積 π。
        assert!((二円の交差面積((0.0, 0.0), 1.0, (0.0, 0.0), 1.0) - PI).abs() < 1e-9);
        // 離れている → 0。
        assert!(二円の交差面積((0.0, 0.0), 1.0, (5.0, 0.0), 1.0).abs() < 1e-12);
        // 半径 1 の円が中心間距離 1 で交差（よく知られた値）。
        // 面積 = 2·acos(1/2) − (√3)/2 = 2π/3 − √3/2。
        let 期待 = 2.0 * (0.5f64).acos() - 3.0f64.sqrt() / 2.0;
        let 実 = 二円の交差面積((0.0, 0.0), 1.0, (1.0, 0.0), 1.0);
        assert!((実 - 期待).abs() < 1e-9, "実={実} 期待={期待}");
        // 内包: 大円の中に小円 → 小円の面積。
        assert!((二円の交差面積((0.0, 0.0), 3.0, (0.5, 0.0), 1.0) - PI).abs() < 1e-9);
    }

    #[test]
    fn 交差面積がモンテカルロと一致() {
        let mut seed = 0xa5c1_3e80u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 11 // 53bit
        };
        let mut 一様 = || 乱数() as f64 / (1u64 << 53) as f64; // [0,1)
        for _ in 0..40 {
            let c1 = (一様() * 6.0 - 3.0, 一様() * 6.0 - 3.0);
            let c2 = (一様() * 6.0 - 3.0, 一様() * 6.0 - 3.0);
            let r1 = 一様() * 2.5 + 0.5;
            let r2 = 一様() * 2.5 + 0.5;
            let 公式 = 二円の交差面積(c1, r1, c2, r2);
            // 両円を覆う矩形でモンテカルロ。
            let xlo = (c1.0 - r1).min(c2.0 - r2);
            let xhi = (c1.0 + r1).max(c2.0 + r2);
            let ylo = (c1.1 - r1).min(c2.1 - r2);
            let yhi = (c1.1 + r1).max(c2.1 + r2);
            let 箱 = (xhi - xlo) * (yhi - ylo);
            let m = 400_000;
            let mut 中 = 0u64;
            for _ in 0..m {
                let x = xlo + 一様() * (xhi - xlo);
                let y = ylo + 一様() * (yhi - ylo);
                let in1 = (x - c1.0).hypot(y - c1.1) <= r1;
                let in2 = (x - c2.0).hypot(y - c2.1) <= r2;
                if in1 && in2 {
                    中 += 1;
                }
            }
            let 推定 = 中 as f64 / m as f64 * 箱;
            assert!(
                (公式 - 推定).abs() < 0.1,
                "公式={公式} 推定={推定} c1={c1:?} r1={r1} c2={c2:?} r2={r2}"
            );
        }
    }

    #[test]
    fn 共通接線本数の既知例() {
        assert_eq!(共通接線本数((0.0, 0.0), 1.0, (5.0, 0.0), 1.0), 4); // 離れている
        assert_eq!(共通接線本数((0.0, 0.0), 1.0, (2.0, 0.0), 1.0), 3); // 外接
        assert_eq!(共通接線本数((0.0, 0.0), 2.0, (2.0, 0.0), 1.0), 2); // 2 点で交差
        assert_eq!(共通接線本数((0.0, 0.0), 3.0, (2.0, 0.0), 1.0), 1); // 内接
        assert_eq!(共通接線本数((0.0, 0.0), 5.0, (1.0, 0.0), 1.0), 0); // 内包
        assert_eq!(共通接線本数((0.0, 0.0), 1.0, (0.0, 0.0), 1.0), usize::MAX); // 一致
    }

    #[test]
    fn 円多角形面積の既知例() {
        // 円が多角形を完全に内包 → 円の面積 πr²。
        let 大四角 = vec![(-10.0, -10.0), (10.0, -10.0), (10.0, 10.0), (-10.0, 10.0)];
        assert!((円と多角形の交差面積((0.0, 0.0), 2.0, &大四角) - PI * 4.0).abs() < 1e-6);
        // 多角形が円を完全に内包 → 多角形の面積。
        let 小四角 = vec![(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
        assert!((円と多角形の交差面積((0.0, 0.0), 100.0, &小四角) - 4.0).abs() < 1e-6);
        // 離れている → 0。
        let 遠四角 = vec![(50.0, 50.0), (51.0, 50.0), (51.0, 51.0), (50.0, 51.0)];
        assert!(円と多角形の交差面積((0.0, 0.0), 1.0, &遠四角).abs() < 1e-9);
        // 半円（x>=0 側に大きな矩形）。
        let 半平面 = vec![(0.0, -10.0), (10.0, -10.0), (10.0, 10.0), (0.0, 10.0)];
        assert!((円と多角形の交差面積((0.0, 0.0), 3.0, &半平面) - PI * 9.0 / 2.0).abs() < 1e-6);
        // 時計回りでも同じ（絶対値）。
        let 半平面逆: Vec<点> = 半平面.iter().rev().cloned().collect();
        assert!((円と多角形の交差面積((0.0, 0.0), 3.0, &半平面逆) - PI * 9.0 / 2.0).abs() < 1e-6);
    }

    #[test]
    fn 円多角形面積がモンテカルロと一致() {
        let mut seed = 0x6c4f_a1d0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 11
        };
        let mut 一様 = || 乱数() as f64 / (1u64 << 53) as f64;
        for _ in 0..40 {
            let pc = (一様() * 6.0 - 3.0, 一様() * 6.0 - 3.0);
            let pr = 一様() * 2.5 + 0.5;
            let k = ((一様() * 6.0) as usize).min(5) + 3;
            let 多 = 円多角形(pc, pr, k, 一様() * PI);
            let c = (一様() * 6.0 - 3.0, 一様() * 6.0 - 3.0);
            let r = 一様() * 2.5 + 0.5;
            let 公式 = 円と多角形の交差面積(c, r, &多);
            let xlo = (c.0 - r).min(pc.0 - pr);
            let xhi = (c.0 + r).max(pc.0 + pr);
            let ylo = (c.1 - r).min(pc.1 - pr);
            let yhi = (c.1 + r).max(pc.1 + pr);
            let 箱 = (xhi - xlo) * (yhi - ylo);
            let 内包 = |q: 点| {
                (0..多.len()).all(|i| {
                    let a = 多[i];
                    let b = 多[(i + 1) % 多.len()];
                    (b.0 - a.0) * (q.1 - a.1) - (b.1 - a.1) * (q.0 - a.0) >= -1e-9
                })
            };
            let m = 400_000;
            let mut 中 = 0u64;
            for _ in 0..m {
                let x = xlo + 一様() * (xhi - xlo);
                let y = ylo + 一様() * (yhi - ylo);
                if (x - c.0).hypot(y - c.1) <= r && 内包((x, y)) {
                    中 += 1;
                }
            }
            let 推定 = 中 as f64 / m as f64 * 箱;
            assert!(
                (公式 - 推定).abs() < 0.1,
                "公式={公式} 推定={推定} c={c:?} r={r} pc={pc:?} pr={pr}"
            );
        }
    }
}
