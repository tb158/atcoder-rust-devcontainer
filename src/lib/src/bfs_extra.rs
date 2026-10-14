use cargo_snippet::snippet;

#[snippet("01BFS")]
#[snippet("0-1BFS")]
#[snippet("ゼロイチ_BFS")]
/// 0-1 BFS。辺の重みが 0 か 1 のグラフの単一始点最短距離を O(V+E) で求める。
///
/// `グラフ[u]` は `(隣接頂点, 重み)` の列で、重みは 0 または 1 のみ。
/// 重み 0 の辺は両端デックの先頭へ、重み 1 の辺は末尾へ入れることで距離順を保つ。
/// 到達不能な頂点は `None`。
fn zero_one_bfs(グラフ: &[Vec<(usize, usize)>], 始点: usize) -> Vec<Option<usize>> {
    use std::collections::VecDeque;
    let n = グラフ.len();
    let mut 距離: Vec<Option<usize>> = vec![None; n];
    let mut 探索: VecDeque<usize> = VecDeque::new();
    距離[始点] = Some(0);
    探索.push_back(始点);
    while let Some(now) = 探索.pop_front() {
        let d = 距離[now].unwrap();
        for &(next, w) in &グラフ[now] {
            let 新 = d + w;
            if 距離[next].is_none_or(|現| 新 < 現) {
                距離[next] = Some(新);
                if w == 0 {
                    探索.push_front(next);
                } else {
                    探索.push_back(next);
                }
            }
        }
    }
    距離
}

#[snippet("多点_BFS")]
#[snippet("複数_始点_BFS")]
#[snippet("多始点_BFS")]
/// 多点（複数始点）BFS。複数の始点から同時に幅優先探索し、各頂点について
/// 「最も近い始点からの距離（辺数）」を求める。O(V+E)。
///
/// `グラフ[u]` は隣接頂点の列。`始点群` の各頂点は距離 0 として一斉にキューへ入れる。
/// どの始点からも到達不能な頂点は `None`。
fn 多点bfs(グラフ: &[Vec<usize>], 始点群: &[usize]) -> Vec<Option<usize>> {
    use std::collections::VecDeque;
    let n = グラフ.len();
    let mut 距離: Vec<Option<usize>> = vec![None; n];
    let mut 探索: VecDeque<usize> = VecDeque::new();
    for &s in 始点群 {
        if 距離[s].is_none() {
            距離[s] = Some(0);
            探索.push_back(s);
        }
    }
    while let Some(now) = 探索.pop_front() {
        let d = 距離[now].unwrap();
        for &next in &グラフ[now] {
            if 距離[next].is_none() {
                距離[next] = Some(d + 1);
                探索.push_back(next);
            }
        }
    }
    距離
}

#[snippet("二部_彩色")]
#[snippet("二部_グラフ_判定")]
#[snippet("bipartite")]
/// 無向グラフを2色に塗り分ける（二部グラフ判定）。O(V+E)。
///
/// `グラフ[u]` は隣接頂点の列。塗り分けられたら各頂点の色（`false` / `true`）を返し、
/// 奇閉路（自己ループを含む）があれば `None`。非連結でもよく、連結成分ごとに
/// 「その成分の最小番号の頂点を `false`」として独立に塗る。
///
/// `petgraph::algo::is_bipartite_undirected` は「始点の連結成分が二部か」の `bool` だけで、
/// 塗り分けそのものは得られない。「白側の頂点を列挙」「白と黒の個数を比べる」型の問題には
/// こちらが要る。辺が後から増えていくオンライン形なら
/// [`crate::weighted_union_find`] を mod 2 のポテンシャルで使う方が向く。
fn 二部彩色(グラフ: &[Vec<usize>]) -> Option<Vec<bool>> {
    use std::collections::VecDeque;
    let n = グラフ.len();
    let mut 色: Vec<Option<bool>> = vec![None; n];
    let mut 探索 = VecDeque::new();
    for 始点 in 0..n {
        if 色[始点].is_some() {
            continue;
        }
        色[始点] = Some(false);
        探索.push_back(始点);
        while let Some(now) = 探索.pop_front() {
            let c = 色[now].unwrap();
            for &next in &グラフ[now] {
                match 色[next] {
                    None => {
                        色[next] = Some(!c);
                        探索.push_back(next);
                    }
                    // 同色の隣接（自己ループ含む）＝奇閉路
                    Some(隣) if 隣 == c => return None,
                    Some(_) => {}
                }
            }
        }
    }
    Some(色.into_iter().map(|c| c.unwrap()).collect())
}

#[snippet("二部_成分_分割", include = "二部_彩色")]
#[snippet("成分ごとの_白黒")]
/// 二部グラフを連結成分ごとに `(色 false の頂点, 色 true の頂点)` へ分ける。O(V log V + E)。
///
/// 奇閉路があれば `None`。「成分ごとに白の個数×黒の個数を足す」「成分ごとに多い方の色を選ぶ」
/// 型の数え上げ用で、[`二部彩色`] の色配列だけでは足りない「どの成分か」をまとめて返す。
/// 頂点番号は各リスト内で昇順。孤立頂点は `(vec![その頂点], vec![])` の成分になる。
fn 二部成分分割(グラフ: &[Vec<usize>]) -> Option<Vec<(Vec<usize>, Vec<usize>)>> {
    use std::collections::VecDeque;
    let 色 = 二部彩色(グラフ)?;
    let n = グラフ.len();
    let mut 到達済 = vec![false; n];
    let mut 成分列 = Vec::new();
    let mut 探索 = VecDeque::new();
    for 始点 in 0..n {
        if 到達済[始点] {
            continue;
        }
        到達済[始点] = true;
        探索.push_back(始点);
        let (mut 白, mut 黒) = (Vec::new(), Vec::new());
        while let Some(now) = 探索.pop_front() {
            if 色[now] { 黒.push(now) } else { 白.push(now) }
            for &next in &グラフ[now] {
                if !到達済[next] {
                    到達済[next] = true;
                    探索.push_back(next);
                }
            }
        }
        白.sort_unstable();
        黒.sort_unstable();
        成分列.push((白, 黒));
    }
    Some(成分列)
}

#[snippet("奇_閉路")]
#[snippet("奇数長_閉路")]
#[snippet("odd_cycle")]
/// 奇数長の閉路をひとつ見つけて頂点列で返す。O(V+E)。
///
/// 二部グラフ（＝奇閉路なし）なら `None`。返り値 `[v0, v1, …, vk]` は
/// `v0-v1-…-vk-v0` がそのまま閉路で、頂点は相異なり辺数 `k+1` は奇数。
/// 自己ループ `(u, u)` は長さ1の奇閉路として `[u]` を返す。
///
/// [`二部彩色`] が `None` を返す「証拠」が欲しいとき（奇閉路を出力させる問題、
/// 奇閉路上の頂点を使う構築）用。BFS 木の親を辿って同色辺の両端の合流点まで戻して復元する。
fn 奇閉路(グラフ: &[Vec<usize>]) -> Option<Vec<usize>> {
    use std::collections::VecDeque;
    let n = グラフ.len();
    let mut 色: Vec<Option<bool>> = vec![None; n];
    let mut prev = vec![!0; n];
    let mut 探索 = VecDeque::new();
    for 始点 in 0..n {
        if 色[始点].is_some() {
            continue;
        }
        色[始点] = Some(false);
        探索.push_back(始点);
        while let Some(now) = 探索.pop_front() {
            let c = 色[now].unwrap();
            for &next in &グラフ[now] {
                match 色[next] {
                    None => {
                        色[next] = Some(!c);
                        prev[next] = now;
                        探索.push_back(next);
                    }
                    Some(隣) if 隣 == c => {
                        if now == next {
                            return Some(vec![now]);
                        }
                        // now から根までの経路に印を付け、next 側から最初に当たる頂点が合流点
                        let mut 経路now = Vec::new();
                        let mut x = now;
                        loop {
                            経路now.push(x);
                            if prev[x] == !0 {
                                break;
                            }
                            x = prev[x];
                        }
                        let mut 位置 = vec![!0; n];
                        for (i, &x) in 経路now.iter().enumerate() {
                            位置[x] = i;
                        }
                        let mut 経路next = Vec::new();
                        let mut y = next;
                        while 位置[y] == !0 {
                            経路next.push(y);
                            y = prev[y];
                        }
                        let mut 閉路 = 経路now[..=位置[y]].to_vec();
                        閉路.extend(経路next.into_iter().rev());
                        return Some(閉路);
                    }
                    Some(_) => {}
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_one_bfsの最短距離() {
        // 0 =0= 1 =1= 2 、 0 =1= 2
        let g = vec![
            vec![(1, 0), (2, 1)],
            vec![(0, 0), (2, 1)],
            vec![(1, 1), (0, 1)],
        ];
        let d = zero_one_bfs(&g, 0);
        assert_eq!(d, vec![Some(0), Some(0), Some(1)]);
    }

    #[test]
    fn 多点bfsの最短距離() {
        // パスグラフ 0-1-2-3-4、始点 {0, 4}
        let g = vec![vec![1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3]];
        let d = 多点bfs(&g, &[0, 4]);
        assert_eq!(d, vec![Some(0), Some(1), Some(2), Some(1), Some(0)]);
    }

    // 総当たり: 2^n 通りの塗り分けを全部試して二部グラフか判定する。
    fn 総当たり二部判定(n: usize, 辺: &[(usize, usize)]) -> bool {
        (0u32..(1 << n))
            .any(|mask| 辺.iter().all(|&(u, v)| (mask >> u) & 1 != (mask >> v) & 1))
    }

    #[test]
    fn 二部彩色の基本() {
        // パスグラフ 0-1-2-3
        let パス = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        assert_eq!(二部彩色(&パス), Some(vec![false, true, false, true]));
        // 三角形（奇閉路）
        let 三角 = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        assert_eq!(二部彩色(&三角), None);
        // 自己ループ
        let 自己ループ = vec![vec![0]];
        assert_eq!(二部彩色(&自己ループ), None);
        // 非連結（0-1 と 2-3）でも成分ごとに塗る
        let 非連結 = vec![vec![1], vec![0], vec![3], vec![2]];
        assert_eq!(二部彩色(&非連結), Some(vec![false, true, false, true]));
    }

    #[test]
    fn 二部成分分割の内訳() {
        // 成分 0-1-2、孤立頂点 3、成分 4-5
        let g = vec![vec![1], vec![0, 2], vec![1], vec![], vec![5], vec![4]];
        assert_eq!(
            二部成分分割(&g),
            Some(vec![
                (vec![0, 2], vec![1]),
                (vec![3], vec![]),
                (vec![4], vec![5]),
            ])
        );
        let 三角 = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        assert_eq!(二部成分分割(&三角), None);
    }

    #[test]
    fn 奇閉路の復元() {
        let パス = vec![vec![1], vec![0, 2], vec![1]];
        assert_eq!(奇閉路(&パス), None);
        let 四角 = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert_eq!(奇閉路(&四角), None);
        let 三角 = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let mut 閉路 = 奇閉路(&三角).unwrap();
        閉路.sort_unstable();
        assert_eq!(閉路, vec![0, 1, 2]);
        assert_eq!(奇閉路(&[vec![0]]), Some(vec![0]));
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x1357_9bdfu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..400 {
            let n = (乱数() as usize % 8) + 1; // 1..=8
            let mut 辺 = Vec::new();
            for u in 0..n {
                for v in u + 1..n {
                    if 乱数() % 5 == 0 {
                        辺.push((u, v));
                    }
                }
                if 乱数() % 31 == 0 {
                    辺.push((u, u));
                }
            }
            let mut g = vec![Vec::new(); n];
            for &(u, v) in &辺 {
                g[u].push(v);
                g[v].push(u);
            }

            let 二部 = 総当たり二部判定(n, &辺);
            let 色 = 二部彩色(&g);
            assert_eq!(色.is_some(), 二部, "n={n} 辺={辺:?}");

            if let Some(色) = &色 {
                for &(u, v) in &辺 {
                    assert_ne!(色[u], 色[v], "辺={辺:?}");
                }
                // 成分分割は彩色と整合し、全頂点をちょうど1回ずつ覆う
                let 成分列 = 二部成分分割(&g).unwrap();
                let mut 所属 = vec![usize::MAX; n];
                for (i, (白, 黒)) in 成分列.iter().enumerate() {
                    assert!(!白.is_empty() || !黒.is_empty());
                    for &x in 白 {
                        assert!(!色[x]);
                        assert_eq!(所属[x], usize::MAX);
                        所属[x] = i;
                    }
                    for &x in 黒 {
                        assert!(色[x]);
                        assert_eq!(所属[x], usize::MAX);
                        所属[x] = i;
                    }
                }
                assert!(所属.iter().all(|&i| i != usize::MAX));
                for &(u, v) in &辺 {
                    assert_eq!(所属[u], 所属[v], "辺={辺:?}");
                }
            }

            // 奇閉路は「非二部のときだけ」見つかり、本当に奇数長の閉路になっている
            let 閉路 = 奇閉路(&g);
            assert_eq!(閉路.is_some(), !二部, "n={n} 辺={辺:?}");
            if let Some(閉路) = 閉路 {
                let k = 閉路.len();
                assert_eq!(k % 2, 1, "閉路={閉路:?}");
                let mut 既出 = vec![false; n];
                for &x in &閉路 {
                    assert!(!既出[x], "閉路={閉路:?}");
                    既出[x] = true;
                }
                for i in 0..k {
                    let (a, b) = (閉路[i], 閉路[(i + 1) % k]);
                    assert!(g[a].contains(&b), "閉路={閉路:?} 辺={辺:?}");
                }
            }
        }
    }
}
