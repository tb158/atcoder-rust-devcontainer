use crate::graph_build::無向グラフ;
use cargo_snippet::snippet;
#[snippet("AHU")]
#[snippet("TreeIsomorphism")]
#[snippet("木の_ハッシュ")]
#[snippet("木の_同型_判定")]
#[snippet("根付き木の_正準化")]
use std::collections::HashMap;

#[snippet("木の_同型_判定", include = "無向_グラフ")]
#[snippet("木の_ハッシュ")]
#[snippet("TreeIsomorphism")]
#[snippet("AHU")]
#[snippet("根付き木の_正準化")]
/// 木の同型判定（AHU アルゴリズムによる正準ラベル付け）。子の正準ラベルを整列して
/// 共有辞書で番号付けすることで、衝突のない正準ラベルを与える。構築は木 1 本あたり O(N log N)。
///
/// 木は `(頂点数, 辺リスト)` で渡す。`辺` は AtCoder の入力そのまま（0-indexed の頂点対）。
/// 同じ [`木の正準化`] インスタンスへ複数の木を渡し、返り値のラベルが等しいかで同型を判定する。
/// ハッシュではなく辞書による完全一致なので誤判定しない。
///
/// - [`Self::根付きラベル`] : 根を固定した木の正準ラベル。
/// - [`Self::無向ラベル`] : 根を持たない木の正準ラベル（重心で根付ける）。
struct 木の正準化 {
    辞書: HashMap<Vec<u64>, u64>,
    次の番号: u64,
}

#[snippet("木の_同型_判定")]
#[snippet("木の_ハッシュ")]
#[snippet("TreeIsomorphism")]
#[snippet("AHU")]
#[snippet("根付き木の_正準化")]
impl 木の正準化 {
    fn new() -> Self {
        Self {
            辞書: HashMap::new(),
            次の番号: 0,
        }
    }

    /// 子ラベル列に対応する番号を引く（無ければ採番）。
    fn 登録(&mut self, mut 子ラベル: Vec<u64>) -> u64 {
        子ラベル.sort_unstable();
        if let Some(&id) = self.辞書.get(&子ラベル) {
            return id;
        }
        let id = self.次の番号;
        self.次の番号 += 1;
        self.辞書.insert(子ラベル, id);
        id
    }

    fn 探索(&mut self, グラフ: &[Vec<usize>], now: usize, prev: usize) -> u64 {
        let mut 子ラベル = Vec::new();
        for &next in &グラフ[now] {
            if next != prev {
                子ラベル.push(self.探索(グラフ, next, now));
            }
        }
        self.登録(子ラベル)
    }

    /// 根 `根` を固定した木の正準ラベルを返す。
    fn 根付きラベル(&mut self, n: usize, 辺: &[(usize, usize)], 根: usize) -> u64 {
        self.探索(&無向グラフ(n, 辺), 根, !0)
    }

    /// 根を持たない木の正準ラベル。重心（1 個または 2 個）で根付けたラベルを束ねる。
    fn 無向ラベル(&mut self, n: usize, 辺: &[(usize, usize)]) -> u64 {
        let グラフ = 無向グラフ(n, 辺);
        let 重心 = 重心列(&グラフ);
        let ラベル: Vec<u64> = 重心
            .iter()
            .map(|&c| self.探索(&グラフ, c, !0))
            .collect();
        self.登録(ラベル)
    }

    /// 2 つの根付き木が同型か。
    fn 根付き同型(
        &mut self,
        na: usize,
        辺a: &[(usize, usize)],
        根a: usize,
        nb: usize,
        辺b: &[(usize, usize)],
        根b: usize,
    ) -> bool {
        self.根付きラベル(na, 辺a, 根a) == self.根付きラベル(nb, 辺b, 根b)
    }

    /// 2 つの無向木が同型か。
    fn 無向同型(
        &mut self,
        na: usize,
        辺a: &[(usize, usize)],
        nb: usize,
        辺b: &[(usize, usize)],
    ) -> bool {
        na == nb && self.無向ラベル(na, 辺a) == self.無向ラベル(nb, 辺b)
    }
}

#[snippet("木の_同型_判定")]
#[snippet("木の_ハッシュ")]
#[snippet("TreeIsomorphism")]
#[snippet("AHU")]
#[snippet("根付き木の_正準化")]
impl Default for 木の正準化 {
    fn default() -> Self {
        Self::new()
    }
}

#[snippet("木の_同型_判定")]
#[snippet("木の_ハッシュ")]
#[snippet("TreeIsomorphism")]
#[snippet("AHU")]
#[snippet("根付き木の_正準化")]
/// 木の重心（最大部分木が N/2 以下になる頂点）を求める。1 個または 2 個。
fn 重心列(グラフ: &[Vec<usize>]) -> Vec<usize> {
    let n = グラフ.len();
    let mut sz = vec![0usize; n];
    let mut 重心 = Vec::new();
    fn dfs(
        now: usize,
        prev: usize,
        n: usize,
        グラフ: &[Vec<usize>],
        sz: &mut [usize],
        重心: &mut Vec<usize>,
    ) {
        sz[now] = 1;
        let mut 最大 = 0;
        for &next in &グラフ[now] {
            if next == prev {
                continue;
            }
            dfs(next, now, n, グラフ, sz, 重心);
            sz[now] += sz[next];
            最大 = 最大.max(sz[next]);
        }
        最大 = 最大.max(n - sz[now]);
        if 最大 * 2 <= n {
            重心.push(now);
        }
    }
    dfs(0, !0, n, グラフ, &mut sz, &mut 重心);
    重心
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 形が同じ根付き木は同型() {
        // どちらも「根に子 2 つ、片方の子にさらに子 1 つ」
        let a = [(0, 1), (0, 2), (1, 3)];
        let b = [(0, 2), (0, 1), (2, 3)];
        let mut 判定 = 木の正準化::new();
        assert!(判定.根付き同型(4, &a, 0, 4, &b, 0));
    }

    #[test]
    fn 形が違う根付き木は非同型() {
        // a: 鎖 0-1-2-3, b: 星 0-{1,2,3}
        let a = [(0, 1), (1, 2), (2, 3)];
        let b = [(0, 1), (0, 2), (0, 3)];
        let mut 判定 = 木の正準化::new();
        assert!(!判定.根付き同型(4, &a, 0, 4, &b, 0));
    }

    #[test]
    fn 無向木は根の取り方に依らず同型判定できる() {
        // 同じ鎖を別の頂点番号で構成
        let a = [(0, 1), (1, 2), (2, 3), (3, 4)];
        let b = [(4, 3), (3, 2), (2, 1), (1, 0)];
        let mut 判定 = 木の正準化::new();
        assert!(判定.無向同型(5, &a, 5, &b));
    }

    #[test]
    fn 二重心の木も判定できる() {
        // 6 頂点の鎖は中央 2 頂点が重心
        let a = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)];
        // 同じ鎖を並べ替え
        let b = [(5, 4), (4, 3), (3, 2), (2, 1), (1, 0)];
        let c = [(0, 1), (0, 2), (0, 3), (3, 4), (4, 5)]; // 別形状
        let mut 判定 = 木の正準化::new();
        assert!(判定.無向同型(6, &a, 6, &b));
        assert!(!判定.無向同型(6, &a, 6, &c));
    }
}
