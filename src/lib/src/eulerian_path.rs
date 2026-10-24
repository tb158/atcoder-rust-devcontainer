use cargo_snippet::snippet;

#[snippet("オイラー_路")]
#[snippet("オイラー_閉路")]
#[snippet("一筆書き")]
#[snippet("Hierholzer")]
#[snippet("有向_オイラー_路")]
/// 有向グラフのオイラー路（全ての辺をちょうど 1 回ずつ通る路）を Hierholzer 法で求める。
/// 再帰なし、O(V + E)。
///
/// `頂点数` は頂点数、`辺` は `(from, to)` の列。存在すれば通過頂点列（長さ `辺数 + 1`）を返し、
/// 存在しなければ `None`。オイラー閉路の場合は始点と終点が一致する列になる。
///
/// 存在条件: 辺を持つ頂点が連結で、各頂点の `出次数 - 入次数` が
/// 「全て 0（閉路）」または「始点だけ +1・終点だけ -1・他 0（路）」であること。
fn オイラー路_有向(頂点数: usize, 辺: &[(usize, usize)]) -> Option<Vec<usize>> {
    let m = 辺.len();
    if m == 0 {
        return None;
    }
    let mut 出辺: Vec<Vec<usize>> = vec![vec![]; 頂点数]; // 各頂点から出る辺番号
    let mut 出次数 = vec![0isize; 頂点数];
    let mut 入次数 = vec![0isize; 頂点数];
    for (i, &(u, v)) in 辺.iter().enumerate() {
        出辺[u].push(i);
        出次数[u] += 1;
        入次数[v] += 1;
    }

    // 始点の決定と次数条件の確認
    let mut 始点 = !0;
    let mut 始点候補数 = 0;
    let mut 終点候補数 = 0;
    for v in 0..頂点数 {
        match 出次数[v] - 入次数[v] {
            0 => {}
            1 => {
                始点候補数 += 1;
                始点 = v;
            }
            -1 => 終点候補数 += 1,
            _ => return None,
        }
    }
    let 閉路 = 始点候補数 == 0 && 終点候補数 == 0;
    if !(閉路 || (始点候補数 == 1 && 終点候補数 == 1)) {
        return None;
    }
    if 始点 == !0 {
        始点 = (0..頂点数).find(|&v| 出次数[v] > 0)?;
    }

    // Hierholzer 法（反復）
    let mut 次に見る = vec![0usize; 頂点数];
    let mut 探索 = vec![始点];
    let mut 経路 = Vec::with_capacity(m + 1);
    while let Some(&now) = 探索.last() {
        if 次に見る[now] < 出辺[now].len() {
            let e = 出辺[now][次に見る[now]];
            次に見る[now] += 1;
            探索.push(辺[e].1);
        } else {
            経路.push(now);
            探索.pop();
        }
    }
    経路.reverse();
    // 全辺を使えていれば連結（= オイラー路が成立）
    if 経路.len() == m + 1 {
        Some(経路)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 単純な路() {
        // 0 -> 1 -> 2
        let 経路 = オイラー路_有向(3, &[(0, 1), (1, 2)]).unwrap();
        assert_eq!(経路, vec![0, 1, 2]);
    }

    #[test]
    fn 閉路() {
        // 0 -> 1 -> 2 -> 0
        let 経路 = オイラー路_有向(3, &[(0, 1), (1, 2), (2, 0)]).unwrap();
        assert_eq!(経路.first(), 経路.last()); // 始点 = 終点
        assert_eq!(経路.len(), 4);
    }

    #[test]
    fn 多重辺を含む路() {
        // 0 ->1, 1->0, 0->1（辺3本）: 路 0-1-0-1
        let 経路 = オイラー路_有向(2, &[(0, 1), (1, 0), (0, 1)]).unwrap();
        assert_eq!(経路, vec![0, 1, 0, 1]);
    }

    #[test]
    fn 非連結は解なし() {
        // 2 つの独立した閉路。次数条件は満たすが辺を全部辿れない
        assert_eq!(オイラー路_有向(4, &[(0, 1), (1, 0), (2, 3), (3, 2)]), None);
    }

    #[test]
    fn 次数条件を満たさないと解なし() {
        // 0->1, 0->2 は出次数 2 で路にならない
        assert_eq!(オイラー路_有向(3, &[(0, 1), (0, 2)]), None);
    }
}
