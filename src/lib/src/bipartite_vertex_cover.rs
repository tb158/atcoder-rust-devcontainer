use cargo_snippet::snippet;

#[snippet("二部_最小_点_被覆")]
#[snippet("ケーニッヒ")]
#[snippet("Konig")]
#[snippet("二部_最大_独立_集合")]
#[snippet("最小_点_被覆")]
/// Kuhn 法で増加路を 1 本探す内部関数。`右相手`・`左相手` を更新する。
fn 増加(
    u: usize,
    グラフ: &[Vec<usize>],
    右相手: &mut [Option<usize>],
    左相手: &mut [Option<usize>],
    到達済: &mut [bool],
) -> bool {
    for &v in &グラフ[u] {
        if 到達済[v] {
            continue;
        }
        到達済[v] = true;
        if 右相手[v].is_none() || 増加(右相手[v].unwrap(), グラフ, 右相手, 左相手, 到達済)
        {
            右相手[v] = Some(u);
            左相手[u] = Some(v);
            return true;
        }
    }
    false
}

#[snippet("二部_最小_点_被覆")]
#[snippet("ケーニッヒ")]
#[snippet("Konig")]
#[snippet("二部_最大_独立_集合")]
#[snippet("最小_点_被覆")]
/// König の定理の核。最大マッチングを求め、未マッチ左頂点から交互路で到達できる
/// 左集合 `z左` と右集合 `z右` を返す。最小点被覆・最大独立集合の両方の土台。
fn ケーニッヒ核(
    左数: usize, 右数: usize, 辺: &[(usize, usize)]
) -> (Vec<bool>, Vec<bool>) {
    let mut グラフ = vec![Vec::new(); 左数];
    for &(u, v) in 辺 {
        グラフ[u].push(v);
    }
    let mut 右相手: Vec<Option<usize>> = vec![None; 右数];
    let mut 左相手: Vec<Option<usize>> = vec![None; 左数];
    for u in 0..左数 {
        let mut 到達済 = vec![false; 右数];
        増加(u, &グラフ, &mut 右相手, &mut 左相手, &mut 到達済);
    }
    // 未マッチ左頂点から、非マッチ辺(左→右)・マッチ辺(右→左)を交互にたどって到達集合を作る。
    let mut z左 = vec![false; 左数];
    let mut z右 = vec![false; 右数];
    let mut 探索 = Vec::new();
    for (u, &相手) in 左相手.iter().enumerate() {
        if 相手.is_none() {
            z左[u] = true;
            探索.push(u);
        }
    }
    while let Some(u) = 探索.pop() {
        for &v in &グラフ[u] {
            if 左相手[u] == Some(v) || z右[v] {
                continue; // マッチ辺は使わない／到達済み
            }
            z右[v] = true;
            if let Some(w) = 右相手[v]
                && !z左[w]
            {
                z左[w] = true;
                探索.push(w);
            }
        }
    }
    (z左, z右)
}

#[snippet("二部_最小_点_被覆")]
#[snippet("ケーニッヒ")]
#[snippet("Konig")]
#[snippet("最小_点_被覆")]
/// König の定理で二部グラフの最小点被覆を求める。`O(V·E)`（内部は Kuhn 法）。
///
/// 左頂点 `0..左数`・右頂点 `0..右数`、`辺` は `(左, 右)` の列。返り値 `(左被覆, 右被覆)` は
/// 被覆に含める左・右の頂点番号。被覆サイズ `= 最大マッチング数`（König の定理）。
/// マッチングのみの [`hopcroft_karp`](crate::hopcroft_karp) と異なり点被覆を復元する。
fn 二部最小点被覆(
    左数: usize,
    右数: usize,
    辺: &[(usize, usize)],
) -> (Vec<usize>, Vec<usize>) {
    let (z左, z右) = ケーニッヒ核(左数, 右数, 辺);
    // 最小点被覆 = (Z に入らない左) ∪ (Z に入る右)。
    let 左被覆: Vec<usize> = (0..左数).filter(|&u| !z左[u]).collect();
    let 右被覆: Vec<usize> = (0..右数).filter(|&v| z右[v]).collect();
    (左被覆, 右被覆)
}

#[snippet("二部_最大_独立_集合")]
#[snippet("ケーニッヒ")]
#[snippet("Konig")]
/// König の定理で二部グラフの最大独立集合を求める（最小点被覆の補集合）。`O(V·E)`。
///
/// 返り値 `(左独立, 右独立)`。サイズ `= 全頂点数 - 最大マッチング数`。
fn 二部最大独立集合(
    左数: usize,
    右数: usize,
    辺: &[(usize, usize)],
) -> (Vec<usize>, Vec<usize>) {
    let (z左, z右) = ケーニッヒ核(左数, 右数, 辺);
    // 最大独立集合 = (Z に入る左) ∪ (Z に入らない右)。
    let 左独立: Vec<usize> = (0..左数).filter(|&u| z左[u]).collect();
    let 右独立: Vec<usize> = (0..右数).filter(|&v| !z右[v]).collect();
    (左独立, 右独立)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 全部分集合を試して最小点被覆サイズ・最大独立集合サイズを求める。
    fn 総当たり(左数: usize, 右数: usize, 辺: &[(usize, usize)]) -> (usize, usize) {
        let n = 左数 + 右数; // 左を 0..左数、右を 左数..n に並べる
        let 辺b: Vec<(usize, usize)> = 辺.iter().map(|&(u, v)| (u, 左数 + v)).collect();
        let mut 最小被覆 = n;
        let mut 最大独立 = 0;
        for mask in 0u32..(1 << n) {
            let 入る = |x: usize| mask >> x & 1 == 1;
            let 被覆 = 辺b.iter().all(|&(u, v)| 入る(u) || 入る(v));
            let 独立 = 辺b.iter().all(|&(u, v)| !(入る(u) && 入る(v)));
            let サイズ = (mask.count_ones()) as usize;
            if 被覆 {
                最小被覆 = 最小被覆.min(サイズ);
            }
            if 独立 {
                最大独立 = 最大独立.max(サイズ);
            }
        }
        (最小被覆, 最大独立)
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0xc04e_19a0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..3000 {
            let 左数 = (乱数() as usize % 4) + 1;
            let 右数 = (乱数() as usize % 4) + 1;
            let mut 辺 = Vec::new();
            for u in 0..左数 {
                for v in 0..右数 {
                    if 乱数() % 2 == 0 {
                        辺.push((u, v));
                    }
                }
            }
            let (最小被覆, 最大独立) = 総当たり(左数, 右数, &辺);
            let (左被覆, 右被覆) = 二部最小点被覆(左数, 右数, &辺);
            // サイズ一致。
            assert_eq!(
                左被覆.len() + 右被覆.len(),
                最小被覆,
                "被覆サイズ 辺={辺:?}"
            );
            // 妥当な被覆か（全辺を被覆）。
            let lc: std::collections::HashSet<usize> = 左被覆.iter().copied().collect();
            let rc: std::collections::HashSet<usize> = 右被覆.iter().copied().collect();
            assert!(
                辺.iter().all(|&(u, v)| lc.contains(&u) || rc.contains(&v)),
                "被覆漏れ 辺={辺:?} 左={左被覆:?} 右={右被覆:?}"
            );
            let (左独立, 右独立) = 二部最大独立集合(左数, 右数, &辺);
            assert_eq!(
                左独立.len() + 右独立.len(),
                最大独立,
                "独立サイズ 辺={辺:?}"
            );
            // 妥当な独立集合か（内部に辺が無い）。
            let li: std::collections::HashSet<usize> = 左独立.iter().copied().collect();
            let ri: std::collections::HashSet<usize> = 右独立.iter().copied().collect();
            assert!(
                辺.iter()
                    .all(|&(u, v)| !(li.contains(&u) && ri.contains(&v))),
                "独立違反 辺={辺:?}"
            );
        }
    }
}
