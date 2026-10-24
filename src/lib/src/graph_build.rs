use cargo_snippet::snippet;

#[snippet("無向_グラフ")]
#[snippet("隣接_リスト")]
#[snippet("グラフ_構築")]
/// 辺リストから無向グラフの隣接リストを作る。`O(n + 辺数)`。
///
/// `辺` は AtCoder の入力をそのまま 0-indexed にしたもの（`辺[i] = (u, v)`）。
/// `グラフ[u]` に `v`、`グラフ[v]` に `u` を積むだけなので、多重辺はそのまま重複して入り、
/// 自己ループ `(u, u)` は `グラフ[u]` に 2 回入る。孤立頂点があってもよいので `n` は必須。
pub fn 無向グラフ(n: usize, 辺: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut グラフ = vec![Vec::new(); n];
    for &(u, v) in 辺 {
        グラフ[u].push(v);
        グラフ[v].push(u);
    }
    グラフ
}

#[snippet("有向_グラフ")]
#[snippet("隣接_リスト")]
#[snippet("グラフ_構築")]
/// 辺リストから有向グラフの隣接リストを作る。`O(n + 辺数)`。
///
/// `辺[i] = (u, v)` を `u → v` の一方向の辺として `グラフ[u]` にだけ積む。
/// 逆辺が欲しい場合は `(v, u)` に読み替えてもう一度呼ぶ。
pub fn 有向グラフ(n: usize, 辺: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut グラフ = vec![Vec::new(); n];
    for &(u, v) in 辺 {
        グラフ[u].push(v);
    }
    グラフ
}

#[snippet("重み付き_無向_グラフ")]
#[snippet("隣接_リスト")]
#[snippet("グラフ_構築")]
/// 重み付き辺リストから無向グラフの隣接リストを作る。`O(n + 辺数)`。
///
/// `辺[i] = (u, v, w)` に対し `グラフ[u]` へ `(v, w)`、`グラフ[v]` へ `(u, w)` を積む。
/// 重みの型 `T` は `Copy` であれば何でもよい（`isize`・`u64`・`(isize, usize)` など）。
pub fn 重み付き無向グラフ<T: Copy>(
    n: usize,
    辺: &[(usize, usize, T)],
) -> Vec<Vec<(usize, T)>> {
    let mut グラフ = vec![Vec::new(); n];
    for &(u, v, w) in 辺 {
        グラフ[u].push((v, w));
        グラフ[v].push((u, w));
    }
    グラフ
}

#[snippet("重み付き_有向_グラフ")]
#[snippet("隣接_リスト")]
#[snippet("グラフ_構築")]
/// 重み付き辺リストから有向グラフの隣接リストを作る。`O(n + 辺数)`。
///
/// `辺[i] = (u, v, w)` を `u → v`（重み `w`）として `グラフ[u]` にだけ積む。
fn 重み付き有向グラフ<T: Copy>(
    n: usize,
    辺: &[(usize, usize, T)],
) -> Vec<Vec<(usize, T)>> {
    let mut グラフ = vec![Vec::new(); n];
    for &(u, v, w) in 辺 {
        グラフ[u].push((v, w));
    }
    グラフ
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 無向は両向きに積まれる() {
        let g = 無向グラフ(4, &[(0, 1), (1, 2), (0, 3)]);
        assert_eq!(g, vec![vec![1, 3], vec![0, 2], vec![1], vec![0]]);
    }

    #[test]
    fn 孤立頂点は空のまま() {
        let g = 無向グラフ(3, &[(0, 1)]);
        assert_eq!(g, vec![vec![1], vec![0], vec![]]);
    }

    #[test]
    fn 多重辺と自己ループはそのまま入る() {
        let g = 無向グラフ(2, &[(0, 1), (0, 1), (0, 0)]);
        assert_eq!(g, vec![vec![1, 1, 0, 0], vec![0, 0]]);
    }

    #[test]
    fn 有向は片向きだけ() {
        let g = 有向グラフ(3, &[(0, 1), (0, 2), (2, 0)]);
        assert_eq!(g, vec![vec![1, 2], vec![], vec![0]]);
    }

    #[test]
    fn 重み付き無向は重みごと両向き() {
        let g = 重み付き無向グラフ(3, &[(0, 1, 5isize), (1, 2, -3)]);
        assert_eq!(g, vec![vec![(1, 5)], vec![(0, 5), (2, -3)], vec![(1, -3)]]);
    }

    #[test]
    fn 重み付き有向は片向きだけ() {
        let g = 重み付き有向グラフ(3, &[(0, 1, 5isize), (1, 2, -3)]);
        assert_eq!(g, vec![vec![(1, 5)], vec![(2, -3)], vec![]]);
    }
}
