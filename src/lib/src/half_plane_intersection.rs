use cargo_snippet::snippet;
#[snippet("HalfPlaneIntersection")]
#[snippet("半平面")]
#[snippet("半平面の_共通部分")]
#[snippet("半平面_交差")]
use std::collections::VecDeque;

#[snippet("半平面_交差")]
#[snippet("HalfPlaneIntersection")]
#[snippet("半平面の_共通部分")]
#[snippet("半平面")]
const イプシロン: f64 = 1e-9;

#[snippet("半平面_交差")]
#[snippet("HalfPlaneIntersection")]
#[snippet("半平面の_共通部分")]
#[snippet("半平面")]
/// 1 つの半平面。境界直線を「点 `p` から方向 `pq` へ向かう有向直線」で表し、その**左側**（`pq` を反時計回りに
/// 見た側）を実行可能領域とする。`角度` は `pq` の偏角で、交差計算のソートキーに使う。
#[derive(Clone, Copy)]
struct 半平面 {
    p: (f64, f64),
    pq: (f64, f64),
    角度: f64,
}

#[snippet("半平面_交差")]
#[snippet("HalfPlaneIntersection")]
#[snippet("半平面の_共通部分")]
#[snippet("半平面")]
impl 半平面 {
    fn cross(a: (f64, f64), b: (f64, f64)) -> f64 {
        a.0 * b.1 - a.1 * b.0
    }

    fn dot(a: (f64, f64), b: (f64, f64)) -> f64 {
        a.0 * b.0 + a.1 * b.1
    }

    /// 有向辺 `a → b` の**左側**を実行可能領域とする半平面。
    fn 二点(a: (f64, f64), b: (f64, f64)) -> Self {
        let pq = (b.0 - a.0, b.1 - a.1);
        Self {
            p: a,
            pq,
            角度: pq.1.atan2(pq.0),
        }
    }

    /// `a·x + b·y + c <= 0` を満たす側を実行可能領域とする半平面。`(a, b) != (0, 0)`。
    fn 直線係数(a: f64, b: f64, c: f64) -> Self {
        let p = if a.abs() > b.abs() {
            (-c / a, 0.0)
        } else {
            (0.0, -c / b)
        };
        let pq = (-b, a);
        Self {
            p,
            pq,
            角度: pq.1.atan2(pq.0),
        }
    }

    /// 境界直線同士の交点。`self` と `other` が平行でないことを前提とする。
    fn 交点(&self, other: &Self) -> (f64, f64) {
        let alpha = Self::cross((other.p.0 - self.p.0, other.p.1 - self.p.1), other.pq)
            / Self::cross(self.pq, other.pq);
        (self.p.0 + self.pq.0 * alpha, self.p.1 + self.pq.1 * alpha)
    }

    /// 点 `r` が実行可能領域の外（境界より右）にあるか。
    fn 外(&self, r: (f64, f64)) -> bool {
        Self::cross(self.pq, (r.0 - self.p.0, r.1 - self.p.1)) < -イプシロン
    }
}

#[snippet("半平面_交差")]
#[snippet("HalfPlaneIntersection")]
#[snippet("半平面の_共通部分")]
#[snippet("半平面")]
/// 半平面群の共通部分（凸多角形）を O(N log N) で求める。返り値は反時計回りの頂点列。
///
/// 共通部分が空なら `None`。共通部分が非有界になりうるので、内部で一辺 `2·範囲` の境界箱を補い、結果は
/// `[-範囲, 範囲]^2` にクリップされる（`範囲` は全入力を含む十分大きな値にすること）。
fn 半平面交差(半平面群: &[半平面], 範囲: f64) -> Option<Vec<(f64, f64)>> {
    let mut h = 半平面群.to_vec();
    let 箱 = [(範囲, 範囲), (-範囲, 範囲), (-範囲, -範囲), (範囲, -範囲)];
    for i in 0..4 {
        h.push(半平面::二点(箱[i], 箱[(i + 1) % 4]));
    }
    h.sort_by(|a, b| a.角度.partial_cmp(&b.角度).unwrap());
    let mut dq: VecDeque<半平面> = VecDeque::new();
    for hp in h {
        while dq.len() > 1 && hp.外(dq[dq.len() - 1].交点(&dq[dq.len() - 2])) {
            dq.pop_back();
        }
        while dq.len() > 1 && hp.外(dq[0].交点(&dq[1])) {
            dq.pop_front();
        }
        // 直前の半平面と平行なときは、より制約の強い方だけ残す。
        if let Some(後) = dq.back()
            && 半平面::cross(hp.pq, 後.pq).abs() < イプシロン
        {
            if 半平面::dot(hp.pq, 後.pq) < 0.0 {
                return None; // 逆向き平行 → 空
            }
            if hp.外(後.p) {
                dq.pop_back();
            } else {
                continue;
            }
        }
        dq.push_back(hp);
    }
    while dq.len() > 2 && dq[0].外(dq[dq.len() - 1].交点(&dq[dq.len() - 2])) {
        dq.pop_back();
    }
    while dq.len() > 2 && dq[dq.len() - 1].外(dq[0].交点(&dq[1])) {
        dq.pop_front();
    }
    if dq.len() < 3 {
        return None;
    }
    let n = dq.len();
    let mut 頂点 = Vec::with_capacity(n);
    for i in 0..n {
        頂点.push(dq[i].交点(&dq[(i + 1) % n]));
    }
    Some(頂点)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 面積(多角形: &[(f64, f64)]) -> f64 {
        let n = 多角形.len();
        let mut s = 0.0;
        for i in 0..n {
            let (x1, y1) = 多角形[i];
            let (x2, y2) = 多角形[(i + 1) % n];
            s += x1 * y2 - x2 * y1;
        }
        s.abs() / 2.0
    }

    #[test]
    fn 単位正方形() {
        let hs = vec![
            半平面::直線係数(-1.0, 0.0, 0.0), // x >= 0
            半平面::直線係数(1.0, 0.0, -1.0), // x <= 1
            半平面::直線係数(0.0, -1.0, 0.0), // y >= 0
            半平面::直線係数(0.0, 1.0, -1.0), // y <= 1
        ];
        let poly = 半平面交差(&hs, 1e4).unwrap();
        assert!((面積(&poly) - 1.0).abs() < 1e-6, "面積={}", 面積(&poly));
    }

    #[test]
    fn 空になる場合() {
        let hs = vec![
            半平面::直線係数(1.0, 0.0, 0.0),   // x <= 0
            半平面::直線係数(0.0, 1.0, 0.0),   // y <= 0
            半平面::直線係数(-1.0, -1.0, 1.0), // x + y >= 1
        ];
        assert!(半平面交差(&hs, 1e4).is_none());
    }

    #[test]
    fn 凸多角形を辺から復元() {
        let mut seed = 0xc0ffee_u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..100 {
            let k = (乱数() as usize % 6) + 3;
            let mut 角度: Vec<f64> = (0..k)
                .map(|_| (乱数() % 100_000) as f64 / 100_000.0 * std::f64::consts::TAU)
                .collect();
            角度.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mut 頂点: Vec<(f64, f64)> = Vec::new();
            for (i, &a) in 角度.iter().enumerate() {
                if i > 0 && (a - 角度[i - 1]) < 0.2 {
                    continue;
                }
                頂点.push((50.0 * a.cos(), 50.0 * a.sin()));
            }
            if 頂点.len() < 3 {
                continue;
            }
            let m = 頂点.len();
            let hs: Vec<半平面> = (0..m)
                .map(|i| 半平面::二点(頂点[i], 頂点[(i + 1) % m]))
                .collect();
            let 復元 = 半平面交差(&hs, 1e4).unwrap();
            assert!(
                (面積(&復元) - 面積(&頂点)).abs() < 1e-3 * 面積(&頂点).max(1.0),
                "元={} 復元={}",
                面積(&頂点),
                面積(&復元)
            );
        }
    }
}
