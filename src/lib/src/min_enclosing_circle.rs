use cargo_snippet::snippet;

#[snippet("最小_包含円")]
#[snippet("最小_内包円")]
#[snippet("Welzl")]
#[snippet("MinEnclosingCircle")]
#[snippet("最小_被覆円")]
/// 2 点間の距離の 2 乗（浮動小数）。整数座標版は `geometry` の `距離2乗`。
fn 距離2乗f64(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx * dx + dy * dy
}

#[snippet("最小_包含円")]
#[snippet("最小_内包円")]
#[snippet("Welzl")]
#[snippet("MinEnclosingCircle")]
#[snippet("最小_被覆円")]
/// 2 点の中点。
fn 中点(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0)
}

#[snippet("最小_包含円")]
#[snippet("最小_内包円")]
#[snippet("Welzl")]
#[snippet("MinEnclosingCircle")]
#[snippet("最小_被覆円")]
/// 3 点を通る円の中心（外心）。3 点が一直線上だと数値的に破綻するので注意。
fn 外心(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> (f64, f64) {
    let (ax, ay) = a;
    let (bx, by) = b;
    let (cx, cy) = c;
    let d = 2.0 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));
    let a2 = ax * ax + ay * ay;
    let b2 = bx * bx + by * by;
    let c2 = cx * cx + cy * cy;
    let ux = (a2 * (by - cy) + b2 * (cy - ay) + c2 * (ay - by)) / d;
    let uy = (a2 * (cx - bx) + b2 * (ax - cx) + c2 * (bx - ax)) / d;
    (ux, uy)
}

#[snippet("最小_包含円")]
#[snippet("最小_内包円")]
#[snippet("Welzl")]
#[snippet("MinEnclosingCircle")]
#[snippet("最小_被覆円")]
/// すべての点を含む最小の円 `(中心x, 中心y, 半径)` を求める（Welzl の漸増法）。期待 O(N)。
/// 点が空なら `None`。
fn 最小包含円(点群: &[(f64, f64)]) -> Option<(f64, f64, f64)> {
    const イプシロン: f64 = 1e-12;
    if 点群.is_empty() {
        return None;
    }
    let mut p = 点群.to_vec();
    // 線形合同法で決定的にシャッフル
    let mut seed = 0x1234_5678u64;
    let mut 乱数 = || {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        seed >> 33
    };
    for i in (1..p.len()).rev() {
        let j = (乱数() as usize) % (i + 1);
        p.swap(i, j);
    }
    let mut 中心 = p[0];
    let mut 半径2 = 0.0f64;
    // i, j, k は「先頭 i/j/k 点まで」を表す境界として使うため添字ループにする
    #[allow(clippy::needless_range_loop)]
    for i in 1..p.len() {
        if 距離2乗f64(中心, p[i]) <= 半径2 + イプシロン {
            continue;
        }
        中心 = p[i];
        半径2 = 0.0;
        for j in 0..i {
            if 距離2乗f64(中心, p[j]) <= 半径2 + イプシロン {
                continue;
            }
            中心 = 中点(p[i], p[j]);
            半径2 = 距離2乗f64(中心, p[i]);
            for k in 0..j {
                if 距離2乗f64(中心, p[k]) <= 半径2 + イプシロン {
                    continue;
                }
                中心 = 外心(p[i], p[j], p[k]);
                半径2 = 距離2乗f64(中心, p[i]);
            }
        }
    }
    Some((中心.0, 中心.1, 半径2.sqrt()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const イプシロン: f64 = 1e-7;

    // 総当たり：全ペアを直径とする円・全三つ組の外接円のうち、全点を含む最小半径
    #[allow(clippy::needless_range_loop)]
    fn 総当たり半径(点: &[(f64, f64)]) -> f64 {
        let n = 点.len();
        if n == 1 {
            return 0.0;
        }
        let mut 最良 = f64::INFINITY;
        // 相対許容誤差（座標が大きいと r2 の丸め誤差も大きくなるため）
        let 全部含む = |中心: (f64, f64), r2: f64| {
            点.iter()
                .all(|&q| 距離2乗f64(中心, q) <= r2 + 1e-6 * (r2 + 1.0))
        };
        for i in 0..n {
            for j in i + 1..n {
                let c = 中点(点[i], 点[j]);
                let r2 = 距離2乗f64(c, 点[i]);
                if 全部含む(c, r2) {
                    最良 = 最良.min(r2.sqrt());
                }
            }
        }
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    // 一直線上は外心が定義できないので除外
                    let (ax, ay) = 点[i];
                    let (bx, by) = 点[j];
                    let (cx, cy) = 点[k];
                    let d = (bx - ax) * (cy - ay) - (by - ay) * (cx - ax);
                    if d.abs() < 1e-9 {
                        continue;
                    }
                    let c = 外心(点[i], 点[j], 点[k]);
                    let r2 = 距離2乗f64(c, 点[i]);
                    if 全部含む(c, r2) {
                        最良 = 最良.min(r2.sqrt());
                    }
                }
            }
        }
        最良
    }

    #[test]
    fn 二点は直径の半分() {
        let r = 最小包含円(&[(0.0, 0.0), (2.0, 0.0)]).unwrap();
        assert!((r.0 - 1.0).abs() < イプシロン);
        assert!((r.1 - 0.0).abs() < イプシロン);
        assert!((r.2 - 1.0).abs() < イプシロン);
    }

    #[test]
    fn 正方形の四隅() {
        let r = 最小包含円(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]).unwrap();
        assert!((r.0 - 1.0).abs() < イプシロン);
        assert!((r.1 - 1.0).abs() < イプシロン);
        assert!((r.2 - 2.0f64.sqrt()).abs() < イプシロン);
    }

    #[test]
    fn 空は結果なし() {
        assert!(最小包含円(&[]).is_none());
    }

    #[test]
    fn 乱数で総当たりと半径一致() {
        let mut seed = 0x9e37u64;
        let mut 生 = move || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed
        };
        // 座標は [0, 100) に収め、丸め誤差を抑える
        let mut 乱数 = move || (生() % 100_000) as f64 / 1000.0;
        for _ in 0..60 {
            let n = (乱数() * 1000.0) as usize % 7 + 1;
            let 点: Vec<(f64, f64)> = (0..n).map(|_| (乱数(), 乱数())).collect();
            let (cx, cy, r) = 最小包含円(&点).unwrap();
            // 全点を含む
            for &(x, y) in &点 {
                let d = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
                assert!(d <= r + 1e-6, "点 ({x},{y}) が円外 d={d} r={r}");
            }
            // 半径が総当たりと一致
            let 期待 = 総当たり半径(&点);
            assert!((r - 期待).abs() < 1e-4, "n={n} r={r} 期待={期待}");
        }
    }
}
