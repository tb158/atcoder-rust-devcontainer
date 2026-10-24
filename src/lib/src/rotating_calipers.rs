use crate::convex_hull::凸包;
use crate::geometry::{外積3点, 距離2乗};
use cargo_snippet::snippet;

#[snippet("回転_キャリパス", include = "凸包")]
#[snippet("最遠点対", include = "距離_2乗")]
#[snippet("RotatingCalipers")]
#[snippet("凸_多角形の_直径")]
#[snippet("最遠点間_距離")]
/// 点群の最遠点対の距離の 2 乗を回転キャリパスで求める。凸包構築 O(N log N) ＋ 走査 O(N)。
/// 点が 1 個以下なら 0。
fn 最遠点対の距離2乗(点群: &[(isize, isize)]) -> i128 {
    let 包 = 凸包(点群);
    let m = 包.len();
    if m <= 1 {
        return 0;
    }
    if m == 2 {
        return 距離2乗(包[0], 包[1]);
    }
    let mut 最良 = 0i128;
    let mut j = 1usize;
    // i は辺 (i, i+1) を表す添字なので添字ループにする
    #[allow(clippy::needless_range_loop)]
    for i in 0..m {
        let ni = (i + 1) % m;
        // 辺 i→ni から最も遠い頂点 j を進めながら探す
        while 外積3点(包[i], 包[ni], 包[(j + 1) % m]) > 外積3点(包[i], 包[ni], 包[j]) {
            j = (j + 1) % m;
        }
        最良 = 最良.max(距離2乗(包[i], 包[j])).max(距離2乗(包[ni], 包[j]));
    }
    最良
}

#[snippet("凸_多角形の_最小_幅", include = "凸包")]
#[snippet("最小_幅")]
#[snippet("MinimumWidth")]
/// 凸多角形（点群の凸包）の**最小幅**＝平行な 2 直線で挟んだときの最小の間隔を返す。回転キャリパスで
/// 凸包の各辺について最遠頂点までの距離を測り、その最小を取る。凸包構築 O(N log N) ＋ 走査 O(N)。
/// 点が 2 個以下（または一直線）なら幅 0。
///
/// 同じファイルの [`最遠点対の距離2乗`] が「最遠＝直径」なのに対し、こちらは「最薄＝幅」。
/// 最小幅は、棒を通せる最小の隙間や、凸形状を平行平板で挟む最小厚みの判定に使う。
fn 凸多角形の最小幅(点群: &[(isize, isize)]) -> f64 {
    let 包 = 凸包(点群);
    let m = 包.len();
    if m <= 2 {
        return 0.0; // 点・線分は幅 0
    }
    let mut 最小 = f64::INFINITY;
    let mut j = 1usize;
    #[allow(clippy::needless_range_loop)]
    for i in 0..m {
        let ni = (i + 1) % m;
        // 辺 i→ni から最も遠い頂点 j を進めながら探す。
        while 外積3点(包[i], 包[ni], 包[(j + 1) % m]) > 外積3点(包[i], 包[ni], 包[j]) {
            j = (j + 1) % m;
        }
        let 高さ2倍 = 外積3点(包[i], 包[ni], 包[j]).abs() as f64; // = 底辺 × 高さ
        let 辺長 =
            (((包[ni].0 - 包[i].0) as f64).powi(2) + ((包[ni].1 - 包[i].1) as f64).powi(2)).sqrt();
        最小 = 最小.min(高さ2倍 / 辺長);
    }
    最小
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 総当たり(点: &[(isize, isize)]) -> i128 {
        let mut 最良 = 0i128;
        for i in 0..点.len() {
            for j in i + 1..点.len() {
                最良 = 最良.max(距離2乗(点[i], 点[j]));
            }
        }
        最良
    }

    #[test]
    fn 正方形の対角線() {
        let 点 = [(0, 0), (10, 0), (10, 10), (0, 10)];
        // 対角線の長さ^2 = 200
        assert_eq!(最遠点対の距離2乗(&点), 200);
    }

    #[test]
    fn 一点と二点() {
        assert_eq!(最遠点対の距離2乗(&[(3, 4)]), 0);
        assert_eq!(最遠点対の距離2乗(&[(0, 0), (3, 4)]), 25);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x77aa_3311u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 40) as isize % 200 - 100
        };
        for _ in 0..200 {
            let n = (乱数().unsigned_abs() % 30) + 2;
            let 点: Vec<(isize, isize)> = (0..n).map(|_| (乱数(), 乱数())).collect();
            assert_eq!(最遠点対の距離2乗(&点), 総当たり(&点), "点={点:?}");
        }
    }

    // 凸包の各辺について全頂点までの最大距離を測り、その最小を取る素朴オラクル。
    fn 幅総当たり(点: &[(isize, isize)]) -> f64 {
        let 包 = 凸包(点);
        let m = 包.len();
        if m <= 2 {
            return 0.0;
        }
        let mut 最小 = f64::INFINITY;
        for i in 0..m {
            let ni = (i + 1) % m;
            let 辺長 = (((包[ni].0 - 包[i].0) as f64).powi(2)
                + ((包[ni].1 - 包[i].1) as f64).powi(2))
            .sqrt();
            let mut 最大 = 0.0f64;
            for &p in &包 {
                最大 = 最大.max(外積3点(包[i], 包[ni], p).abs() as f64 / 辺長);
            }
            最小 = 最小.min(最大);
        }
        最小
    }

    #[test]
    fn 最小幅の既知例() {
        // 横長長方形 10×4 → 最小幅 4。
        let 長方形 = [(0, 0), (10, 0), (10, 4), (0, 4)];
        assert!((凸多角形の最小幅(&長方形) - 4.0).abs() < 1e-9);
        // 正方形 10×10 → 10。
        let 正方形 = [(0, 0), (10, 0), (10, 10), (0, 10)];
        assert!((凸多角形の最小幅(&正方形) - 10.0).abs() < 1e-9);
        // 3-4-5 直角三角形 → 最小幅 = 2·面積/最長辺 = 12/5 = 2.4。
        let 三角 = [(0, 0), (4, 0), (0, 3)];
        assert!((凸多角形の最小幅(&三角) - 2.4).abs() < 1e-9);
        // 点・線分 → 0。
        assert_eq!(凸多角形の最小幅(&[(1, 1)]), 0.0);
        assert_eq!(凸多角形の最小幅(&[(0, 0), (5, 5)]), 0.0);
    }

    #[test]
    fn 最小幅が総当たりと一致() {
        let mut seed = 0x1c6e_aa90u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 40) as isize % 100 - 50
        };
        for _ in 0..300 {
            let n = (乱数().unsigned_abs() % 30) + 3;
            let 点: Vec<(isize, isize)> = (0..n).map(|_| (乱数(), 乱数())).collect();
            let 速 = 凸多角形の最小幅(&点);
            let 遅 = 幅総当たり(&点);
            assert!((速 - 遅).abs() < 1e-6, "速={速} 遅={遅} 点={点:?}");
        }
    }
}
