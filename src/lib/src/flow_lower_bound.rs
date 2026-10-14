use crate::to_int::to_i64;
use cargo_snippet::snippet;

#[snippet("下限付き_最大流")]
#[snippet("LowerBoundFlow")]
#[snippet("BoundedFlow")]
/// 各辺に流量の**下限・上限**を課したネットワークで、`s` から `t` への最大流を求める。
///
/// 内部は [`ac_library::MfGraph`] の Dinic なので `O(V²E)`。
/// 下限を満たす循環の存在判定（実行可能流）も同時に得られる。
struct 下限付き最大流 {
    n: usize,
    辺: Vec<(usize, usize, i64, i64)>, // (from, to, 下限, 上限)
}

#[snippet("下限付き_最大流", include = "to_i64")]
#[snippet("LowerBoundFlow")]
#[snippet("BoundedFlow")]
impl 下限付き最大流 {
    /// 頂点数 `n` の空ネットワークを作る。
    fn new(n: usize) -> Self {
        下限付き最大流 { n, 辺: Vec::new() }
    }

    /// 辺 `from → to` を下限 `下限`・上限 `上限`（`0 ≤ 下限 ≤ 上限`）で追加し、その辺番号を返す。
    fn 辺追加(
        &mut self,
        from: usize,
        to: usize,
        下限: impl TryInto<i64>,
        上限: impl TryInto<i64>,
    ) -> usize {
        let (下限, 上限) = (to_i64(下限), to_i64(上限));
        assert!(from < self.n && to < self.n);
        assert!(0 <= 下限 && 下限 <= 上限);
        let id = self.辺.len();
        self.辺.push((from, to, 下限, 上限));
        id
    }

    /// `s → t` の最大流を解く。実行可能なら `Some((最大流量, 各辺の流量))`、
    /// 下限を満たす流が存在しなければ `None`。各辺の流量は [`辺追加`](Self::辺追加) の順。
    fn 解く(&self, s: usize, t: usize) -> Option<(i64, Vec<i64>)> {
        use ac_library::MfGraph;
        const INF: i64 = 1 << 60;
        let n = self.n;
        let (超源, 超匯) = (n, n + 1);
        let mut g = MfGraph::<i64>::new(n + 2);
        let mut 辺id = Vec::with_capacity(self.辺.len());
        let mut d = vec![0i64; n]; // 各頂点の過不足
        for &(u, v, lo, cap) in &self.辺 {
            辺id.push(g.add_edge(u, v, cap - lo));
            d[u] -= lo;
            d[v] += lo;
        }
        // 無限容量の戻り辺 t→s で循環に帰着。
        let 戻り = g.add_edge(t, s, INF);
        let mut 必要 = 0i64;
        for (v, &dv) in d.iter().enumerate() {
            if dv > 0 {
                g.add_edge(超源, v, dv);
                必要 += dv;
            } else if dv < 0 {
                g.add_edge(v, 超匯, -dv);
            }
        }
        // 実行可能性: 超源→超匯 が過不足総和を飽和できるか。
        if g.flow(超源, 超匯) != 必要 {
            return None;
        }
        // 実行可能。戻り辺の流量が現時点の s→t 流量。
        let f0 = g.get_edge(戻り).flow;
        // 戻り辺を取り除き、残余で s→t を最大化。
        g.change_edge(戻り, 0, 0);
        let 追加 = g.flow(s, t);
        let 値 = f0 + 追加;
        // 各辺の実流量 = 下限 + 容量 c−l 辺に流れた分。
        let flows = self
            .辺
            .iter()
            .zip(&辺id)
            .map(|(&(_, _, lo, _), &id)| lo + g.get_edge(id).flow)
            .collect();
        Some((値, flows))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ac_library::MfGraph;

    #[test]
    fn 既知例() {
        // 下限が流量を強制する例。s=0, t=3。
        // 0->1 [2,5], 1->3 [0,5], 0->2 [0,5], 2->3 [3,5]
        let mut net = 下限付き最大流::new(4);
        net.辺追加(0, 1, 2, 5);
        net.辺追加(1, 3, 0, 5);
        net.辺追加(0, 2, 0, 5);
        net.辺追加(2, 3, 3, 5);
        let (値, f) = net.解く(0, 3).unwrap();
        // 各辺が下限・上限を満たす。
        assert!(f[0] >= 2 && f[0] <= 5);
        assert!(f[3] >= 3 && f[3] <= 5);
        // 流量保存: 頂点1,2 で入=出。
        assert_eq!(f[0], f[1]);
        assert_eq!(f[2], f[3]);
        // s から出る総量 = 値。
        assert_eq!(f[0] + f[2], 値);
    }

    #[test]
    fn 実行不能を検出() {
        // 0->1 の下限 3 だが 1 からの出口上限が 1 → 保存できず不能。
        let mut net = 下限付き最大流::new(3);
        net.辺追加(0, 1, 3, 5);
        net.辺追加(1, 2, 0, 1);
        assert!(net.解く(0, 2).is_none());
    }

    // 全整数流量を総当たりして最大の s 純流出を求める独立オラクル。
    fn 総当たり(n: usize, 辺: &[(usize, usize, i64, i64)], s: usize, t: usize) -> Option<i64> {
        let m = 辺.len();
        let mut f = vec![0i64; m];
        // 各辺は下限から始める。
        for (i, &(_, _, lo, _)) in 辺.iter().enumerate() {
            f[i] = lo;
        }
        let mut 最良: Option<i64> = None;
        loop {
            // 保存則を確認（s, t 以外は入=出）。
            let mut 純: Vec<i64> = vec![0; n]; // 出 − 入
            for (i, &(u, v, _, _)) in 辺.iter().enumerate() {
                純[u] += f[i];
                純[v] -= f[i];
            }
            let 可 = (0..n).all(|w| w == s || w == t || 純[w] == 0);
            if 可 {
                let 値 = 純[s]; // s の純流出
                if 値 >= 0 {
                    最良 = Some(最良.map_or(値, |b: i64| b.max(値)));
                }
            }
            // 次の流量ベクトルへ（各辺 [下限, 上限] の桁上がり）。
            let mut k = 0;
            while k < m {
                if f[k] < 辺[k].3 {
                    f[k] += 1;
                    break;
                } else {
                    f[k] = 辺[k].2;
                    k += 1;
                }
            }
            if k == m {
                break;
            }
        }
        最良
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x83c2_41a0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..4000 {
            let n = (乱数() as usize % 4) + 2; // 2..=5
            let m = (乱数() as usize % 6) + 1; // 1..=6
            let 辺: Vec<(usize, usize, i64, i64)> = (0..m)
                .map(|_| {
                    let u = 乱数() as usize % n;
                    let mut v = 乱数() as usize % n;
                    if v == u {
                        v = (v + 1) % n;
                    }
                    let cap = (乱数() % 4) as i64; // 0..=3
                    let lo = if cap == 0 {
                        0
                    } else {
                        (乱数() % (cap as u64 + 1)) as i64
                    };
                    (u, v, lo, cap)
                })
                .collect();
            let s = 乱数() as usize % n;
            let mut t = 乱数() as usize % n;
            if t == s {
                t = (t + 1) % n;
            }
            let mut net = 下限付き最大流::new(n);
            for &(u, v, lo, cap) in &辺 {
                net.辺追加(u, v, lo, cap);
            }
            let 期待 = 総当たり(n, &辺, s, t);
            match (net.解く(s, t), 期待) {
                (None, None) => {}
                (Some((値, f)), Some(e)) => {
                    assert_eq!(値, e, "辺={辺:?} s={s} t={t}");
                    // 返した流量が下限上限・保存則を満たすことも確認。
                    let mut 純 = vec![0i64; n];
                    for (i, &(u, v, lo, cap)) in 辺.iter().enumerate() {
                        assert!(lo <= f[i] && f[i] <= cap, "辺{i} の流量 {} が範囲外", f[i]);
                        純[u] += f[i];
                        純[v] -= f[i];
                    }
                    for (w, &純w) in 純.iter().enumerate() {
                        if w != s && w != t {
                            assert_eq!(純w, 0, "頂点{w} で保存則違反");
                        }
                    }
                    assert_eq!(純[s], 値, "s の純流出が値と不一致");
                }
                (a, b) => panic!("実行可能性が不一致 自前={a:?} 期待={b:?} 辺={辺:?} s={s} t={t}"),
            }
        }
    }

    #[test]
    fn 下限ゼロは素の最大流に一致() {
        let mut seed = 0x9d70_5520u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..600 {
            let n = (乱数() as usize % 7) + 2; // 2..=8
            let m = (乱数() as usize % 12) + 1;
            let 辺: Vec<(usize, usize, i64)> = (0..m)
                .map(|_| {
                    let u = 乱数() as usize % n;
                    let mut v = 乱数() as usize % n;
                    if v == u {
                        v = (v + 1) % n;
                    }
                    let cap = (乱数() % 11) as i64;
                    (u, v, cap)
                })
                .collect();
            let s = 乱数() as usize % n;
            let mut t = 乱数() as usize % n;
            if t == s {
                t = (t + 1) % n;
            }
            // 下限 0 で自前。
            let mut net = 下限付き最大流::new(n);
            for &(u, v, cap) in &辺 {
                net.辺追加(u, v, 0, cap);
            }
            let (値, _) = net.解く(s, t).unwrap();
            // 素の ac_library 最大流。
            let mut g = MfGraph::<i64>::new(n);
            for &(u, v, cap) in &辺 {
                g.add_edge(u, v, cap);
            }
            assert_eq!(値, g.flow(s, t), "辺={辺:?} s={s} t={t}");
        }
    }
}
