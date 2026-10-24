use crate::geometry::距離2乗;
use cargo_snippet::snippet;

#[snippet("最近点対", include = "距離_2乗")]
#[snippet("ClosestPair")]
#[snippet("最近点")]
#[snippet("最も_近い_2点")]
/// 平面上の点群から最も近い 2 点を分割統治で求める。O(N log^2 N)。
/// 返り値は `(最小距離の 2 乗, (点 i の添字, 点 j の添字))`。点が 2 個未満なら `None`。
///
/// 距離は整数の 2 乗で厳密に比較する（誤差なし）。
fn 最近点対(点群: &[(isize, isize)]) -> Option<(i128, (usize, usize))> {
    if 点群.len() < 2 {
        return None;
    }
    let mut idx: Vec<usize> = (0..点群.len()).collect();
    idx.sort_by_key(|&i| (点群[i].0, 点群[i].1));
    let n = idx.len();
    Some(再帰(点群, &idx, 0, n))
}

#[snippet("最近点対")]
#[snippet("ClosestPair")]
#[snippet("最近点")]
#[snippet("最も_近い_2点")]
/// `idx[l..r]`（x 昇順）の範囲で最近点対を求める。
fn 再帰(点群: &[(isize, isize)], idx: &[usize], l: usize, r: usize) -> (i128, (usize, usize)) {
    if r - l <= 3 {
        let mut 最良 = (i128::MAX, (0usize, 0usize));
        for a in l..r {
            for b in a + 1..r {
                let d = 距離2乗(点群[idx[a]], 点群[idx[b]]);
                if d < 最良.0 {
                    最良 = (d, (idx[a], idx[b]));
                }
            }
        }
        return 最良;
    }
    let 中 = (l + r) / 2;
    let 中x = 点群[idx[中]].0;
    let 左 = 再帰(点群, idx, l, 中);
    let 右 = 再帰(点群, idx, 中, r);
    let mut 最良 = if 左.0 <= 右.0 { 左 } else { 右 };
    // 分割線から距離（の 2 乗）が最良未満の点だけを y 順に並べて突き合わせる。
    let mut 帯: Vec<usize> = idx[l..r]
        .iter()
        .copied()
        .filter(|&p| {
            let dx = (点群[p].0 - 中x) as i128;
            dx * dx < 最良.0
        })
        .collect();
    帯.sort_by_key(|&p| 点群[p].1);
    for (i, &pi) in 帯.iter().enumerate() {
        for &pj in &帯[i + 1..] {
            let dy = (点群[pj].1 - 点群[pi].1) as i128;
            if dy * dy >= 最良.0 {
                break;
            }
            let d = 距離2乗(点群[pi], 点群[pj]);
            if d < 最良.0 {
                最良 = (d, (pi, pj));
            }
        }
    }
    最良
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 総当たり(点群: &[(isize, isize)]) -> i128 {
        let mut 最良 = i128::MAX;
        for i in 0..点群.len() {
            for j in i + 1..点群.len() {
                最良 = 最良.min(距離2乗(点群[i], 点群[j]));
            }
        }
        最良
    }

    #[test]
    fn 小さな例() {
        let p = vec![(0, 0), (3, 4), (1, 1), (10, 10)];
        let (d, (i, j)) = 最近点対(&p).unwrap();
        assert_eq!(d, 2); // (0,0)-(1,1)
        assert_eq!(距離2乗(p[i], p[j]), 2);
    }

    #[test]
    fn 点が1個以下はnone() {
        assert!(最近点対(&[]).is_none());
        assert!(最近点対(&[(5, 5)]).is_none());
    }

    #[test]
    fn 同一点があると0() {
        let p = vec![(2, 3), (7, 1), (2, 3), (0, 0)];
        assert_eq!(最近点対(&p).unwrap().0, 0);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0xdead_beefu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..200 {
            let n = (乱数() as usize % 30) + 2;
            let p: Vec<(isize, isize)> = (0..n)
                .map(|_| ((乱数() % 50) as isize, (乱数() % 50) as isize))
                .collect();
            let (d, (i, j)) = 最近点対(&p).unwrap();
            assert_eq!(d, 総当たり(&p), "p={p:?}");
            assert_eq!(距離2乗(p[i], p[j]), d);
            assert_ne!(i, j);
        }
    }
}
