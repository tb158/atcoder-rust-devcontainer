use cargo_snippet::snippet;

#[snippet("素数_関連")]
// primal::Sieve: エラトステネス系（区間ごとの篩）。素数列挙・個数など向き。素因数分解は主に factorize(n)。
// エラトステネスはO(nloglogn)で線形篩O(n)よりメモリ効率も実測の計算速度も良い
// ↓はエラトステネスではなく線形篩（SPF 表）。同じ上限の数を何度も分解する用途向き。
#[snippet]
use rustc_hash::FxHashMap; // 高速ハッシュ関数、default()で作成

#[snippet("素数_関連")]
// 線型篩により最大値までの最小素因数テーブルと素数リストを求める
// 最小素因数テーブル[対象の数] = その数の最小の素因数
#[derive(Default)]
struct 素因数分解器 {
    最小素因数テーブル: Vec<usize>,
    素数リスト: Vec<usize>,
}

#[snippet("素数_関連")]
impl 素因数分解器 {
    // コンストラクタ: 1〜最大値 までのテーブルを O(N) で作成（線形篩）
    fn new() -> Self {
        // メモリ:400MB 実行時間: 0.4sくらい (Atcoderのコードテストで確認)
        let 最大値 = 5 * 10u32.pow(7) as usize;
        let mut 最小素因数テーブル = vec![0; 最大値 + 1];
        let mut 素数リスト = Vec::new();

        for i in 2..=最大値 {
            if 最小素因数テーブル[i] == 0 {
                最小素因数テーブル[i] = i;
                素数リスト.push(i);
            }
            for &素因数 in &素数リスト {
                // ②素因数は算出する合成数の最小素因数であることを保証する
                let 合成数 = i * 素因数;
                if 合成数 > 最大値 {
                    break;
                }
                最小素因数テーブル[合成数] = 素因数;
                // ①もしiの最小素因数が素因数以上であれば、素因数を大きくしてしまうと②が保証できないのでbreak
                if 素因数 >= 最小素因数テーブル[i] {
                    break;
                }
            }
        }
        Self {
            最小素因数テーブル,
            素数リスト,
        }
    }

    fn 素因数分解(&self, mut n: usize) -> FxHashMap<usize, usize> {
        let mut 結果 = FxHashMap::default();

        while n > 1 {
            let 因数 = self.最小素因数テーブル[n];
            let mut 指数 = 0;

            // テーブルを使って、因数で割れるだけ割る
            while n % 因数 == 0 {
                n /= 因数;
                指数 += 1;
            }

            if 指数 > 0 {
                結果.insert(因数, 指数);
            }
        }
        結果
    }
}

#[snippet("区間_篩")]
#[snippet("セグメント_篩")]
#[snippet("SegmentedSieve")]
/// 区間（`l..r`=半開 / `l..=r`=閉。`..=r` のように下端省略も可）に含まれる素数をすべて昇順で返す
/// **区間篩**。`r` が `10¹²` 程度でも、区間幅 `r−l` が小さければ高速に列挙できる。計算量
/// `O((√r) + (r−l) loglog r)`、追加メモリ `O(√r + 区間幅)`。上端は必須（土台の長さが無いので
/// `..` や `l..` は解決できず panic）。
///
/// 同じファイルの [`素因数分解器`] は上限 `5·10⁷` までの線形篩なので `r` が大きいと使えない。
/// 区間篩は「`10¹²` 近辺の素数」のように上限だけ大きく幅が狭い場合に使う。
fn 区間篩<T: Copy + TryInto<u64>>(範囲: impl std::ops::RangeBounds<T>) -> Vec<usize> {
    use std::ops::Bound::{Excluded, Included, Unbounded};
    let 変換 = |x: T| -> u64 {
        x.try_into()
            .ok()
            .expect("区間篩: 引数が u64 に収まりません")
    };
    // 内部は閉区間 [l, r] で扱う。上端は必須（.. / l.. は上限が定まらず不可）。
    let l: u64 = match 範囲.start_bound() {
        Included(&x) => 変換(x),
        Excluded(&x) => 変換(x).saturating_add(1),
        Unbounded => 0,
    };
    let r: u64 = match 範囲.end_bound() {
        Included(&x) => 変換(x),
        Excluded(&x) => 変換(x).saturating_sub(1),
        Unbounded => panic!("区間篩: 上端は必須（.. や l.. は不可。l..r か l..=r を使う）"),
    };
    if r < 2 || l > r {
        return Vec::new();
    }
    let l = l.max(2);
    // √r までの基底素数。
    let lim = (r as f64).sqrt() as u64 + 2;
    let mut 合成基底 = vec![false; (lim + 1) as usize];
    let mut 基底素数 = Vec::new();
    for i in 2..=lim {
        if !合成基底[i as usize] {
            基底素数.push(i);
            let mut j = i * i;
            while j <= lim {
                合成基底[j as usize] = true;
                j += i;
            }
        }
    }
    // 区間 [l, r] の合成数を消す。
    let 幅 = (r - l + 1) as usize;
    let mut 合成 = vec![false; 幅];
    for &p in &基底素数 {
        let mut 始 = l.div_ceil(p) * p; // ⌈l/p⌉·p
        始 = 始.max(p * p);
        let mut j = 始;
        while j <= r {
            合成[(j - l) as usize] = true;
            j += p;
        }
    }
    (l..=r)
        .filter(|&x| !合成[(x - l) as usize])
        .map(|x| x as usize)
        .collect()
}

// -------------------------------------------------------------------
// テストコード
// -------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    fn 素数判定(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        let mut p = 2u64;
        while p * p <= n {
            if n % p == 0 {
                return false;
            }
            p += 1;
        }
        true
    }

    #[test]
    fn 区間篩の既知例() {
        assert_eq!(区間篩(2..=20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
        assert_eq!(区間篩(0..=1), Vec::<usize>::new());
        assert_eq!(区間篩(14..=16), Vec::<usize>::new());
        assert_eq!(区間篩(17..=17), vec![17]);
        // 半開 `l..r` は上端を含まない。
        assert_eq!(区間篩(2..19), vec![2, 3, 5, 7, 11, 13, 17]);
        assert_eq!(区間篩(14..17), Vec::<usize>::new());
        // 下端省略 `..=r` は 0 起点（実質 2 以上）。
        assert_eq!(区間篩(..=20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
        // 10^9 近辺（10^9 + 7, 10^9 + 9 は有名な素数）。
        let v = 区間篩(1_000_000_000..=1_000_000_010);
        assert!(v.contains(&1_000_000_007) && v.contains(&1_000_000_009));
    }

    #[test]
    fn 区間篩が試し割りと一致() {
        let mut seed = 0x33c7_91e0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..300 {
            let l = 乱数() % 50_000;
            let 幅 = 乱数() % 2000;
            let r = l + 幅;
            let 篩 = 区間篩(l..=r);
            let 期待: Vec<usize> = (l..=r)
                .filter(|&x| 素数判定(x))
                .map(|x| x as usize)
                .collect();
            assert_eq!(篩, 期待, "l={l} r={r}");
        }
    }

    #[test]
    fn 区間篩は大きな上限でも一致() {
        // 上限は大きいが幅が狭い区間。試し割りで全数チェック。
        let l = 1_000_000_000_000u64; // 10^12
        let r = l + 200;
        let 篩 = 区間篩(l..=r);
        let 期待: Vec<usize> = (l..=r)
            .filter(|&x| 素数判定(x))
            .map(|x| x as usize)
            .collect();
        assert_eq!(篩, 期待);
    }

    #[test]
    fn test_prime_factorization_correctness() {
        // 10^7 までの篩を作成
        let solver = 素因数分解器::new();

        // ケース1: 単純な合成数 (12 = 2^2 * 3^1)
        let res1 = solver.素因数分解(12);
        assert_eq!(res1.get(&2), Some(&2));
        assert_eq!(res1.get(&3), Some(&1));
    }
}
