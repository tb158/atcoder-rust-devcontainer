#[snippet("FPS")]
#[snippet("FormalPowerSeries")]
#[snippet("多項式_exp")]
#[snippet("多項式_log")]
#[snippet("多項式_sqrt")]
#[snippet("多項式_平方根")]
#[snippet("多項式_累乗")]
#[snippet("多項式_逆元")]
#[snippet("形式的_冪級数")]
use ac_library::{ModInt998244353, convolution};
use cargo_snippet::snippet;

#[snippet("形式的_冪級数")]
#[snippet("FPS")]
#[snippet("FormalPowerSeries")]
#[snippet("多項式_逆元")]
#[snippet("多項式_log")]
#[snippet("多項式_exp")]
#[snippet("多項式_累乗")]
#[snippet("多項式_平方根")]
#[snippet("多項式_sqrt")]
/// 形式的冪級数（FPS）で扱う係数の型。mod 998244353（NTT フレンドリ素数）。
type 係数 = ModInt998244353;

#[snippet("形式的_冪級数")]
#[snippet("FPS")]
#[snippet("FormalPowerSeries")]
#[snippet("多項式_逆元")]
#[snippet("多項式_log")]
#[snippet("多項式_exp")]
#[snippet("多項式_累乗")]
/// 形式的微分。`f = Σ a_i x^i` → `Σ i·a_i x^{i-1}`。
fn 微分(f: &[係数]) -> Vec<係数> {
    if f.len() <= 1 {
        return Vec::new();
    }
    (1..f.len()).map(|i| f[i] * 係数::new(i as u64)).collect()
}

#[snippet("形式的_冪級数")]
#[snippet("FPS")]
#[snippet("FormalPowerSeries")]
#[snippet("多項式_逆元")]
#[snippet("多項式_log")]
#[snippet("多項式_exp")]
#[snippet("多項式_累乗")]
/// 形式的積分。`f = Σ a_i x^i` → `Σ a_i/(i+1) x^{i+1}`（定数項 0）。
fn 積分(f: &[係数]) -> Vec<係数> {
    let mut r = vec![係数::new(0); f.len() + 1];
    for (i, &c) in f.iter().enumerate() {
        r[i + 1] = c * 係数::new((i + 1) as u64).inv();
    }
    r
}

#[snippet("形式的_冪級数")]
#[snippet("FPS")]
#[snippet("FormalPowerSeries")]
#[snippet("多項式_逆元")]
#[snippet("多項式_log")]
#[snippet("多項式_exp")]
#[snippet("多項式_累乗")]
#[snippet("多項式_平方根")]
#[snippet("多項式_sqrt")]
/// `f * g ≡ 1 (mod x^n)` なる逆元 `g`（長さ `n`）を Newton 法で求める。O(n log n)。
/// 定数項が 0 のときは逆元が存在しないので panic する。乗算には `ac_library::convolution`（NTT）を使う。
fn 逆元(f: &[係数], n: usize) -> Vec<係数> {
    assert!(
        !f.is_empty() && f[0].val() != 0,
        "constant term must be nonzero"
    );
    if n == 0 {
        return Vec::new();
    }
    let mut g = vec![f[0].inv()];
    let mut k = 1;
    while k < n {
        k <<= 1;
        let 上限 = k.min(f.len());
        let mut fg = convolution(&f[..上限], &g);
        fg.truncate(k);
        for x in fg.iter_mut() {
            *x = -*x;
        }
        fg[0] += 係数::new(2);
        let mut ng = convolution(&g, &fg);
        ng.truncate(k);
        g = ng;
    }
    g.truncate(n);
    g.resize(n, 係数::new(0));
    g
}

#[snippet("形式的_冪級数")]
#[snippet("FPS")]
#[snippet("FormalPowerSeries")]
#[snippet("多項式_逆元")]
#[snippet("多項式_log")]
#[snippet("多項式_exp")]
#[snippet("多項式_累乗")]
/// `log f` を mod x^n で求める（長さ `n`）。定数項は 1 でなければならない。`log f = ∫ f'/f`。
fn log(f: &[係数], n: usize) -> Vec<係数> {
    assert!(
        !f.is_empty() && f[0].val() == 1,
        "constant term must be 1 for log"
    );
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![係数::new(0)];
    }
    let fd = 微分(f);
    let fi = 逆元(f, n);
    let mut prod = convolution(&fd, &fi);
    prod.truncate(n - 1);
    let mut r = 積分(&prod);
    r.truncate(n);
    r.resize(n, 係数::new(0));
    r
}

#[snippet("形式的_冪級数")]
#[snippet("FPS")]
#[snippet("FormalPowerSeries")]
#[snippet("多項式_逆元")]
#[snippet("多項式_log")]
#[snippet("多項式_exp")]
#[snippet("多項式_累乗")]
/// `exp f` を mod x^n で求める（長さ `n`）。定数項は 0 でなければならない。Newton 法で O(n log n)。
fn exp(f: &[係数], n: usize) -> Vec<係数> {
    assert!(
        f.is_empty() || f[0].val() == 0,
        "constant term must be 0 for exp"
    );
    if n == 0 {
        return Vec::new();
    }
    let mut g = vec![係数::new(1)];
    let mut k = 1;
    while k < n {
        k <<= 1;
        let lg = log(&g, k);
        let mut h = vec![係数::new(0); k];
        h[0] = 係数::new(1);
        for (i, hi) in h.iter_mut().enumerate() {
            *hi -= lg[i];
            if i < f.len() {
                *hi += f[i];
            }
        }
        let mut ng = convolution(&g, &h);
        ng.truncate(k);
        g = ng;
    }
    g.truncate(n);
    g.resize(n, 係数::new(0));
    g
}

#[snippet("形式的_冪級数")]
#[snippet("FPS")]
#[snippet("FormalPowerSeries")]
#[snippet("多項式_逆元")]
#[snippet("多項式_log")]
#[snippet("多項式_exp")]
#[snippet("多項式_累乗")]
/// `f^m` を mod x^n で求める（長さ `n`）。最低次の非零項を括り出し `exp(m·log)` で計算する。
/// `f ≡ 0` でも `m == 0` でも正しく扱う（`0^0 = 1`）。
fn 累乗(f: &[係数], m: impl TryInto<u64>, n: usize) -> Vec<係数> {
    let m: u64 = m.try_into().ok().expect("累乗: 引数が u64 に収まりません");
    if n == 0 {
        return Vec::new();
    }
    if m == 0 {
        let mut r = vec![係数::new(0); n];
        r[0] = 係数::new(1);
        return r;
    }
    // 最低次の非零項を探す
    let mut d = 0usize;
    while d < f.len() && f[d].val() == 0 {
        d += 1;
    }
    if d == f.len() {
        return vec![係数::new(0); n]; // f ≡ 0 かつ m >= 1
    }
    if (d as u128) * (m as u128) >= n as u128 {
        return vec![係数::new(0); n]; // シフトだけで n を超える
    }
    let shift = d * (m as usize);
    let len = n - shift;
    let c = f[d];
    let cinv = c.inv();
    // f を c·x^d で割って定数項 1 に正規化
    let g: Vec<係数> = (0..len)
        .map(|i| {
            let idx = d + i;
            if idx < f.len() {
                f[idx] * cinv
            } else {
                係数::new(0)
            }
        })
        .collect();
    let lg = log(&g, len);
    let mm = 係数::new(m);
    let scaled: Vec<係数> = lg.iter().map(|&x| x * mm).collect();
    let eg = exp(&scaled, len);
    let cm = c.pow(m);
    let mut r = vec![係数::new(0); n];
    for (i, &v) in eg.iter().enumerate() {
        r[shift + i] = v * cm;
    }
    r
}

#[snippet("形式的_冪級数")]
#[snippet("FPS")]
#[snippet("FormalPowerSeries")]
#[snippet("多項式_逆元")]
#[snippet("多項式_log")]
#[snippet("多項式_exp")]
#[snippet("多項式_累乗")]
#[snippet("多項式_平方根")]
#[snippet("多項式_sqrt")]
/// mod 998244353 上の平方根（Tonelli–Shanks）。`a` が平方剰余なら 1 つの解（小さい方）を `Some`、
/// 非平方剰余なら `None` を返す。FPS 平方根の定数項計算に使う。`p - 1 = 2^23·119` なので確実に動く。
fn 平方剰余の根(a: u64) -> Option<u64> {
    const P: u64 = 998244353;
    let a = a % P;
    if a == 0 {
        return Some(0);
    }
    let pw = |mut b: u64, mut e: u64| -> u64 {
        let mut r = 1u64;
        b %= P;
        while e > 0 {
            if e & 1 == 1 {
                r = r * b % P;
            }
            b = b * b % P;
            e >>= 1;
        }
        r
    };
    if pw(a, (P - 1) / 2) != 1 {
        return None; // 非平方剰余
    }
    // P - 1 = q·2^s（q 奇数）。
    let mut q = P - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    // 非平方剰余 z を 1 つ探す。
    let mut z = 2u64;
    while pw(z, (P - 1) / 2) == 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = pw(z, q);
    let mut t = pw(a, q);
    let mut r = pw(a, q.div_ceil(2)); // q は奇数なので (q+1)/2 と等しい
    loop {
        if t == 1 {
            return Some(r.min(P - r));
        }
        // t^(2^i) = 1 となる最小の i を探す。
        let mut i = 0u32;
        let mut tt = t;
        while tt != 1 {
            tt = tt * tt % P;
            i += 1;
        }
        let b = pw(c, 1u64 << (m - i - 1));
        m = i;
        c = b * b % P;
        t = t * c % P;
        r = r * b % P;
    }
}

#[snippet("形式的_冪級数")]
#[snippet("FPS")]
#[snippet("FormalPowerSeries")]
#[snippet("多項式_逆元")]
#[snippet("多項式_log")]
#[snippet("多項式_exp")]
#[snippet("多項式_累乗")]
#[snippet("多項式_平方根")]
#[snippet("多項式_sqrt")]
/// `g^2 ≡ f (mod x^n)` なる平方根 `g`（長さ `n`）を Newton 法で求める。`O(n log n)`。
///
/// 最低次の非零項の次数が奇数、または定数項が平方剰余でない場合は平方根が存在せず `None` を返す。
/// `f ≡ 0` なら全 0 を返す。定数項が 1 でなくても（平方剰余であれば）扱える。
fn 平方根(f: &[係数], n: usize) -> Option<Vec<係数>> {
    if n == 0 {
        return Some(Vec::new());
    }
    // 最低次の非零項を探す。
    let mut d = 0usize;
    while d < f.len() && f[d].val() == 0 {
        d += 1;
    }
    if d == f.len() {
        return Some(vec![係数::new(0); n]); // f ≡ 0
    }
    if d % 2 != 0 {
        return None; // 最低次が奇数 → FPS 平方根なし
    }
    let c0 = 平方剰余の根(f[d].val() as u64)?; // 定数項が非平方剰余なら None
    let shift = d / 2;
    if shift >= n {
        return Some(vec![係数::new(0); n]);
    }
    let len = n - shift;
    // f を f[d]·x^d で割って定数項 1 に正規化。
    let cinv = f[d].inv();
    let h: Vec<係数> = (0..len)
        .map(|i| {
            let idx = d + i;
            if idx < f.len() {
                f[idx] * cinv
            } else {
                係数::new(0)
            }
        })
        .collect();
    // 定数項 1 の系列に対する Newton 平方根（g[0] = 1）。
    let 逆2 = 係数::new(2).inv();
    let mut g = vec![係数::new(1)];
    let mut k = 1;
    while k < len {
        k <<= 1;
        let 上限 = k.min(h.len());
        let ig = 逆元(&g, k); // g^{-1} mod x^k
        let mut t = convolution(&h[..上限], &ig); // f·g^{-1}
        t.truncate(k);
        g.resize(k, 係数::new(0));
        for (gi, &ti) in g.iter_mut().zip(t.iter()) {
            *gi = (*gi + ti) * 逆2; // g ← (g + f/g)/2
        }
    }
    g.truncate(len);
    // sqrt(c0)·x^{d/2} を掛けて元のスケールへ戻す。
    let c = 係数::new(c0);
    let mut r = vec![係数::new(0); n];
    for (i, &v) in g.iter().enumerate() {
        r[shift + i] = v * c;
    }
    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn から(v: &[i64]) -> Vec<係数> {
        v.iter().map(|&x| 係数::new(x)).collect()
    }

    // mod x^n の素朴な乗算
    fn 乗算(a: &[係数], b: &[係数], n: usize) -> Vec<係数> {
        let mut c = convolution(a, b);
        c.truncate(n);
        c.resize(n, 係数::new(0));
        c
    }

    #[test]
    fn 逆元は積が1になる() {
        let f = から(&[1, 2, 3, 4, 5, 6, 7]);
        let n = 12;
        let g = 逆元(&f, n);
        let prod = 乗算(&f, &g, n);
        for (i, x) in prod.iter().enumerate() {
            let 期待 = if i == 0 { 1 } else { 0 };
            assert_eq!(x.val(), 期待, "係数 {i}");
        }
    }

    #[test]
    fn expとlogは互いに逆() {
        // 定数項 0 の f について log(exp f) == f
        let f = から(&[0, 1, 2, 3, 4, 5]);
        let n = 10;
        let e = exp(&f, n);
        assert_eq!(e[0].val(), 1);
        let l = log(&e, n);
        for (i, x) in l.iter().enumerate() {
            let 期待 = if i < f.len() { f[i].val() } else { 0 };
            assert_eq!(x.val(), 期待, "係数 {i}");
        }
    }

    #[test]
    fn 累乗は素朴な繰り返し乗算と一致() {
        let f = から(&[2, 3, 1, 4, 5]);
        let n = 14;
        for m in 0..6u64 {
            let got = 累乗(&f, m, n);
            let mut acc = から(&[1]);
            for _ in 0..m {
                acc = 乗算(&acc, &f, n);
            }
            acc.resize(n, 係数::new(0));
            for (i, (a, b)) in got.iter().zip(acc.iter()).enumerate() {
                assert_eq!(a.val(), b.val(), "m={m}, 係数 {i}");
            }
        }
    }

    #[test]
    fn 累乗は最低次シフトを扱う() {
        // f = x^2 (1 + x), m = 3 → x^6 (1+x)^3
        let f = から(&[0, 0, 1, 1]);
        let n = 14;
        let got = 累乗(&f, 3, n);
        let mut acc = から(&[1]);
        for _ in 0..3 {
            acc = 乗算(&acc, &f, n);
        }
        acc.resize(n, 係数::new(0));
        for (i, (a, b)) in got.iter().zip(acc.iter()).enumerate() {
            assert_eq!(a.val(), b.val(), "係数 {i}");
        }
    }

    #[test]
    fn 累乗のm0は1() {
        let f = から(&[3, 1, 4]);
        let r = 累乗(&f, 0, 5);
        assert_eq!(r[0].val(), 1);
        for x in &r[1..] {
            assert_eq!(x.val(), 0);
        }
    }

    #[test]
    fn 平方根は2乗で元に戻る() {
        // 定数項が平方剰余（=4）の系列。g^2 == f (mod x^n) を確認。
        let f = から(&[4, 3, 2, 7, 1, 5, 9]);
        let n = 12;
        let g = 平方根(&f, n).expect("4 は平方剰余なので存在する");
        let sq = 乗算(&g, &g, n);
        for (i, x) in sq.iter().enumerate() {
            let 期待 = if i < f.len() { f[i].val() } else { 0 };
            assert_eq!(x.val(), 期待, "係数 {i}");
        }
    }

    #[test]
    fn 平方根は最低次シフトを扱う() {
        // f = x^2·(9 + x + ...) → sqrt は x^1·(3 + ...)。最低次が偶数なら OK。
        let f = から(&[0, 0, 9, 6, 1, 4, 2]);
        let n = 12;
        let g = 平方根(&f, n).expect("最低次 2（偶数）かつ 9 は平方剰余");
        // g の最低次は 1、それ未満は 0。
        assert_eq!(g[0].val(), 0);
        let sq = 乗算(&g, &g, n);
        for (i, x) in sq.iter().enumerate() {
            let 期待 = if i < f.len() { f[i].val() } else { 0 };
            assert_eq!(x.val(), 期待, "係数 {i}");
        }
    }

    #[test]
    fn 平方根なしの判定() {
        // 最低次が奇数 → None。
        assert!(平方根(&から(&[0, 1, 2, 3]), 8).is_none());
        // 定数項 3 は mod 998244353 で非平方剰余 → None。
        assert!(平方根(&から(&[3, 1, 1]), 8).is_none());
    }

    #[test]
    fn 平方根を乱数で検証() {
        let mut seed = 0x59b7_ea71u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        let n = 16;
        let mut 検証回数 = 0;
        for _ in 0..400 {
            // 適当な系列を作り、自分自身の 2 乗を入力にすれば必ず平方剰余になる。
            let len = (乱数() as usize % 8) + 1;
            let base: Vec<係数> = (0..len).map(|_| 係数::new(乱数() % 50)).collect();
            if base[0].val() == 0 {
                continue; // 定数項 0 のときはシフトの別経路なので飛ばす
            }
            let f = 乗算(&base, &base, n);
            let g = 平方根(&f, n).expect("2 乗から作ったので平方根は存在する");
            let sq = 乗算(&g, &g, n);
            assert_eq!(sq, f, "g^2 != f");
            検証回数 += 1;
        }
        assert!(検証回数 > 0);
    }
}
