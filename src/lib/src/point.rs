use cargo_snippet::snippet;

#[snippet("点_P")]
#[proconio::derive_readable]
/// 2次元の格子点/ベクトル。`input! { p: P }` や `input! { ps: [P; n] }` で直接読める。
///
/// 座標は `isize` 固定。
/// 積を取るメソッド（内積・外積・ノルム2乗など）は座標の絶対値が 2×10^9 以下で安全。
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct P {
    x: isize,
    y: isize,
}

#[snippet("点_P")]
impl P {
    /// 内積 `self・other = x1*x2 + y1*y2`。
    fn 内積(self, other: P) -> isize {
        self.x * other.x + self.y * other.y
    }

    /// 外積 `self×other = x1*y2 - y1*x2`。
    ///
    /// 符号: 正 → other は self の反時計回り側、負 → 時計回り側、0 → 平行。
    fn 外積(self, other: P) -> isize {
        self.x * other.y - self.y * other.x
    }

    /// ノルムの2乗 `|self|^2`（整数のまま比較したいとき用）。
    fn ノルム2乗(self) -> isize {
        self.内積(self)
    }

    /// ユークリッドノルム `|self|`（f64）。
    fn ノルム(self) -> f64 {
        (self.ノルム2乗() as f64).sqrt()
    }

    /// other との距離の2乗（整数のまま比較したいとき用）。
    fn 距離2乗(self, other: P) -> isize {
        (self - other).ノルム2乗()
    }

    /// other とのユークリッド距離（f64）。
    fn 距離(self, other: P) -> f64 {
        (self - other).ノルム()
    }

    /// other とのマンハッタン距離 `|dx| + |dy|`。
    fn マンハッタン距離(self, other: P) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// other とのチェビシェフ距離 `max(|dx|, |dy|)`。
    fn チェビシェフ距離(self, other: P) -> isize {
        (self.x - other.x).abs().max((self.y - other.y).abs())
    }

    /// 原点中心に反時計回り 90 度回転した点。
    fn 回転90(self) -> P {
        P {
            x: -self.y,
            y: self.x,
        }
    }

    /// x 軸正方向からの偏角（ラジアン、`(-π, π]`）。
    fn 偏角(self) -> f64 {
        (self.y as f64).atan2(self.x as f64)
    }
}

#[snippet("点_P")]
impl std::ops::Add for P {
    type Output = P;
    fn add(self, other: P) -> P {
        P {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[snippet("点_P")]
impl std::ops::Sub for P {
    type Output = P;
    fn sub(self, other: P) -> P {
        P {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[snippet("点_P")]
impl std::ops::Neg for P {
    type Output = P;
    fn neg(self) -> P {
        P {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[snippet("点_P")]
impl std::ops::Mul<isize> for P {
    type Output = P;
    fn mul(self, k: isize) -> P {
        P {
            x: self.x * k,
            y: self.y * k,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_ops() {
        let a = P { x: 3, y: 4 };
        let b = P { x: -1, y: 2 };
        assert_eq!(a + b, P { x: 2, y: 6 });
        assert_eq!(a - b, P { x: 4, y: 2 });
        assert_eq!(-a, P { x: -3, y: -4 });
        assert_eq!(b * 3, P { x: -3, y: 6 });
        assert_eq!(a.内積(b), 5);
        assert_eq!(a.外積(b), 10);
        assert_eq!(a.ノルム2乗(), 25);
        assert_eq!(a.ノルム(), 5.0);
        assert_eq!(a.距離2乗(b), 20);
        assert_eq!(a.マンハッタン距離(b), 6);
        assert_eq!(a.チェビシェフ距離(b), 4);
        assert_eq!(a.回転90(), P { x: -4, y: 3 });
        assert_eq!(P { x: 0, y: 1 }.偏角(), std::f64::consts::FRAC_PI_2);
    }
}
