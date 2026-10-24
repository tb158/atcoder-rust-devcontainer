use cargo_snippet::snippet;
#[snippet("HopcroftKarp")]
#[snippet("ホップクロフト_カープ")]
#[snippet("二部_グラフ_マッチング")]
#[snippet("二部_マッチング")]
#[snippet("最大_二部_マッチング")]
use std::collections::VecDeque;

#[snippet("ホップクロフト_カープ")]
#[snippet("二部_マッチング")]
#[snippet("最大_二部_マッチング")]
#[snippet("HopcroftKarp")]
#[snippet("二部_グラフ_マッチング")]
/// Hopcroft-Karp 法による二部グラフの最大マッチング。O(E√V)。
///
/// `ac_library::MaxFlow` でも最大二部マッチングは解けるが、こちらは二部マッチング専用で
/// E√V と高速。`pathfinding::kuhn_munkres` は重み付き割当専用なので、
/// 単純な最大本数マッチングにはこちらを使う。
///
/// - [`Self::最大マッチング`] : マッチした本数を返す（内部に対応も保持）。
/// - [`Self::マッチング`] : マッチした `(左, 右)` の組を返す。
pub struct ホップクロフトカープ {
    左数: usize,
    グラフ: Vec<Vec<usize>>,
    左相手: Vec<isize>,
    右相手: Vec<isize>,
    距離: Vec<isize>,
}

#[snippet("ホップクロフト_カープ")]
#[snippet("二部_マッチング")]
#[snippet("最大_二部_マッチング")]
#[snippet("HopcroftKarp")]
#[snippet("二部_グラフ_マッチング")]
impl ホップクロフトカープ {
    /// 左頂点 `左数` 個・右頂点 `右数` 個の二部グラフを作る。
    pub fn new(左数: usize, 右数: usize) -> Self {
        Self {
            左数,
            グラフ: vec![Vec::new(); 左数],
            左相手: vec![-1; 左数],
            右相手: vec![-1; 右数],
            距離: vec![-1; 左数],
        }
    }

    /// 左頂点 `u` と右頂点 `v` を結ぶ辺を追加する。
    pub fn 辺追加(&mut self, u: usize, v: usize) {
        self.グラフ[u].push(v);
    }

    /// 未マッチ左頂点から BFS して層（距離）を作る。増加道が存在すれば true。
    fn bfs(&mut self) -> bool {
        let mut 探索 = VecDeque::new();
        for u in 0..self.左数 {
            if self.左相手[u] < 0 {
                self.距離[u] = 0;
                探索.push_back(u);
            } else {
                self.距離[u] = -1;
            }
        }
        let mut 増加道あり = false;
        while let Some(u) = 探索.pop_front() {
            for i in 0..self.グラフ[u].len() {
                let v = self.グラフ[u][i];
                let w = self.右相手[v];
                if w < 0 {
                    増加道あり = true;
                } else {
                    let w = w as usize;
                    if self.距離[w] < 0 {
                        self.距離[w] = self.距離[u] + 1;
                        探索.push_back(w);
                    }
                }
            }
        }
        増加道あり
    }

    /// 層に沿って `u` から増加道を探し、見つかれば反転して true。
    // 再帰中に self を可変借用するため隣接は添字で走査する
    #[allow(clippy::needless_range_loop)]
    fn dfs(&mut self, u: usize) -> bool {
        for i in 0..self.グラフ[u].len() {
            let v = self.グラフ[u][i];
            let w = self.右相手[v];
            let 進める =
                w < 0 || (self.距離[w as usize] == self.距離[u] + 1 && self.dfs(w as usize));
            if 進める {
                self.左相手[u] = v as isize;
                self.右相手[v] = u as isize;
                return true;
            }
        }
        self.距離[u] = -1;
        false
    }

    /// 最大マッチングの本数を求める。
    pub fn 最大マッチング(&mut self) -> usize {
        let mut 本数 = 0;
        while self.bfs() {
            for u in 0..self.左数 {
                if self.左相手[u] < 0 && self.dfs(u) {
                    本数 += 1;
                }
            }
        }
        本数
    }

    /// マッチした `(左, 右)` の組を返す。先に [`Self::最大マッチング`] を呼ぶこと。
    pub fn マッチング(&self) -> Vec<(usize, usize)> {
        (0..self.左数)
            .filter(|&u| self.左相手[u] >= 0)
            .map(|u| (u, self.左相手[u] as usize))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 独立な検証用：Kuhn 法（単一増加道）で最大マッチング本数を求める
    fn kuhn(左数: usize, 右数: usize, 辺: &[(usize, usize)]) -> usize {
        let mut 隣接 = vec![Vec::new(); 左数];
        for &(u, v) in 辺 {
            隣接[u].push(v);
        }
        let mut 右相手 = vec![-1isize; 右数];
        fn 試行(
            u: usize, 隣接: &[Vec<usize>], 右相手: &mut [isize], 訪問: &mut [bool]
        ) -> bool {
            for &v in &隣接[u] {
                if !訪問[v] {
                    訪問[v] = true;
                    if 右相手[v] < 0 || 試行(右相手[v] as usize, 隣接, 右相手, 訪問)
                    {
                        右相手[v] = u as isize;
                        return true;
                    }
                }
            }
            false
        }
        let mut 本数 = 0;
        for u in 0..左数 {
            let mut 訪問 = vec![false; 右数];
            if 試行(u, &隣接, &mut 右相手, &mut 訪問) {
                本数 += 1;
            }
        }
        本数
    }

    #[test]
    fn 完全二部グラフは小さい方が答え() {
        let mut m = ホップクロフトカープ::new(3, 4);
        for u in 0..3 {
            for v in 0..4 {
                m.辺追加(u, v);
            }
        }
        assert_eq!(m.最大マッチング(), 3);
    }

    #[test]
    fn マッチングの組が整合する() {
        let mut m = ホップクロフトカープ::new(3, 3);
        m.辺追加(0, 0);
        m.辺追加(0, 1);
        m.辺追加(1, 0);
        m.辺追加(2, 2);
        let 本数 = m.最大マッチング();
        let 組 = m.マッチング();
        assert_eq!(本数, 組.len());
        // 各右頂点は高々 1 回
        let mut 右使用 = [false; 3];
        for &(_, v) in &組 {
            assert!(!右使用[v]);
            右使用[v] = true;
        }
    }

    #[test]
    fn 乱数でkuhnと一致() {
        let mut seed = 0xabcd_1234u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..100 {
            let 左 = (乱数() as usize) % 8 + 1;
            let 右 = (乱数() as usize) % 8 + 1;
            let 辺数 = (乱数() as usize) % (左 * 右 + 1);
            let mut 辺 = Vec::new();
            let mut m = ホップクロフトカープ::new(左, 右);
            for _ in 0..辺数 {
                let u = (乱数() as usize) % 左;
                let v = (乱数() as usize) % 右;
                辺.push((u, v));
                m.辺追加(u, v);
            }
            assert_eq!(m.最大マッチング(), kuhn(左, 右, &辺));
        }
    }
}
