use cargo_snippet::snippet;

#[snippet("最小_有向_全域木")]
#[snippet("有向_最小_全域木")]
#[snippet("ChuLiu")]
#[snippet("ChuLiuEdmonds")]
#[snippet("最小_全域_有向木")]
/// 最小有向全域木（最小全域内向木 / arborescence）の総コストを Chu-Liu/Edmonds 法で求める。
/// 根から全頂点へ到達する有向木のうちコスト最小のもの。O(V·E)。
///
/// `辺` は `(始点, 終点, コスト)`。根から到達できない頂点があれば `None`。
/// petgraph の `min_spanning_tree` は無向専用なので、有向版が要るときはこれを使う。
fn 最小有向全域木(頂点数: usize, 根: usize, 辺: &[(usize, usize, i64)]) -> Option<i64> {
    const 無限: i64 = i64::MAX;
    let mut n = 頂点数;
    let mut 根 = 根;
    let mut 辺: Vec<(usize, usize, i64)> = 辺.to_vec();
    let mut 合計 = 0i64;
    loop {
        // 各頂点への最小入辺
        let mut 入コスト = vec![無限; n];
        let mut 入元 = vec![!0; n];
        for &(u, v, w) in &辺 {
            if u != v && w < 入コスト[v] {
                入コスト[v] = w;
                入元[v] = u;
            }
        }
        for (v, &c) in 入コスト.iter().enumerate() {
            if v != 根 && c == 無限 {
                return None; // 到達不能
            }
        }
        // 閉路検出と縮約用の id 付け
        let mut id = vec![!0; n];
        let mut 到達済 = vec![!0; n];
        let mut 閉路数 = 0usize;
        // v は頂点 ID として入元の追跡に使うため添字ループのまま扱う
        #[allow(clippy::needless_range_loop)]
        for v in 0..n {
            if v == 根 {
                continue;
            }
            合計 += 入コスト[v];
            // 入元を辿って閉路を探す
            let mut u = v;
            while 到達済[u] != v && id[u] == !0 && u != 根 {
                到達済[u] = v;
                u = 入元[u];
            }
            if u != 根 && id[u] == !0 {
                let mut x = 入元[u];
                while x != u {
                    id[x] = 閉路数;
                    x = 入元[x];
                }
                id[u] = 閉路数;
                閉路数 += 1;
            }
        }
        if 閉路数 == 0 {
            return Some(合計); // 閉路なし＝完成
        }
        // 閉路に属さない頂点に新 id を振る
        for x in id.iter_mut() {
            if *x == !0 {
                *x = 閉路数;
                閉路数 += 1;
            }
        }
        // 縮約
        let mut 新辺 = Vec::new();
        for &(u, v, w) in &辺 {
            if id[u] != id[v] {
                新辺.push((id[u], id[v], w - 入コスト[v]));
            }
        }
        辺 = 新辺;
        n = 閉路数;
        根 = id[根];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 各非根頂点に入辺を 1 本選び、根への到達木になる組合せを総当たりして最小コストを求める
    fn 素朴(頂点数: usize, 根: usize, 辺: &[(usize, usize, i64)]) -> Option<i64> {
        // 各頂点 v != root の候補入辺一覧
        let mut 候補: Vec<Vec<(usize, i64)>> = vec![Vec::new(); 頂点数];
        for &(u, v, w) in 辺 {
            if u != v && v != 根 {
                候補[v].push((u, w));
            }
        }
        let 非根: Vec<usize> = (0..頂点数).filter(|&v| v != 根).collect();
        if 非根.iter().any(|&v| 候補[v].is_empty()) {
            return None;
        }
        let mut 最良: Option<i64> = None;
        // 各非根頂点の候補番号を全探索
        let mut 選択 = vec![0usize; 頂点数];
        loop {
            // 親配列を作り到達性を確認
            let mut 親 = vec![usize::MAX; 頂点数];
            let mut コスト = 0i64;
            for &v in &非根 {
                let (u, w) = 候補[v][選択[v]];
                親[v] = u;
                コスト += w;
            }
            // 全頂点が根へ到達できるか
            let 妥当 = (0..頂点数).all(|start| {
                let mut cur = start;
                for _ in 0..=頂点数 {
                    if cur == 根 {
                        return true;
                    }
                    if 親[cur] == usize::MAX {
                        return false;
                    }
                    cur = 親[cur];
                }
                false
            });
            if 妥当 {
                最良 = Some(最良.map_or(コスト, |b| b.min(コスト)));
            }
            // カウンタを進める
            let mut k = 0;
            loop {
                if k == 非根.len() {
                    return 最良;
                }
                let v = 非根[k];
                選択[v] += 1;
                if 選択[v] < 候補[v].len() {
                    break;
                }
                選択[v] = 0;
                k += 1;
            }
        }
    }

    #[test]
    fn 単純な例() {
        // 0 を根に 0->1(1), 0->2(2), 1->2(1) → 0->1->2 で合計 2
        let 辺 = [(0, 1, 1), (0, 2, 2), (1, 2, 1)];
        assert_eq!(最小有向全域木(3, 0, &辺), Some(2));
    }

    #[test]
    fn 閉路を含む例() {
        // 1->2->3->1 の閉路があるケース
        let 辺 = [(0, 1, 5), (1, 2, 1), (2, 3, 1), (3, 1, 1), (0, 3, 10)];
        assert_eq!(最小有向全域木(4, 0, &辺), 素朴(4, 0, &辺));
    }

    #[test]
    fn 到達不能は解なし() {
        // 頂点 2 へ入る辺が無い
        assert_eq!(最小有向全域木(3, 0, &[(0, 1, 1)]), None);
    }

    #[test]
    fn 総当たりと一致() {
        let 辺集合: Vec<Vec<(usize, usize, i64)>> = vec![
            vec![(0, 1, 3), (0, 2, 5), (1, 2, 1), (2, 1, 2)],
            vec![(0, 1, 1), (1, 2, 2), (2, 0, 3), (0, 2, 4), (2, 1, 1)],
            vec![
                (0, 1, 2),
                (0, 2, 2),
                (0, 3, 2),
                (1, 2, 1),
                (2, 3, 1),
                (3, 1, 1),
            ],
        ];
        for (k, 辺) in 辺集合.iter().enumerate() {
            let n = 辺.iter().flat_map(|&(u, v, _)| [u, v]).max().unwrap() + 1;
            assert_eq!(最小有向全域木(n, 0, 辺), 素朴(n, 0, 辺), "ケース {k}");
        }
    }
}
