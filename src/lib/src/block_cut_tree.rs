use cargo_snippet::snippet;

#[snippet("二重_頂点_連結_成分")]
#[snippet("ブロック_カット_木")]
#[snippet("BlockCutTree")]
#[snippet("BCC")]
#[snippet("二点_連結_成分_分解")]
/// 二重頂点連結成分（BCC, block）分解と block-cut 木。`lowlink`（橋・関節点の列挙のみ）とは別で、
/// 辺をブロックに分割し、関節点とブロックを頂点とする木（非連結なら森）を構成する。
///
/// - `成分[i]`: i 番目の BCC に属する元グラフ頂点の一覧（関節点は複数の BCC に現れる）。
/// - `木`: block-cut 木の隣接リスト（木頂点空間）。前半 `成分数` 個が BCC、後半が関節点。
/// - `関節点番号[v]`: 元頂点 `v` が関節点なら対応する木頂点番号（`>= 成分数`）、そうでなければ `None`。
/// - `所属[v]`: 元頂点 `v` が属する BCC 番号の一覧。
///
/// 反復 DFS なので深いグラフでもスタック安全。
struct ブロックカット木 {
    /// 各 BCC の頂点集合。
    成分: Vec<Vec<usize>>,
    /// BCC の個数。
    成分数: usize,
    /// block-cut 木の隣接リスト（木頂点番号空間、頂点数 = `木頂点数`）。
    木: Vec<Vec<usize>>,
    /// block-cut 木の頂点数（= `成分数` + 関節点数）。
    木頂点数: usize,
    /// 元頂点 → 関節点に対応する木頂点番号（非関節点は `None`）。
    関節点番号: Vec<Option<usize>>,
    /// 元頂点 → 属する BCC 番号の一覧。
    所属: Vec<Vec<usize>>,
}

#[snippet("二重_頂点_連結_成分")]
#[snippet("ブロック_カット_木")]
#[snippet("BlockCutTree")]
#[snippet("BCC")]
#[snippet("二点_連結_成分_分解")]
impl ブロックカット木 {
    /// 関節点（複数の BCC に属する頂点）の一覧を昇順で返す。
    fn 関節点一覧(&self) -> Vec<usize> {
        (0..self.関節点番号.len())
            .filter(|&v| self.関節点番号[v].is_some())
            .collect()
    }
}

#[snippet("二重_頂点_連結_成分")]
#[snippet("ブロック_カット_木")]
#[snippet("BlockCutTree")]
#[snippet("BCC")]
#[snippet("二点_連結_成分_分解")]
/// 無向グラフを二重頂点連結成分に分解し block-cut 木を構成する。`辺` は多重辺・自己ループを含んでよい
/// （自己ループは無視、多重辺は同じ 2 頂点の BCC を成す）。計算量 `O(n + 辺数)`。
fn ブロックカット木分解(
    n: usize, 辺: &[(usize, usize)]
) -> ブロックカット木 {
    let mut グラフ: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n]; // (相手, 辺番号)
    for (i, &(u, v)) in 辺.iter().enumerate() {
        if u != v {
            グラフ[u].push((v, i));
            グラフ[v].push((u, i));
        }
    }
    let mut ord = vec![-1isize; n];
    let mut low = vec![0isize; n];
    let mut 時刻 = 0isize;
    let mut 辺スタック: Vec<(usize, usize)> = Vec::new();
    let mut 成分: Vec<Vec<usize>> = Vec::new();

    for s in 0..n {
        if ord[s] != -1 {
            continue;
        }
        if グラフ[s].is_empty() {
            成分.push(vec![s]); // 孤立頂点は単独ブロック
            ord[s] = 時刻;
            時刻 += 1;
            continue;
        }
        ord[s] = 時刻;
        low[s] = 時刻;
        時刻 += 1;
        // (頂点, 親辺番号, 隣接走査位置)
        let mut 探索: Vec<(usize, isize, usize)> = vec![(s, -1, 0)];
        while let Some(&(now, 親辺, 位置)) = 探索.last() {
            if 位置 < グラフ[now].len() {
                探索.last_mut().unwrap().2 += 1;
                let (next, eid) = グラフ[now][位置];
                if eid as isize == 親辺 {
                    continue; // 親辺だけ 1 回スキップ（多重辺は別 id なので残る）
                }
                if ord[next] == -1 {
                    ord[next] = 時刻;
                    low[next] = 時刻;
                    時刻 += 1;
                    辺スタック.push((now, next)); // 木辺
                    探索.push((next, eid as isize, 0));
                } else if ord[next] < ord[now] {
                    辺スタック.push((now, next)); // 後退辺（祖先方向、1 回だけ積む）
                    low[now] = low[now].min(ord[next]);
                }
            } else {
                探索.pop();
                if let Some(&(prev, _, _)) = 探索.last() {
                    low[prev] = low[prev].min(low[now]);
                    if low[now] >= ord[prev] {
                        // prev は関節点（または根）。辺 (prev, now) まで取り出して 1 つの BCC とする。
                        let mut 頂点 = Vec::new();
                        loop {
                            let (x, y) = 辺スタック.pop().unwrap();
                            頂点.push(x);
                            頂点.push(y);
                            if (x, y) == (prev, now) {
                                break;
                            }
                        }
                        頂点.sort_unstable();
                        頂点.dedup();
                        成分.push(頂点);
                    }
                }
            }
        }
    }

    let 成分数 = 成分.len();
    // 出現回数 >= 2 の頂点が関節点。
    let mut 所属: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, c) in 成分.iter().enumerate() {
        for &v in c {
            所属[v].push(i);
        }
    }
    let mut 関節点番号 = vec![None; n];
    let mut 木頂点数 = 成分数;
    for (v, 番号) in 関節点番号.iter_mut().enumerate() {
        if 所属[v].len() >= 2 {
            *番号 = Some(木頂点数);
            木頂点数 += 1;
        }
    }
    // block-cut 木の辺: 各 BCC とそれに含まれる関節点を結ぶ。
    let mut 木: Vec<Vec<usize>> = vec![Vec::new(); 木頂点数];
    for (i, c) in 成分.iter().enumerate() {
        for &v in c {
            if let Some(a) = 関節点番号[v] {
                木[i].push(a);
                木[a].push(i);
            }
        }
    }

    ブロックカット木 {
        成分,
        成分数,
        木,
        木頂点数,
        関節点番号,
        所属,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    // x を除いたグラフでの連結成分数。
    fn 連結成分数(n: usize, 辺: &[(usize, usize)], 除外: usize) -> usize {
        let mut 親: Vec<usize> = (0..n).collect();
        fn 根(親: &mut [usize], x: usize) -> usize {
            if 親[x] == x {
                x
            } else {
                let r = 根(親, 親[x]);
                親[x] = r;
                r
            }
        }
        for &(u, v) in 辺 {
            if u != v && u != 除外 && v != 除外 {
                let (ru, rv) = (根(&mut 親, u), 根(&mut 親, v));
                親[ru] = rv;
            }
        }
        (0..n)
            .filter(|&x| x != 除外)
            .map(|x| 根(&mut 親, x))
            .collect::<BTreeSet<_>>()
            .len()
    }

    // 素朴な関節点判定（頂点除去で連結成分が増えるか。孤立頂点や葉は対象外の通常定義）。
    fn 素朴関節点(n: usize, 辺: &[(usize, usize)]) -> BTreeSet<usize> {
        // x を含む全頂点での連結成分数 C と、x を除いた残り n-1 頂点での連結成分数 C'。
        // x の所属成分が k 個に割れると C' = (C-1)+k。関節点 ⇔ k>=2 ⇔ C' > C。
        let 基準 = 連結成分数(n, 辺, usize::MAX);
        (0..n).filter(|&x| 連結成分数(n, 辺, x) > 基準).collect()
    }

    // 頂点集合 sub の誘導部分グラフが 2 重頂点連結か（|sub|<=2 は真）。
    fn 誘導2重連結(sub: &[usize], 辺: &[(usize, usize)]) -> bool {
        if sub.len() <= 2 {
            return true;
        }
        let 集合: BTreeSet<usize> = sub.iter().copied().collect();
        let 内辺: Vec<(usize, usize)> = 辺
            .iter()
            .copied()
            .filter(|&(u, v)| u != v && 集合.contains(&u) && 集合.contains(&v))
            .collect();
        // どの 1 頂点を除いても連結（= 関節点なし）かを確認。
        for &除外 in sub {
            let 残り: Vec<usize> = sub.iter().copied().filter(|&x| x != 除外).collect();
            // 残り頂点だけで連結か。
            let mut 親: std::collections::HashMap<usize, usize> =
                残り.iter().map(|&x| (x, x)).collect();
            fn 根(親: &mut std::collections::HashMap<usize, usize>, x: usize) -> usize {
                let p = 親[&x];
                if p == x {
                    x
                } else {
                    let r = 根(親, p);
                    親.insert(x, r);
                    r
                }
            }
            for &(u, v) in &内辺 {
                if u != 除外 && v != 除外 {
                    let (ru, rv) = (根(&mut 親, u), 根(&mut 親, v));
                    親.insert(ru, rv);
                }
            }
            let 連結数 = 残り
                .iter()
                .map(|&x| 根(&mut 親, x))
                .collect::<BTreeSet<_>>()
                .len();
            if 連結数 > 1 {
                return false; // 除外が関節点 → 誘導部分グラフは 2 重連結でない
            }
        }
        true
    }

    #[test]
    fn 小さな例() {
        // 0-1-2 と 2-3, 三角形 0-1-2 ではなく直線 + 枝。
        // 辺: (0,1),(1,2),(2,0) 三角形 + (2,3) 橋
        let 辺 = vec![(0, 1), (1, 2), (2, 0), (2, 3)];
        let r = ブロックカット木分解(4, &辺);
        assert_eq!(r.成分数, 2); // 三角形{0,1,2} と 橋{2,3}
        assert_eq!(r.関節点一覧(), vec![2]); // 2 が関節点
    }

    #[test]
    fn 乱数で不変条件を検証() {
        let mut seed = 0xb10c_c074u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..400 {
            let n = (乱数() as usize % 8) + 1;
            let 本数 = 乱数() as usize % (n * 2 + 1);
            let 辺: Vec<(usize, usize)> = (0..本数)
                .map(|_| (乱数() as usize % n, 乱数() as usize % n))
                .collect();
            let r = ブロックカット木分解(n, &辺);

            // (A) 関節点が素朴判定と一致。
            let 得関節点: BTreeSet<usize> = r.関節点一覧().into_iter().collect();
            assert_eq!(得関節点, 素朴関節点(n, &辺), "関節点 n={n} 辺={辺:?}");

            // (D) 各 BCC は誘導 2 重連結。
            for c in &r.成分 {
                assert!(誘導2重連結(c, &辺), "BCC非2重連結 {c:?} 辺={辺:?}");
            }

            // (F) 相異なる BCC は高々 1 頂点しか共有しない。
            for i in 0..r.成分数 {
                for j in (i + 1)..r.成分数 {
                    let a: BTreeSet<usize> = r.成分[i].iter().copied().collect();
                    let 共有 = r.成分[j].iter().filter(|v| a.contains(v)).count();
                    assert!(共有 <= 1, "BCC共有>1 i={i} j={j} 辺={辺:?}");
                }
            }

            // (C') 各非ループ辺は、両端を含む BCC をちょうど 1 つ持つ。
            for &(u, v) in &辺 {
                if u == v {
                    continue;
                }
                let cnt = r
                    .成分
                    .iter()
                    .filter(|c| c.contains(&u) && c.contains(&v))
                    .count();
                assert_eq!(cnt, 1, "辺({u},{v}) の所属BCC数={cnt} 辺={辺:?}");
            }

            // (B) block-cut 木は森（辺数 = 頂点数 - 連結成分数）。
            let 木辺数: usize = r.木.iter().map(|a| a.len()).sum::<usize>() / 2;
            // 木の連結成分数を数える。
            let mut 訪問 = vec![false; r.木頂点数];
            let mut 連結 = 0;
            for s in 0..r.木頂点数 {
                if 訪問[s] {
                    continue;
                }
                連結 += 1;
                let mut st = vec![s];
                訪問[s] = true;
                while let Some(x) = st.pop() {
                    for &y in &r.木[x] {
                        if !訪問[y] {
                            訪問[y] = true;
                            st.push(y);
                        }
                    }
                }
            }
            assert_eq!(木辺数, r.木頂点数 - 連結, "森性 辺={辺:?}");
        }
    }
}
