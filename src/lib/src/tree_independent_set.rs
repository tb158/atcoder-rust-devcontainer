use crate::graph_build::無向グラフ;
use cargo_snippet::snippet;

#[snippet("木の_最大_重み_独立_集合", include = "無向_グラフ")]
#[snippet("木の_独立_集合")]
#[snippet("TreeMWIS")]
/// 木（または森）の**最大重み独立集合**を求める。隣接する 2 頂点を同時に選べない条件の下で、
/// 選んだ頂点の重み和を最大化する。最大重みと、各頂点を選んだか否かの `Vec<bool>` を返す。`O(N)`。
///
/// 重みが負の頂点は選ばない方が得になり、空集合（重み 0）も許す。連結でなくてもよい
/// （各連結成分を独立に処理）。再帰しないので深い木でもスタック安全。
///
/// 一般グラフの最大独立集合は NP 困難（[`max_clique`](crate::max_clique) の補グラフ版）で別物。
/// 木に限れば本関数が線形時間で厳密解を与える。
///
/// `辺` は AtCoder の入力そのまま（0-indexed の頂点対）で、頂点数は `重み.len()` から決まる。
///
/// 重みの型 `T` は順序比較・加算でき、`T::default()` が空集合の重み 0（加法単位元）に
/// なる型なら何でもよい（`isize`・`f64` など）。
fn 木の最大重み独立集合<T>(辺: &[(usize, usize)], 重み: &[T]) -> (T, Vec<bool>)
where
    T: Copy + Ord + Default + std::ops::Add<Output = T> + std::ops::AddAssign,
{
    let n = 重み.len();
    let グラフ = 無向グラフ(n, 辺);
    let mut prev = vec![!0; n];
    let mut 順 = Vec::with_capacity(n); // BFS 訪問順（後で逆順 = 後行順に使う）
    let mut 到達済 = vec![false; n];
    // 森に対応：未訪問の頂点それぞれを根に BFS。
    for s in 0..n {
        if 到達済[s] {
            continue;
        }
        到達済[s] = true;
        let mut 探索 = std::collections::VecDeque::new();
        探索.push_back(s);
        while let Some(now) = 探索.pop_front() {
            順.push(now);
            for &next in &グラフ[now] {
                if !到達済[next] {
                    到達済[next] = true;
                    prev[next] = now;
                    探索.push_back(next);
                }
            }
        }
    }
    let mut 選ぶ = 重み.to_vec();
    let mut 選ばない = vec![T::default(); n];
    // 葉から（BFS 逆順）集約。
    for &v in 順.iter().rev() {
        let p = prev[v];
        if p != !0 {
            選ぶ[p] += 選ばない[v];
            // 同じ配列を読みつつ更新するため、子の寄与を一旦取り出してから足す。
            let 子寄与 = 選ぶ[v].max(選ばない[v]);
            選ばない[p] += 子寄与;
        }
    }
    // 復元：根から降りる（順 は前順なので親が先に確定）。
    let mut 採用 = vec![false; n];
    let mut 親が採用 = vec![false; n];
    for &v in &順 {
        if 親が採用[v] {
            採用[v] = false; // 親が選ばれている → 選べない
        } else {
            採用[v] = 選ぶ[v] >= 選ばない[v];
        }
        if 採用[v] {
            for &next in &グラフ[v] {
                if prev[next] == v {
                    親が採用[next] = true;
                }
            }
        }
    }
    let 合計: T = 採用
        .iter()
        .enumerate()
        .filter(|&(_, &b)| b)
        .fold(T::default(), |acc, (i, _)| acc + 重み[i]);
    (合計, 採用)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 全 2^n 部分集合を試す独立集合オラクル。
    fn 総当たり(辺: &[(usize, usize)], 重み: &[isize]) -> isize {
        let n = 重み.len();
        let mut 最良 = 0isize; // 空集合
        for s in 0u32..(1 << n) {
            // 独立性チェック。
            if 辺
                .iter()
                .all(|&(a, b)| s & (1 << a) == 0 || s & (1 << b) == 0)
            {
                let w: isize = (0..n).filter(|&i| s & (1 << i) != 0).map(|i| 重み[i]).sum();
                最良 = 最良.max(w);
            }
        }
        最良
    }

    // 採用集合が独立かつ重み和が値と一致するか検証。
    fn 採用検証(辺: &[(usize, usize)], 重み: &[isize], 値: isize, 採用: &[bool]) {
        assert!(
            辺.iter().all(|&(a, b)| !採用[a] || !採用[b]),
            "独立でない 採用={採用:?}"
        );
        let w: isize = (0..重み.len()).filter(|&i| 採用[i]).map(|i| 重み[i]).sum();
        assert_eq!(w, 値, "採用の重み和が値と不一致");
    }

    #[test]
    fn 既知例() {
        // パス 0-1-2、重み 3,5,3 → 端 2 つ取って 6 < 中央 5 ではなく、3+3=6。
        let 辺 = vec![(0, 1), (1, 2)];
        let 重み = vec![3, 5, 3];
        let (v, a) = 木の最大重み独立集合(&辺, &重み);
        assert_eq!(v, 6);
        採用検証(&辺, &重み, v, &a);
        // 中央が大きい場合は中央 1 つ。
        let 重み2 = vec![3, 10, 3];
        let (v2, _) = 木の最大重み独立集合(&辺, &重み2);
        assert_eq!(v2, 10);
        // 全て負 → 空集合（0）。
        let (v3, a3) = 木の最大重み独立集合(&辺, &[-1, -2, -3]);
        assert_eq!(v3, 0);
        assert!(a3.iter().all(|&b| !b));
        // 単独頂点。
        let (v4, _) = 木の最大重み独立集合(&[], &[7]);
        assert_eq!(v4, 7);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x2bf6_71a0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..2000 {
            let n = (乱数() as usize % 11) + 1; // 1..=11
            // ランダム木：頂点 i (>=1) を 0..i のランダムな親に繋ぐ。
            let mut 辺 = Vec::new();
            for i in 1..n {
                let p = 乱数() as usize % i;
                辺.push((p, i));
            }
            // 半分の確率で森にする（最後の辺を落とす）ことはせず、木として検証。
            let 重み: Vec<isize> = (0..n).map(|_| (乱数() % 21) as isize - 10).collect();
            let (値, 採用) = 木の最大重み独立集合(&辺, &重み);
            let 期待 = 総当たり(&辺, &重み);
            assert_eq!(値, 期待, "n={n} 辺={辺:?} 重み={重み:?}");
            採用検証(&辺, &重み, 値, &採用);
        }
    }

    #[test]
    fn 森でも正しい() {
        // 2 つの成分：0-1（重み 5,1）と 2（重み 4）。
        let 辺 = vec![(0, 1)];
        let (値, 採用) = 木の最大重み独立集合(&辺, &[5, 1, 4]);
        assert_eq!(値, 9); // 頂点 0 と 2
        assert!(採用[0] && !採用[1] && 採用[2]);
    }
}
