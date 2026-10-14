//! 単峰関数の最小化。三分探索の上位互換として、実数は黄金分割探索、整数はフィボナッチ探索を置く。
//! どちらも前回の評価点を 1 つ使い回すので、1 反復あたりの `f` 評価は 1 回
//! （三分探索は 2 回評価して区間 2/3、こちらは 1 回評価で区間 0.618 倍）。
//!
//! 最大値が欲しいときは `f` の符号を反転させて（または `std::cmp::Reverse` で包んで）渡す。

use cargo_snippet::snippet;

#[snippet("黄金_分割_探索")]
#[snippet("凸_関数_最小化")]
/// 下に凸（単峰）な実数関数 `f` の最小値を与える `x` を黄金分割探索で求める。
/// 区間 `[左, 右]` で単峰であることが前提。`反復回数` 回だけ区間を 0.618 倍に狭める
/// （`f` の評価は `反復回数 + 2` 回。幅 1e9 → 1e-9 でも 90 回程度で足りる）。
/// `f` の値は `PartialOrd` なら何でもよい（`f64` / `i64` / タプルなど）。
fn 黄金分割探索<T: PartialOrd>(
    mut 左: f64, mut 右: f64, 反復回数: usize, mut f: impl FnMut(f64) -> T
) -> f64 {
    let r = (5f64.sqrt() - 1.0) / 2.0;
    let mut m1 = 右 - r * (右 - 左);
    let mut m2 = 左 + r * (右 - 左);
    let (mut f1, mut f2) = (f(m1), f(m2));
    for _ in 0..反復回数 {
        if f1 < f2 {
            右 = m2;
            (m2, f2) = (m1, f1);
            m1 = 右 - r * (右 - 左);
            f1 = f(m1);
        } else {
            左 = m1;
            (m1, f1) = (m2, f2);
            m2 = 左 + r * (右 - 左);
            f2 = f(m2);
        }
    }
    (左 + 右) / 2.0
}

#[snippet("フィボナッチ_探索")]
#[snippet("凸_関数_最小化")]
/// 下に凸（単峰）な整数関数 `f` の最小値を与える整数 `x` をフィボナッチ探索で求める。
/// 閉区間 `[左, 右]` で単峰であること（同値の平坦部が最小値以外にあると誤る場合がある）。
/// `f` の評価は約 `log_φ(右 - 左)` 回で、`[左, 右]` の外は評価しない。
/// `f` の値は `PartialOrd` なら何でもよい（`i64` / `f64` / タプルなど）。
fn フィボナッチ探索<T: PartialOrd>(左: i64, 右: i64, mut f: impl FnMut(i64) -> T) -> i64 {
    assert!(左 <= 右, "フィボナッチ探索: 空区間です");
    // 開区間 (lo, hi) の幅をフィボナッチ数にし、右にはみ出た点は +∞ (None) 扱い
    let (mut a, mut b) = (1i64, 2i64);
    while b < 右 - 左 + 2 {
        (a, b) = (b, a + b);
    }
    let mut g = |x: i64| if x <= 右 { Some(f(x)) } else { None };
    let より小 = |p: &Option<T>, q: &Option<T>| match (p, q) {
        (Some(p), Some(q)) => p < q,
        (Some(_), None) => true,
        _ => false,
    };
    let (mut lo, mut hi) = (左 - 1, 左 - 1 + b);
    let (mut m1, mut m2) = (hi - a, lo + a);
    let (mut f1, mut f2) = (g(m1), g(m2));
    while hi - lo > 3 {
        if より小(&f1, &f2) {
            hi = m2;
            (m2, f2) = (m1, f1);
            m1 = lo + hi - m2;
            f1 = g(m1);
        } else {
            lo = m1;
            (m1, f1) = (m2, f2);
            m2 = lo + hi - m1;
            f2 = g(m2);
        }
    }
    let mut 最良 = (m1, f1);
    for x in lo + 1..hi {
        if x != m1 {
            let v = g(x);
            if より小(&v, &最良.1) {
                最良 = (x, v);
            }
        }
    }
    最良.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 実数二次関数の最小() {
        // f(x) = (x-3)^2 + 1 → 最小は x=3
        let x = 黄金分割探索(-10.0, 10.0, 200, |x| (x - 3.0).powi(2) + 1.0);
        assert!((x - 3.0).abs() < 1e-6);
    }

    #[test]
    fn 実数の評価回数() {
        let mut 回数 = 0;
        黄金分割探索(0.0, 1.0, 50, |x| {
            回数 += 1;
            x.abs()
        });
        assert_eq!(回数, 52);
    }

    #[test]
    fn 整数二次関数の最小() {
        // f(x) = (x-7)^2 → 最小は x=7
        let x = フィボナッチ探索(0, 100, |x| (x - 7) * (x - 7));
        assert_eq!(x, 7);
    }

    #[test]
    fn 端が最小の場合() {
        assert_eq!(フィボナッチ探索(0, 50, |x| x), 0);
        assert_eq!(フィボナッチ探索(0, 50, |x| -x), 50);
    }

    #[test]
    fn 整数の区間外を評価しない() {
        let x = フィボナッチ探索(-5, 3, |x| {
            assert!((-5..=3).contains(&x));
            (x + 1).abs()
        });
        assert_eq!(x, -1);
    }

    #[test]
    fn 整数総当たり照合() {
        // 全区間 [l, r]（幅 0..=40）× 全頂点位置 × 左右の傾き違いで、厳密単峰関数の argmin を照合
        for l in -3..=3i64 {
            for w in 0..=40i64 {
                let r = l + w;
                for p in l..=r {
                    for (s, t) in [(1, 1), (1, 3), (5, 2)] {
                        let f = |x: i64| if x < p { (p - x) * s } else { (x - p) * t };
                        let mut 回数 = 0;
                        let x = フィボナッチ探索(l, r, |x| {
                            回数 += 1;
                            assert!(l <= x && x <= r);
                            f(x)
                        });
                        assert_eq!(x, p, "l={l} r={r} p={p}");
                        assert!(回数 <= 12, "評価回数 {回数} (w={w})");
                    }
                }
            }
        }
    }

    #[test]
    fn 整数の巨大区間() {
        let x = フィボナッチ探索(-1_000_000_000_000_000_000, 1_000_000_000_000_000_000, |x| {
            (x as i128 - 123_456_789_012_345).abs()
        });
        assert_eq!(x, 123_456_789_012_345);
    }
}
