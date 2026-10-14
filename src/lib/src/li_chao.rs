use crate::to_int::to_i64;
use cargo_snippet::snippet;

#[snippet("李超木")]
#[snippet("LiChaoTree")]
#[snippet("LiChao")]
#[snippet("直線_集合_最小_任意順")]
#[snippet("最小値_直線_追加")]
/// Li Chao 木（最小クエリ版）。直線 `y = 傾き·x + 切片` を **任意の順序** で追加でき、
/// あらかじめ与えたクエリ座標群の各点での最小値を O(log) で求める。
///
/// CHT（[`crate::convex_hull_trick`]）と違い傾きの単調性を要求しない。
/// クエリ座標は `new` に渡したものに限る。最大値が欲しいときは傾き・切片の符号を
/// 反転して入れ、結果を反転する（この実装は直線のみ、線分追加は非対応）。
/// 評価は i64 素のまま行うので `傾き·x + 切片` が i64 に収まる範囲で使うこと。
struct 李超木 {
    xs: Vec<i64>,
    直線: Vec<Option<(i64, i64)>>,
}

#[snippet("李超木", include = "to_i64")]
#[snippet("LiChaoTree")]
#[snippet("LiChao")]
#[snippet("直線_集合_最小_任意順")]
#[snippet("最小値_直線_追加")]
impl 李超木 {
    /// クエリで使う x 座標を全て渡して構築する（重複・未ソートで可）。
    /// 座標は任意の整数型の Vec / イテレータでよい（各メソッドの引数も同様）。
    fn new<I>(座標群: I) -> Self
    where
        I: IntoIterator,
        I::Item: TryInto<i64>,
    {
        let mut 座標群: Vec<i64> = 座標群.into_iter().map(to_i64).collect();
        座標群.sort_unstable();
        座標群.dedup();
        if 座標群.is_empty() {
            座標群.push(0);
        }
        let n = 座標群.len();
        Self {
            xs: 座標群,
            直線: vec![None; 4 * n],
        }
    }

    fn 評価(直線: (i64, i64), x: i64) -> i64 {
        直線.0 * x + 直線.1
    }

    /// 直線 `y = 傾き·x + 切片` を追加する。
    fn 直線追加(&mut self, 傾き: impl TryInto<i64>, 切片: impl TryInto<i64>) {
        let n = self.xs.len();
        self.追加内部(1, 0, n, (to_i64(傾き), to_i64(切片)));
    }

    fn 追加内部(&mut self, ノード: usize, lo: usize, hi: usize, mut 新規: (i64, i64)) {
        let m = (lo + hi) / 2;
        let Some(mut 現在) = self.直線[ノード] else {
            self.直線[ノード] = Some(新規);
            return;
        };
        // 中点で下にある方をこのノードに残す
        if Self::評価(新規, self.xs[m]) < Self::評価(現在, self.xs[m]) {
            std::mem::swap(&mut 新規, &mut 現在);
            self.直線[ノード] = Some(現在);
        }
        if hi - lo == 1 {
            return;
        }
        // 負けた直線（新規）がまだ下に来られる側へ降ろす
        if Self::評価(新規, self.xs[lo]) < Self::評価(現在, self.xs[lo]) {
            self.追加内部(2 * ノード, lo, m, 新規);
        } else {
            self.追加内部(2 * ノード + 1, m, hi, 新規);
        }
    }

    /// クエリ座標 `x`（`new` に渡した値のいずれか）における最小値。
    fn 最小値(&self, x: impl TryInto<i64>) -> i64 {
        let x = to_i64(x);
        let n = self.xs.len();
        let idx = self.xs.partition_point(|&v| v < x);
        debug_assert!(
            idx < n && self.xs[idx] == x,
            "李超木: 未登録のクエリ座標 {x}"
        );
        self.問い合わせ内部(1, 0, n, idx)
    }

    fn 問い合わせ内部(&self, ノード: usize, lo: usize, hi: usize, idx: usize) -> i64 {
        let mut 最小 = self.直線[ノード]
            .map(|l| Self::評価(l, self.xs[idx]))
            .unwrap_or(i64::MAX);
        if hi - lo > 1 {
            let m = (lo + hi) / 2;
            if idx < m {
                最小 = 最小.min(self.問い合わせ内部(2 * ノード, lo, m, idx));
            } else {
                最小 = 最小.min(self.問い合わせ内部(2 * ノード + 1, m, hi, idx));
            }
        }
        最小
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 総当たりと一致() {
        let 直線群 = [(2, 3), (-1, 5), (0, 0), (3, -4), (-2, 8), (5, -10)];
        let xs: Vec<i64> = (-10..=10).collect();
        let mut t = 李超木::new(xs.clone());
        for &(a, b) in &直線群 {
            t.直線追加(a, b);
        }
        for &x in &xs {
            let 期待 = 直線群.iter().map(|&(a, b)| a * x + b).min().unwrap();
            assert_eq!(t.最小値(x), 期待, "x={x}");
        }
    }

    #[test]
    fn 一本でも動く() {
        let mut t = 李超木::new(vec![0, 1, 2, 3]);
        t.直線追加(-2, 7);
        assert_eq!(t.最小値(3), 1);
    }
}
