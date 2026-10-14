use cargo_snippet::snippet;

#[snippet("ConvexCut")]
#[snippet("PolygonCentroid")]
#[snippet("凸_多角形_切断")]
#[snippet("多角形の_重心")]
#[snippet("多角形_クリップ")]
type 点 = (f64, f64);

#[snippet("多角形の_重心")]
#[snippet("PolygonCentroid")]
/// 単純多角形（頂点を順に並べた列）の**重心**（面積の中心）を返す。面積 0 の退化形は `None`。`O(N)`。
///
/// 頂点の単純平均ではない点に注意。[`重心`](crate::triangle_centers::重心) は三角形 3 点専用。
fn 多角形の重心(多角形: &[点]) -> Option<点> {
    let n = 多角形.len();
    if n < 3 {
        return None;
    }
    let mut 面積2 = 0.0f64;
    let mut cx = 0.0f64;
    let mut cy = 0.0f64;
    for i in 0..n {
        let (x0, y0) = 多角形[i];
        let (x1, y1) = 多角形[(i + 1) % n];
        let cross = x0 * y1 - x1 * y0;
        面積2 += cross;
        cx += (x0 + x1) * cross;
        cy += (y0 + y1) * cross;
    }
    if 面積2.abs() < 1e-12 {
        return None; // 退化（共線など）
    }
    Some((cx / (3.0 * 面積2), cy / (3.0 * 面積2)))
}

#[snippet("凸_多角形_切断")]
#[snippet("多角形_クリップ")]
#[snippet("ConvexCut")]
/// 凸多角形 `多角形`（反時計回り）を有向直線 `a → b` で切り、その**左側**に残る部分を返す。`O(N)`。
///
/// Sutherland–Hodgman の片側クリップ。左側が空なら空の頂点列を返す。
/// [`half_plane_intersection`](crate::half_plane_intersection) は多数の半平面の共通部分を求める別物。
fn 凸多角形切断(多角形: &[点], a: 点, b: 点) -> Vec<点> {
    let n = 多角形.len();
    let 符号 = |p: 点| (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
    let mut 結果 = Vec::with_capacity(n + 1);
    for i in 0..n {
        let c = 多角形[i];
        let d = 多角形[(i + 1) % n];
        let fc = 符号(c);
        let fd = 符号(d);
        if fc >= 0.0 {
            結果.push(c); // 左側・境界上は残す
        }
        if fc * fd < 0.0 {
            // 厳密に反対側 → 交点を追加
            let t = fc / (fc - fd);
            結果.push((c.0 + t * (d.0 - c.0), c.1 + t * (d.1 - c.1)));
        }
    }
    結果
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    // 反時計回り凸多角形に点が含まれるか（境界含む）。
    fn 凸内包(多角形: &[点], q: 点) -> bool {
        let n = 多角形.len();
        (0..n).all(|i| {
            let c = 多角形[i];
            let d = 多角形[(i + 1) % n];
            (d.0 - c.0) * (q.1 - c.1) - (d.1 - c.1) * (q.0 - c.0) >= -1e-9
        })
    }

    // 円周上に等間隔で k 点を取った凸多角形（必ず凸・反時計回り）。
    fn 円多角形(中心: 点, r: f64, k: usize, 回転: f64) -> Vec<点> {
        (0..k)
            .map(|i| {
                let θ = 回転 + 2.0 * PI * i as f64 / k as f64;
                (中心.0 + r * θ.cos(), 中心.1 + r * θ.sin())
            })
            .collect()
    }

    #[test]
    fn 重心の既知例() {
        // 三角形の重心は 3 頂点の平均。
        let g = 多角形の重心(&[(0.0, 0.0), (6.0, 0.0), (0.0, 9.0)]).unwrap();
        assert!((g.0 - 2.0).abs() < 1e-9 && (g.1 - 3.0).abs() < 1e-9);
        // 正方形の重心は中心。
        let g = 多角形の重心(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]).unwrap();
        assert!((g.0 - 2.0).abs() < 1e-9 && (g.1 - 2.0).abs() < 1e-9);
        // L 字（凹多角形）でも面積中心を返す。
        let l = vec![
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ];
        let g = 多角形の重心(&l).unwrap();
        // 2 つの単位正方形（中心 (1,0.5) と (0.5,1.5)）の合成重心。
        assert!((g.0 - 0.8333333).abs() < 1e-5 && (g.1 - 0.8333333).abs() < 1e-5);
        // 退化。
        assert!(多角形の重心(&[(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)]).is_none());
    }

    #[test]
    fn 重心がモンテカルロと一致() {
        let mut seed = 0x3aa1_27c0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 11
        };
        let mut 一様 = || 乱数() as f64 / (1u64 << 53) as f64;
        for _ in 0..30 {
            let 中心 = (一様() * 4.0 - 2.0, 一様() * 4.0 - 2.0);
            let r = 一様() * 3.0 + 1.0;
            let k = ((一様() * 6.0) as usize).min(5) + 3;
            let 多 = 円多角形(中心, r, k, 一様() * PI);
            let 公式 = 多角形の重心(&多).unwrap();
            // 外接矩形で棄却サンプリングして重心を推定。
            let m = 300_000;
            let (mut sx, mut sy, mut cnt) = (0.0f64, 0.0f64, 0u64);
            for _ in 0..m {
                let x = 中心.0 + (一様() * 2.0 - 1.0) * r;
                let y = 中心.1 + (一様() * 2.0 - 1.0) * r;
                if 凸内包(&多, (x, y)) {
                    sx += x;
                    sy += y;
                    cnt += 1;
                }
            }
            let 推定 = (sx / cnt as f64, sy / cnt as f64);
            assert!(
                (公式.0 - 推定.0).abs() < 0.05 && (公式.1 - 推定.1).abs() < 0.05,
                "公式={公式:?} 推定={推定:?}"
            );
        }
    }

    #[test]
    fn 切断の既知例() {
        // 単位正方形を直線 x=0.5（下向き a→b で左側が x<=0.5）で切る。
        let 正方形 = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        // a=(0.5,0) → b=(0.5,1) の左側は x <= 0.5。
        let 左 = 凸多角形切断(&正方形, (0.5, 0.0), (0.5, 1.0));
        // 左側はすべて x <= 0.5。
        assert!(左.iter().all(|p| p.0 <= 0.5 + 1e-9));
        // 面積は元の半分（重心の x で確認）。
        let g = 多角形の重心(&左).unwrap();
        assert!((g.0 - 0.25).abs() < 1e-9, "g={g:?}");
    }

    #[test]
    fn 切断が内包と整合() {
        let mut seed = 0x5fc8_91a0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 11
        };
        let mut 一様 = || 乱数() as f64 / (1u64 << 53) as f64;
        for _ in 0..400 {
            let k = ((一様() * 6.0) as usize).min(5) + 3;
            let 多 = 円多角形((0.0, 0.0), 5.0, k, 一様() * PI);
            let a = (一様() * 10.0 - 5.0, 一様() * 10.0 - 5.0);
            let b = (一様() * 10.0 - 5.0, 一様() * 10.0 - 5.0);
            if (a.0 - b.0).hypot(a.1 - b.1) < 1e-6 {
                continue;
            }
            let 切 = 凸多角形切断(&多, a, b);
            if 切.len() < 3 {
                continue; // 左側が空・線分のみのときは内包判定の対象外
            }
            // 点 q が切断結果に入る ⇔ 元多角形に入り、かつ直線の左側。
            // 境界付近は浮動小数で揺れるため、十分内側／外側の点だけ厳密一致を要求する。
            let 符号 = |p: 点| (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
            let 直線長 = (a.0 - b.0).hypot(a.1 - b.1);
            for _ in 0..50 {
                let q = (一様() * 12.0 - 6.0, 一様() * 12.0 - 6.0);
                // 元多角形の各辺から十分離れているか。
                let 多余裕 = (0..多.len()).all(|i| {
                    let c = 多[i];
                    let d = 多[(i + 1) % 多.len()];
                    ((d.0 - c.0) * (q.1 - c.1) - (d.1 - c.1) * (q.0 - c.0)).abs()
                        > 1e-2 * (d.0 - c.0).hypot(d.1 - c.1)
                });
                // 切断直線からも十分離れているか。
                if !多余裕 || 符号(q).abs() <= 1e-2 * 直線長 {
                    continue;
                }
                let 期待 = 凸内包(&多, q) && 符号(q) >= 0.0;
                let 実 = 凸内包(&切, q);
                assert_eq!(実, 期待, "q={q:?} a={a:?} b={b:?} 多={多:?}");
            }
        }
    }
}
