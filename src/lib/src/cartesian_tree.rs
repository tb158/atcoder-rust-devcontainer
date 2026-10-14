use cargo_snippet::snippet;

#[snippet("デカルト_木")]
#[snippet("CartesianTree")]
#[snippet("カルテシアン_木")]
#[snippet("直交_木")]
/// 配列からデカルト木（Cartesian tree）を O(N) で構築する。返り値は `(親配列, 根)`。
/// 親配列の根は `-1`。値が等しいときは添字の小さい方を祖先（根側）にする最小ヒープ版。
///
/// デカルト木は「中順走査が元の配列順」かつ「親の値 <= 子の値（最小ヒープ）」を満たす二分木で、
/// 区間最小値クエリ（RMQ）を LCA に帰着したり、ヒストグラム最大長方形・単調スタック系の問題に使える。
///
/// 要素型 `T` は順序比較できる型なら何でもよい（`i64`・`f64` など `PartialOrd`）。
fn デカルト木<T: PartialOrd>(a: &[T]) -> (Vec<i64>, usize) {
    let n = a.len();
    let mut prev = vec![-1i64; n];
    let mut スタック: Vec<usize> = Vec::new();
    for i in 0..n {
        let mut 直前 = -1i64;
        // 自分より大きい要素をスタックから外す（外した最後の要素が自分の左の子になる）
        while let Some(&天) = スタック.last() {
            if a[天] <= a[i] {
                break;
            }
            直前 = 天 as i64;
            スタック.pop();
        }
        if 直前 != -1 {
            prev[直前 as usize] = i as i64;
        }
        if let Some(&天) = スタック.last() {
            prev[i] = 天 as i64;
        }
        スタック.push(i);
    }
    let 根 = (0..n).find(|&i| prev[i] == -1).unwrap_or(0);
    (prev, 根)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 総当たり: 区間の最小値（最左）を根として再帰的に親を決める。
    fn 総当たり(a: &[i64]) -> Vec<i64> {
        let n = a.len();
        let mut 親 = vec![-1i64; n];
        fn 構築(a: &[i64], l: usize, r: usize, 親元: i64, 親: &mut [i64]) {
            if l >= r {
                return;
            }
            // [l, r) の最左最小
            let mut m = l;
            for i in l + 1..r {
                if a[i] < a[m] {
                    m = i;
                }
            }
            親[m] = 親元;
            構築(a, l, m, m as i64, 親);
            構築(a, m + 1, r, m as i64, 親);
        }
        構築(a, 0, n, -1, &mut 親);
        親
    }

    // 中順走査が 0..n になるか確認する。
    fn 中順(親: &[i64], 根: usize) -> Vec<usize> {
        let n = 親.len();
        let mut 子 = vec![[usize::MAX; 2]; n];
        for (c, &p) in 親.iter().enumerate() {
            if p != -1 {
                let p = p as usize;
                // 中順のため左右を添字で割り当てる
                if c < p {
                    子[p][0] = c;
                } else {
                    子[p][1] = c;
                }
            }
        }
        let mut 順 = Vec::new();
        let mut スタック = vec![(根, false)];
        while let Some((v, 展開済)) = スタック.pop() {
            if v == usize::MAX {
                continue;
            }
            if 展開済 {
                順.push(v);
            } else {
                スタック.push((子[v][1], false));
                スタック.push((v, true));
                スタック.push((子[v][0], false));
            }
        }
        順
    }

    #[test]
    fn 小さな例() {
        let a = [3, 1, 4, 1, 5, 9, 2, 6];
        let (親, 根) = デカルト木(&a);
        assert_eq!(親, 総当たり(&a));
        assert_eq!(中順(&親, 根), (0..a.len()).collect::<Vec<_>>());
        // ヒープ性
        for (c, &p) in 親.iter().enumerate() {
            if p != -1 {
                assert!(a[p as usize] <= a[c]);
            }
        }
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x77cc_11ddu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..300 {
            let n = (乱数() as usize % 30) + 1;
            let a: Vec<i64> = (0..n).map(|_| (乱数() % 10) as i64).collect();
            let (親, 根) = デカルト木(&a);
            assert_eq!(親, 総当たり(&a), "a={a:?}");
            assert_eq!(中順(&親, 根), (0..n).collect::<Vec<_>>(), "a={a:?}");
            for (c, &p) in 親.iter().enumerate() {
                if p != -1 {
                    assert!(a[p as usize] <= a[c], "a={a:?}");
                }
            }
        }
    }
}
