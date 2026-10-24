use crate::range_bounds::半開;
use cargo_snippet::snippet;

#[snippet("区間_最頻値", include = "半開_区間")]
#[snippet("RangeMode")]
/// 静的配列の区間最頻値（最も多く出現する値とその回数）を `O(√N)` で答えるデータ構造。
///
/// 平方分割。構築 `O(N√N)`、1 クエリ `O(√N log N)`。
///
/// 区間最頻値は和や最小と違って差分が取れず、セグ木では扱えない。Mo でもオフラインで解けるが、
/// 本構造はオンライン（クエリを即時に答える）。[`ウェーブレット行列`](crate::wavelet_matrix)の
/// 区間 k 番目とも別問題。
/// 値の型 `T` は座標圧縮のため順序比較できる型なら何でもよい（`isize`・`&str` など `Ord`）。
struct 区間最頻値<T> {
    元: Vec<usize>,                         // 座標圧縮後の値の列
    値表: Vec<T>,                           // 圧縮値 → 元の値
    位置: Vec<Vec<usize>>,                  // 圧縮値 → 出現位置（昇順）
    幅: usize,                              // ブロック幅
    ブロック最頻: Vec<Vec<(usize, usize)>>, // [bi][bj] = (回数, 圧縮値)
}

#[snippet("区間_最頻値")]
#[snippet("RangeMode")]
impl<T: Ord + Copy> 区間最頻値<T> {
    /// 配列から構築する。`O(N√N)`。
    fn new(配列: &[T]) -> Self {
        let n = 配列.len();
        // 座標圧縮。
        let mut 値表: Vec<T> = 配列.to_vec();
        値表.sort_unstable();
        値表.dedup();
        let 元: Vec<usize> = 配列
            .iter()
            .map(|x| 値表.partition_point(|&y| y < *x))
            .collect();
        let mut 位置 = vec![Vec::new(); 値表.len()];
        for (i, &v) in 元.iter().enumerate() {
            位置[v].push(i);
        }
        let 幅 = ((n as f64).sqrt() as usize).max(1);
        let ブロック数 = n.div_ceil(幅);
        // ブロック区間ごとの最頻値を前計算。
        let mut ブロック最頻 = vec![vec![(0usize, 0usize); ブロック数]; ブロック数];
        let mut 個数 = vec![0usize; 値表.len()];
        for (bi, 行) in ブロック最頻.iter_mut().enumerate() {
            個数.fill(0);
            let mut 現数 = 0usize;
            let mut 現値 = 0usize;
            for (bj, セル) in 行.iter_mut().enumerate().skip(bi) {
                let lo = bj * 幅;
                let hi = ((bj + 1) * 幅).min(n);
                for &v in &元[lo..hi] {
                    個数[v] += 1;
                    if 個数[v] > 現数 {
                        現数 = 個数[v];
                        現値 = v;
                    }
                }
                *セル = (現数, 現値);
            }
        }
        区間最頻値 {
            元,
            値表,
            位置,
            幅,
            ブロック最頻,
        }
    }

    /// 圧縮値 `v` が `[l, r]`（両端含む）に出現する回数。`O(log N)`。
    fn 区間出現数(&self, v: usize, l: usize, r: usize) -> usize {
        let p = &self.位置[v];
        let lo = p.partition_point(|&x| x < l);
        let hi = p.partition_point(|&x| x <= r);
        hi - lo
    }

    /// 区間（`a..b` / `a..=b` / `..` など、0-indexed）の最頻値 `(値, 出現回数)` を返す。`O(√N)`。
    /// 空区間は問い合わせ不可。
    ///
    /// 同点の値が複数あるときはそのうちの 1 つを返す（回数は最大値で一意）。
    fn クエリ(&self, 範囲: impl std::ops::RangeBounds<usize>) -> (T, usize) {
        let (l, 半開右) = 半開(範囲, self.元.len());
        assert!(l < 半開右, "区間最頻値: 空区間は問い合わせ不可");
        let r = 半開右 - 1; // 内部は閉区間 [l, r] で扱う
        let bl = l / self.幅;
        let br = r / self.幅;
        if bl == br {
            // 同一ブロック内は素朴に数える。
            let mut 最良数 = 0usize;
            let mut 最良値 = 0usize;
            for &v in &self.元[l..=r] {
                let c = self.区間出現数(v, l, r);
                if c > 最良数 {
                    最良数 = c;
                    最良値 = v;
                }
            }
            return (self.値表[最良値], 最良数);
        }
        // 完全に含まれるブロック [bl+1, br-1] があれば起点にする。
        let (mut 最良数, mut 最良値) = if bl + 2 <= br {
            self.ブロック最頻[bl + 1][br - 1]
        } else {
            (0usize, 0usize)
        };
        // 左の半端 [l, (bl+1)*幅) と右の半端 [br*幅, r] の各要素を候補に。
        let 左終 = ((bl + 1) * self.幅).min(self.元.len());
        for &v in &self.元[l..左終] {
            let c = self.区間出現数(v, l, r);
            if c > 最良数 {
                最良数 = c;
                最良値 = v;
            }
        }
        for &v in &self.元[br * self.幅..=r] {
            let c = self.区間出現数(v, l, r);
            if c > 最良数 {
                最良数 = c;
                最良値 = v;
            }
        }
        (self.値表[最良値], 最良数)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // 区間 [l, r] の最頻出現回数を素朴に求める。
    fn 素朴最頻数(配列: &[isize], l: usize, r: usize) -> usize {
        let mut 数 = HashMap::new();
        let mut 最大 = 0;
        for &x in &配列[l..=r] {
            let e = 数.entry(x).or_insert(0);
            *e += 1;
            最大 = 最大.max(*e);
        }
        最大
    }

    #[test]
    fn 既知例() {
        let a = vec![1, 2, 2, 3, 3, 3, 1, 1];
        let rm = 区間最頻値::new(&a);
        // 全区間: 3 が 3 回、1 が 3 回 → 回数 3。
        assert_eq!(rm.クエリ(0..=7).1, 3);
        // [0,2]: 2 が 2 回。
        let (v, c) = rm.クエリ(0..=2);
        assert_eq!((v, c), (2, 2));
        // [3,5]: 3 が 3 回。
        assert_eq!(rm.クエリ(3..=5), (3, 3));
        // 単一要素。
        assert_eq!(rm.クエリ(0..=0), (1, 1));
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x6a3d_90c0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..400 {
            let n = (乱数() as usize % 60) + 1;
            let 値域 = (乱数() as usize % 8) + 1;
            let a: Vec<isize> = (0..n).map(|_| (乱数() as isize) % 値域 as isize).collect();
            let rm = 区間最頻値::new(&a);
            for _ in 0..40 {
                let x = 乱数() as usize % n;
                let y = 乱数() as usize % n;
                let (l, r) = (x.min(y), x.max(y));
                let (値, 回数) = rm.クエリ(l..=r);
                let 期待 = 素朴最頻数(&a, l, r);
                assert_eq!(回数, 期待, "a={a:?} l={l} r={r}");
                // 返した値が実際にその回数だけ出現することも確認。
                let 実数 = a[l..=r].iter().filter(|&&z| z == 値).count();
                assert_eq!(実数, 回数, "返した値 {値} の出現数が回数と不一致");
            }
        }
    }
}
