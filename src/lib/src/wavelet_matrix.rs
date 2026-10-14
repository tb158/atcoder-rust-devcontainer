use crate::range_bounds::半開;
use crate::to_int::to_u64;
use cargo_snippet::snippet;

#[snippet("ウェーブレット_行列", include = "半開_区間")]
#[snippet("WaveletMatrix")]
#[snippet("区間_k_番目")]
#[snippet("区間_第_k_小")]
#[snippet("区間内_個数")]
/// rank 補助付きビットベクトル（ウェーブレット行列の各層）。64 ビットブロック + 累積 popcount。
struct ビットベクトル {
    bits: Vec<u64>,
    累積: Vec<u32>,
}

#[snippet("ウェーブレット_行列")]
#[snippet("WaveletMatrix")]
#[snippet("区間_k_番目")]
#[snippet("区間_第_k_小")]
#[snippet("区間内_個数")]
impl ビットベクトル {
    fn new(n: usize) -> Self {
        let ブロック数 = n / 64 + 1;
        Self {
            bits: vec![0; ブロック数],
            累積: vec![0; ブロック数 + 1],
        }
    }

    fn 立てる(&mut self, i: usize) {
        self.bits[i / 64] |= 1 << (i % 64);
    }

    fn 構築(&mut self) {
        for i in 0..self.bits.len() {
            self.累積[i + 1] = self.累積[i] + self.bits[i].count_ones();
        }
    }

    /// `[0, i)` に含まれる 1 の個数。
    fn rank1(&self, i: usize) -> usize {
        let ブロック = i / 64;
        let 余り = i % 64;
        let マスク = if 余り == 0 {
            0
        } else {
            self.bits[ブロック] & ((1u64 << 余り) - 1)
        };
        self.累積[ブロック] as usize + マスク.count_ones() as usize
    }

    /// `[0, i)` に含まれる 0 の個数。
    fn rank0(&self, i: usize) -> usize {
        i - self.rank1(i)
    }
}

#[snippet("ウェーブレット_行列")]
#[snippet("WaveletMatrix")]
#[snippet("区間_k_番目")]
#[snippet("区間_第_k_小")]
#[snippet("区間内_個数")]
/// ウェーブレット行列。非負整数列に対し「位置区間 `[l, r)` の第 k 小」「位置区間 × 値域の個数」を
/// それぞれ O(ビット長) で答える静的構造。構築 O(N·ビット長)。
///
/// 位置・値域はいずれも `a..b` / `a..=b` / `..b` / `a..` / `..` を受け取れる（`RangeBounds`）。
/// 値域が広くても座標圧縮なしに `u64` のまま扱える（ビット長は最大値で決まる）。
/// 同種の用途（区間第 k 小・順位）を提供するクレートは AtCoder 標準セットにないため自作する。
struct ウェーブレット行列 {
    n: usize,
    ビット数: usize,
    層: Vec<ビットベクトル>,
    ゼロ数: Vec<usize>,
}

#[snippet("ウェーブレット_行列", include = "to_u64")]
#[snippet("WaveletMatrix")]
#[snippet("区間_k_番目")]
#[snippet("区間_第_k_小")]
#[snippet("区間内_個数")]
impl ウェーブレット行列 {
    /// 非負整数列から構築する。
    fn new(配列: &[impl TryInto<u64> + Copy]) -> Self {
        let 配列: Vec<u64> = 配列
            .iter()
            .map(|&x| {
                x.try_into()
                    .ok()
                    .expect("WaveletMatrix: 引数が u64 に収まりません")
            })
            .collect();
        let n = 配列.len();
        let 最大 = 配列.iter().copied().max().unwrap_or(0);
        let ビット数 = (64 - 最大.leading_zeros() as usize).max(1);
        let mut 現在 = 配列.to_vec();
        let mut 層 = Vec::with_capacity(ビット数);
        let mut ゼロ数 = Vec::with_capacity(ビット数);
        // 上位ビットから順に安定分割
        for b in (0..ビット数).rev() {
            let mut bv = ビットベクトル::new(n);
            for (i, &x) in 現在.iter().enumerate() {
                if x >> b & 1 == 1 {
                    bv.立てる(i);
                }
            }
            bv.構築();
            let mut ゼロ群 = Vec::with_capacity(n);
            let mut 一群 = Vec::new();
            for &x in &現在 {
                if x >> b & 1 == 1 {
                    一群.push(x);
                } else {
                    ゼロ群.push(x);
                }
            }
            ゼロ数.push(ゼロ群.len());
            ゼロ群.extend(一群);
            現在 = ゼロ群;
            層.push(bv);
        }
        Self {
            n,
            ビット数,
            層,
            ゼロ数,
        }
    }

    /// 位置範囲 `位置`（`a..b` / `a..=b` / `..` など）で `k` 番目（0-indexed、昇順）に
    /// 小さい値。`k < 範囲の長さ` であること。
    fn 区間k番目(&self, 位置: impl std::ops::RangeBounds<usize>, mut k: usize) -> usize {
        let (mut l, mut r) = 半開(位置, self.n);
        assert!(l <= r && r <= self.n && k < r - l, "区間k番目: 範囲外");
        let mut 結果 = 0u64;
        for d in 0..self.ビット数 {
            let bv = &self.層[d];
            let l0 = bv.rank0(l);
            let r0 = bv.rank0(r);
            let ゼロ個 = r0 - l0;
            if k < ゼロ個 {
                l = l0;
                r = r0;
            } else {
                結果 |= 1 << (self.ビット数 - 1 - d);
                k -= ゼロ個;
                let z = self.ゼロ数[d];
                l = z + (l - l0);
                r = z + (r - r0);
            }
        }
        結果 as usize
    }

    /// 位置 `[l, r)` の中で値が `x` 未満である要素の個数（内部プリミティブ）。
    fn 未満数(&self, mut l: usize, mut r: usize, x: u64) -> usize {
        if x >> self.ビット数 != 0 {
            return r - l; // x が値域上限を超える → 全て未満
        }
        let mut 個数 = 0usize;
        for d in 0..self.ビット数 {
            let bv = &self.層[d];
            let l0 = bv.rank0(l);
            let r0 = bv.rank0(r);
            if x >> (self.ビット数 - 1 - d) & 1 == 1 {
                // このビットが 1 → 0 側（このビットが 0 の要素）は全て x 未満
                個数 += r0 - l0;
                let z = self.ゼロ数[d];
                l = z + (l - l0);
                r = z + (r - r0);
            } else {
                l = l0;
                r = r0;
            }
        }
        個数
    }

    /// 位置範囲 `位置` にある要素のうち、値が `値域` に入るものの個数。
    ///
    /// 位置は `a..b` / `a..=b` / `..` など、値域は `a..b` / `a..=b` / `..b`（＝旧「未満の個数」）/
    /// `a..` / `..` などを受け取れる。値域の端点は `i64`・`usize` など `u64` へ変換できる整数型で
    /// よい（負や超過は panic）。
    fn 区間内個数<T: Copy + TryInto<u64>>(
        &self,
        位置: impl std::ops::RangeBounds<usize>,
        値域: impl std::ops::RangeBounds<T>,
    ) -> usize {
        use std::ops::Bound::{Excluded, Included, Unbounded};
        let (l, r) = 半開(位置, self.n);
        assert!(l <= r && r <= self.n, "区間内個数: 範囲外");
        // 「値 < 上限」の個数から「値 < 下限」の個数を引く（＝値域内の個数）。
        let 上限個数 = match 値域.end_bound() {
            Included(&x) => self.未満数(l, r, to_u64(x).saturating_add(1)),
            Excluded(&x) => self.未満数(l, r, to_u64(x)),
            Unbounded => r - l,
        };
        let 下限個数 = match 値域.start_bound() {
            Included(&x) => self.未満数(l, r, to_u64(x)),
            Excluded(&x) => self.未満数(l, r, to_u64(x).saturating_add(1)),
            Unbounded => 0,
        };
        上限個数.saturating_sub(下限個数)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 素朴k番目(a: &[u64], l: usize, r: usize, k: usize) -> u64 {
        let mut v: Vec<u64> = a[l..r].to_vec();
        v.sort_unstable();
        v[k]
    }

    #[test]
    fn 区間第k小が総当たりと一致() {
        let a = vec![5u64, 4, 1, 9, 2, 8, 0, 3, 7, 6, 2, 2];
        let wm = ウェーブレット行列::new(&a);
        let n = a.len();
        for l in 0..n {
            for r in l + 1..=n {
                for k in 0..r - l {
                    assert_eq!(
                        wm.区間k番目(l..r, k),
                        素朴k番目(&a, l, r, k) as usize,
                        "l={l}, r={r}, k={k}"
                    );
                }
            }
        }
    }

    #[test]
    fn 区間内個数が総当たりと一致() {
        let a = vec![5u64, 4, 1, 9, 2, 8, 0, 3, 7, 6];
        let wm = ウェーブレット行列::new(&a);
        let n = a.len();
        for l in 0..n {
            for r in l + 1..=n {
                // 半開・閉・下端開放など、あらゆる値域を総当たりで検証する。
                for lo in 0..=11u64 {
                    for hi in lo..=11u64 {
                        let 期待半開 = a[l..r].iter().filter(|&&v| lo <= v && v < hi).count();
                        assert_eq!(
                            wm.区間内個数(l..r, lo..hi),
                            期待半開,
                            "[{l},{r}) {lo}..{hi}"
                        );
                        let 期待閉 = a[l..r].iter().filter(|&&v| lo <= v && v <= hi).count();
                        assert_eq!(
                            wm.区間内個数(l..r, lo..=hi),
                            期待閉,
                            "[{l},{r}) {lo}..={hi}"
                        );
                    }
                    // `..hi`（＝旧「未満の個数」）と `lo..`（下端以上）も確認。
                    let 期待未満 = a[l..r].iter().filter(|&&v| v < lo).count();
                    assert_eq!(wm.区間内個数(l..r, ..lo), 期待未満, "[{l},{r}) ..{lo}");
                    let 期待以上 = a[l..r].iter().filter(|&&v| v >= lo).count();
                    assert_eq!(wm.区間内個数(l..r, lo..), 期待以上, "[{l},{r}) {lo}..");
                }
            }
        }
    }

    #[test]
    fn 位置範囲の各種指定が効く() {
        let a = vec![5u64, 4, 1, 9, 2, 8, 0, 3, 7, 6];
        let wm = ウェーブレット行列::new(&a);
        // 位置 `..`（全体）× 値域 `..`（全体）＝ 全要素数。
        assert_eq!(wm.区間内個数(.., ..1_000i64), a.len());
        // `l..=r`（閉）と `l..r+1`（半開）は一致する。
        assert_eq!(
            wm.区間内個数(2..=5, 0i64..10),
            wm.区間内個数(2..6, 0i64..10)
        );
        // k 番目でも位置 `..` を受け取れる。
        assert_eq!(wm.区間k番目(.., 0), 0); // 全体の最小
    }

    #[test]
    fn 全要素同一でも動く() {
        let a = vec![3u64, 3, 3, 3];
        let wm = ウェーブレット行列::new(&a);
        assert_eq!(wm.区間k番目(0..4, 2), 3);
        assert_eq!(wm.区間内個数(0..4, ..3), 0);
        assert_eq!(wm.区間内個数(0..4, ..4), 4);
        assert_eq!(wm.区間内個数(0..4, 3..=3), 4);
    }
}
