use crate::to_int::to_isize;
use cargo_snippet::snippet;

#[snippet("KD_木")]
#[snippet("kd_木")]
#[snippet("KDTree")]
/// 平面上の静的な点集合を、軸を交互に切り替えながら中央値で分割して作る平衡 **2 次元 kd 木**。
/// 構築 `O(N log N)`、最近傍クエリ平均 `O(log N)`、矩形範囲計数は最悪 `O(√N)`。
///
/// [`closest_pair`](crate::closest_pair) は点集合全体の**最近点対 1 組**を `O(N log N)` で求める別物で、
/// 任意のクエリ点に対する最近傍探索や矩形内点数は扱わない。[`wavelet_matrix`](crate::wavelet_matrix) は
/// 1 次元列の区間クエリ用。
struct KD木 {
    点: Vec<(isize, isize, usize)>,
}

#[snippet("KD_木")]
#[snippet("kd_木")]
#[snippet("KDTree")]
/// 区間 `点[..]` を `axis`（0=x, 1=y）の中央値で再帰的に分割し、陰的平衡 kd 木を組む内部関数。
fn 構築(点: &mut [(isize, isize, usize)], axis: usize) {
    let n = 点.len();
    if n <= 1 {
        return;
    }
    let m = n / 2;
    if axis == 0 {
        点.select_nth_unstable_by_key(m, |&(x, _, _)| x);
    } else {
        点.select_nth_unstable_by_key(m, |&(_, y, _)| y);
    }
    let (左, 右) = 点.split_at_mut(m);
    構築(左, 1 - axis);
    構築(&mut 右[1..], 1 - axis);
}

#[snippet("KD_木", include = "to_isize")]
#[snippet("kd_木")]
#[snippet("KDTree")]
impl KD木 {
    /// 点群から kd 木を構築する。`O(N log N)`。
    fn new(点群: &[(isize, isize)]) -> Self {
        let mut 点: Vec<(isize, isize, usize)> = 点群
            .iter()
            .enumerate()
            .map(|(i, &(x, y))| (x, y, i))
            .collect();
        構築(&mut 点, 0);
        KD木 { 点 }
    }

    /// クエリ点 `q` に最も近い点の `(距離の2乗, 元の添字)` を返す。点が無ければ `None`。
    /// 距離が等しい点が複数あるときどの添字を返すかは不定（距離は必ず最小）。平均 `O(log N)`。
    fn 最近傍<X: TryInto<isize>, Y: TryInto<isize>>(&self, q: (X, Y)) -> Option<(i128, usize)> {
        if self.点.is_empty() {
            return None;
        }
        let q = (to_isize(q.0), to_isize(q.1));
        let mut 最良 = (i128::MAX, !0);
        self.近傍探索(0, self.点.len(), 0, q, &mut 最良);
        Some(最良)
    }

    /// 軸平行な閉矩形 `[x1, x2] × [y1, y2]`（端点含む）に入る点の個数を返す。最悪 `O(√N)`。
    fn 矩形内点数(
        &self,
        x1: impl TryInto<isize>,
        y1: impl TryInto<isize>,
        x2: impl TryInto<isize>,
        y2: impl TryInto<isize>,
    ) -> usize {
        let (x1, y1, x2, y2) = (to_isize(x1), to_isize(y1), to_isize(x2), to_isize(y2));
        self.範囲計数(0, self.点.len(), 0, x1, y1, x2, y2)
    }

    fn 近傍探索(
        &self,
        l: usize,
        r: usize,
        axis: usize,
        q: (isize, isize),
        最良: &mut (i128, usize),
    ) {
        if l >= r {
            return;
        }
        let m = l + (r - l) / 2;
        let (px, py, idx) = self.点[m];
        let dx = (q.0 - px) as i128;
        let dy = (q.1 - py) as i128;
        let d = dx * dx + dy * dy;
        if d < 最良.0 {
            *最良 = (d, idx);
        }
        // 分割面までの符号付き距離。
        let 面 = if axis == 0 {
            (q.0 - px) as i128
        } else {
            (q.1 - py) as i128
        };
        // q が居る側を先に、反対側は分割面が現在の最良円より近いときだけ探索する。
        let q側が左 = if axis == 0 { q.0 <= px } else { q.1 <= py };
        let (近l, 近r, 遠l, 遠r) = if q側が左 {
            (l, m, m + 1, r)
        } else {
            (m + 1, r, l, m)
        };
        self.近傍探索(近l, 近r, 1 - axis, q, 最良);
        if 面 * 面 < 最良.0 {
            self.近傍探索(遠l, 遠r, 1 - axis, q, 最良);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn 範囲計数(
        &self,
        l: usize,
        r: usize,
        axis: usize,
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    ) -> usize {
        if l >= r {
            return 0;
        }
        let m = l + (r - l) / 2;
        let (px, py, _) = self.点[m];
        let mut c = usize::from(x1 <= px && px <= x2 && y1 <= py && py <= y2);
        // 左の子は当該軸座標 ≤ px、右の子は ≥ px なので、矩形と交わる側だけ降りる。
        let (下, 上) = if axis == 0 {
            (x1 <= px, px <= x2)
        } else {
            (y1 <= py, py <= y2)
        };
        if 下 {
            c += self.範囲計数(l, m, 1 - axis, x1, y1, x2, y2);
        }
        if 上 {
            c += self.範囲計数(m + 1, r, 1 - axis, x1, y1, x2, y2);
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 最近傍素朴(点群: &[(isize, isize)], q: (isize, isize)) -> Option<(i128, usize)> {
        let mut 最良 = (i128::MAX, usize::MAX);
        for (i, &(x, y)) in 点群.iter().enumerate() {
            let dx = (q.0 - x) as i128;
            let dy = (q.1 - y) as i128;
            let d = dx * dx + dy * dy;
            if d < 最良.0 || (d == 最良.0 && i < 最良.1) {
                最良 = (d, i);
            }
        }
        if 最良.1 == usize::MAX {
            None
        } else {
            Some(最良)
        }
    }

    fn 計数素朴(
        点群: &[(isize, isize)], x1: isize, y1: isize, x2: isize, y2: isize
    ) -> usize {
        点群
            .iter()
            .filter(|&&(x, y)| x1 <= x && x <= x2 && y1 <= y && y <= y2)
            .count()
    }

    #[test]
    fn 既知例() {
        let 点 = [(0, 0), (10, 10), (5, 5), (1, 1), (9, 1)];
        let 木 = KD木::new(&点);
        // (2,2) の最近傍は (1,1)（距離2乗 2）。
        assert_eq!(木.最近傍((2, 2)), Some((2, 3)));
        // (8,2) の最近傍は (9,1)（距離2乗 2）。
        assert_eq!(木.最近傍((8, 2)), Some((2, 4)));
        // 矩形 [0,6]×[0,6] には (0,0),(5,5),(1,1) の 3 点。
        assert_eq!(木.矩形内点数(0, 0, 6, 6), 3);
        // 全体。
        assert_eq!(木.矩形内点数(-100, -100, 100, 100), 5);
        // 空の木。
        let 空 = KD木::new(&[]);
        assert_eq!(空.最近傍((0, 0)), None);
        assert_eq!(空.矩形内点数(0, 0, 1, 1), 0);
    }

    #[test]
    fn 乱数で素朴と一致() {
        let mut seed = 0x51d0_88a0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..400 {
            let n = (乱数() as usize % 30) + 1;
            let 点群: Vec<(isize, isize)> = (0..n)
                .map(|_| ((乱数() % 40) as isize - 20, (乱数() % 40) as isize - 20))
                .collect();
            let 木 = KD木::new(&点群);
            // 最近傍クエリ。距離（最小値）が一致し、返した添字が実際にその距離を達成することを確認。
            for _ in 0..20 {
                let q = ((乱数() % 50) as isize - 25, (乱数() % 50) as isize - 25);
                let 速 = 木.最近傍(q).unwrap();
                let 遅 = 最近傍素朴(&点群, q).unwrap();
                assert_eq!(速.0, 遅.0, "距離不一致 点群={点群:?} q={q:?}");
                let (px, py) = 点群[速.1];
                let 実距離 = (q.0 - px) as i128 * (q.0 - px) as i128
                    + (q.1 - py) as i128 * (q.1 - py) as i128;
                assert_eq!(実距離, 速.0, "返した添字の距離が不正 q={q:?}");
            }
            // 矩形計数クエリ。
            for _ in 0..20 {
                let a = (乱数() % 40) as isize - 20;
                let b = (乱数() % 40) as isize - 20;
                let c = (乱数() % 40) as isize - 20;
                let e = (乱数() % 40) as isize - 20;
                let (x1, x2) = (a.min(b), a.max(b));
                let (y1, y2) = (c.min(e), c.max(e));
                assert_eq!(
                    木.矩形内点数(x1, y1, x2, y2),
                    計数素朴(&点群, x1, y1, x2, y2),
                    "点群={点群:?} 矩形=({x1},{y1},{x2},{y2})"
                );
            }
        }
    }
}
