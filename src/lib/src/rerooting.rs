use crate::graph_build::無向グラフ;
use cargo_snippet::snippet;

#[snippet("全方位_木_DP_汎用", include = "無向_グラフ")]
#[snippet("Rerooting")]
#[snippet("ReRooting")]
/// 全方位木DP（rerooting）を任意の可換モノイドで行うための演算インタフェース。
///
/// 部分木の集約値の型 [`値`](Self::値) に対し、子方向の結果をまとめる可換モノイド
/// （[`単位元`](Self::単位元)・[`併合`](Self::併合)）、子部分木の値を辺で親側へ持ち上げる
/// [`辺寄与`](Self::辺寄与)、子方向寄与の併合に頂点自身を載せる [`頂点載せ`](Self::頂点載せ)
/// を与える。[`全方位木dp実行`] がこれらを使い、全頂点を根にした答えを `O(N)` でまとめて返す。
///
/// [`rerooting_distance`](crate::rerooting_distance) は「距離総和」専用の固定実装で、本トレイトは
/// 任意の集約（個数・最大深さ・木 DP など）を載せられる汎用版。
trait 全方位木DP {
    /// 部分木の集約値。
    type 値: Clone;
    /// [`併合`](Self::併合) の単位元。
    fn 単位元(&self) -> Self::値;
    /// 子方向の結果同士をまとめる可換モノイド演算。
    fn 併合(&self, a: &Self::値, b: &Self::値) -> Self::値;
    /// 子頂点 `子` を根とする部分木の集約 `下` を、辺 `(親, 子)` を通して親 `親` 側の寄与に変換する。
    fn 辺寄与(&self, 下: &Self::値, 親: usize, 子: usize) -> Self::値;
    /// 頂点 `v` の子方向寄与の併合結果 `集約` に、頂点 `v` 自身を載せて部分木値にする。
    fn 頂点載せ(&self, 集約: &Self::値, v: usize) -> Self::値;
}

#[snippet("全方位_木_DP_汎用")]
#[snippet("Rerooting")]
#[snippet("ReRooting")]
/// 頂点数 `n`・辺リスト `辺` の木について、各頂点 `v` を根としたときの部分木集約値を `O(N)` で求める。
///
/// `辺` は AtCoder の入力そのまま（0-indexed の頂点対）を渡せばよい。
fn 全方位木dp実行<S: 全方位木DP>(
    n: usize,
    辺: &[(usize, usize)],
    状態: &S,
) -> Vec<S::値> {
    if n == 0 {
        return Vec::new();
    }
    let グラフ = 無向グラフ(n, 辺);
    // 根 0 から DFS 前順と親を求める。
    let mut prev = vec![!0; n];
    let mut 順 = Vec::with_capacity(n);
    let mut 到達済 = vec![false; n];
    let mut 探索 = vec![0usize];
    到達済[0] = true;
    while let Some(now) = 探索.pop() {
        順.push(now);
        for &next in &グラフ[now] {
            if !到達済[next] {
                到達済[next] = true;
                prev[next] = now;
                探索.push(next);
            }
        }
    }
    // 葉→根: 部分木集約 down[now]。
    let mut down: Vec<S::値> = (0..n).map(|_| 状態.単位元()).collect();
    for &now in 順.iter().rev() {
        let mut acc = 状態.単位元();
        for &next in &グラフ[now] {
            if next != prev[now] {
                acc = 状態.併合(&acc, &状態.辺寄与(&down[next], now, next));
            }
        }
        down[now] = 状態.頂点載せ(&acc, now);
    }
    // 根→葉: 親方向の寄与 α(now, parent) を伝播し、各頂点の答えを確定。
    let mut 親寄与: Vec<S::値> = (0..n).map(|_| 状態.単位元()).collect(); // 非根で有効
    let mut 答え: Vec<S::値> = (0..n).map(|_| 状態.単位元()).collect();
    for &now in &順 {
        let deg = グラフ[now].len();
        // 各隣接の寄与 α を隣接順に並べる（親方向は 親寄与[now]）。
        let mut 寄与列: Vec<S::値> = Vec::with_capacity(deg);
        for &next in &グラフ[now] {
            if next == prev[now] {
                寄与列.push(親寄与[now].clone());
            } else {
                寄与列.push(状態.辺寄与(&down[next], now, next));
            }
        }
        let mut pre = vec![状態.単位元(); deg + 1];
        for k in 0..deg {
            pre[k + 1] = 状態.併合(&pre[k], &寄与列[k]);
        }
        let mut suf = vec![状態.単位元(); deg + 1];
        for k in (0..deg).rev() {
            suf[k] = 状態.併合(&寄与列[k], &suf[k + 1]);
        }
        答え[now] = 状態.頂点載せ(&pre[deg], now);
        // 子へ「親方向（自分を除いた併合 + 頂点）」を辺で渡す。
        for (k, &next) in グラフ[now].iter().enumerate() {
            if next != prev[now] {
                let m = 状態.併合(&pre[k], &suf[k + 1]);
                let 親側 = 状態.頂点載せ(&m, now);
                親寄与[next] = 状態.辺寄与(&親側, next, now);
            }
        }
    }
    答え
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    // 距離総和: 値 = (部分木の頂点数, その頂点数に対する距離和)。
    struct 距離和;
    impl 全方位木DP for 距離和 {
        type 値 = (isize, isize);
        fn 単位元(&self) -> (isize, isize) {
            (0, 0)
        }
        fn 併合(&self, a: &(isize, isize), b: &(isize, isize)) -> (isize, isize) {
            (a.0 + b.0, a.1 + b.1)
        }
        fn 辺寄与(&self, 下: &(isize, isize), _親: usize, _子: usize) -> (isize, isize) {
            (下.0, 下.1 + 下.0) // 辺を 1 本越えると下側 a 頂点すべての距離が +1
        }
        fn 頂点載せ(&self, 集約: &(isize, isize), _v: usize) -> (isize, isize) {
            (集約.0 + 1, 集約.1) // 自身を 1 頂点追加（自分への距離 0）
        }
    }

    // 偏心(eccentricity): 値 = その方向の最遠距離。
    struct 偏心;
    impl 全方位木DP for 偏心 {
        type 値 = isize;
        fn 単位元(&self) -> isize {
            isize::MIN
        }
        fn 併合(&self, a: &isize, b: &isize) -> isize {
            *a.max(b)
        }
        fn 辺寄与(&self, 下: &isize, _親: usize, _子: usize) -> isize {
            下 + 1
        }
        fn 頂点載せ(&self, 集約: &isize, _v: usize) -> isize {
            (*集約).max(0) // 自身は距離 0
        }
    }

    // BFS による独立オラクル: 始点 src からの全頂点への距離。
    fn bfs距離(隣接: &[Vec<usize>], src: usize) -> Vec<isize> {
        let n = 隣接.len();
        let mut 距離 = vec![-1isize; n];
        let mut q = VecDeque::new();
        距離[src] = 0;
        q.push_back(src);
        while let Some(v) = q.pop_front() {
            for &u in &隣接[v] {
                if 距離[u] < 0 {
                    距離[u] = 距離[v] + 1;
                    q.push_back(u);
                }
            }
        }
        距離
    }

    #[test]
    fn 距離和と偏心がbfsと一致() {
        let mut seed = 0x6c01_aa20u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..500 {
            let n = (乱数() as usize % 40) + 1;
            let mut 辺 = Vec::new();
            for i in 1..n {
                let p = (乱数() as usize) % i;
                辺.push((p, i));
            }
            let 隣接 = 無向グラフ(n, &辺);
            let 和 = 全方位木dp実行(n, &辺, &距離和);
            let ecc = 全方位木dp実行(n, &辺, &偏心);
            for v in 0..n {
                let d = bfs距離(&隣接, v);
                let 期待和: isize = d.iter().sum();
                let 期待ecc: isize = d.iter().copied().max().unwrap();
                assert_eq!(和[v].1, 期待和, "距離和 n={n} v={v}");
                assert_eq!(和[v].0, n as isize, "頂点数 n={n} v={v}");
                assert_eq!(ecc[v], 期待ecc, "偏心 n={n} v={v}");
            }
        }
    }

    #[test]
    fn 一頂点() {
        let 和 = 全方位木dp実行(1, &[], &距離和);
        assert_eq!(和, vec![(1, 0)]);
        let ecc = 全方位木dp実行(1, &[], &偏心);
        assert_eq!(ecc, vec![0]);
    }
}
