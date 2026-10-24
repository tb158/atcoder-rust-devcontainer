use cargo_snippet::snippet;

#[snippet("偏角_ソート")]
#[snippet("偏角")]
#[snippet("外積")]
use std::cmp::Ordering;

#[snippet("偏角_ソート")]
#[snippet("偏角")]
fn 偏角グループ<T: Copy + PartialOrd + Default>((x, y): (T, T)) -> i32 {
    let z = T::default();
    if y > z || (y == z && x > z) { 0 } else { 1 }
}

#[snippet("外積")]
#[snippet("偏角_ソート")]
#[snippet("偏角")]
/// 2次元ベクトル `a`, `b` の外積 `a×b = a.x*b.y - a.y*b.x` を isize で返す。
///
/// 符号: 正 → b は a の反時計回り側、負 → 時計回り側、0 → 平行。
/// `i32`・`usize`・`isize` など `TryInto<isize>` な整数型に対応。座標の絶対値が 2×10^9 以下で安全。
fn 外積<T: Copy + TryInto<isize>>(a: (T, T), b: (T, T)) -> isize {
    let f = |x: T| {
        x.try_into()
            .ok()
            .expect("外積: 引数が isize に収まりません")
    };
    f(a.0) * f(b.1) - f(a.1) * f(b.0)
}

#[snippet("外積_3点")]
#[snippet("三点_外積")]
#[snippet("ccw")]
/// 3 点 `o, a, b` の外積 `(a-o) × (b-o)` を `i128` で厳密に返す。
///
/// 符号: 正 → `o→a→b` は左折（反時計回り）、負 → 右折、0 → 一直線。絶対値は
/// 3 点が張る平行四辺形の面積で、`o` から見た直線 `ab` までの距離の比較にも使える。
/// 差を取ってから `i128` に広げるので、座標が `±10^18` でも桁あふれしない。
///
/// 始点をそろえた 2 ベクトルの外積が欲しいだけなら [`外積`] を使う。
pub fn 外積3点(o: (isize, isize), a: (isize, isize), b: (isize, isize)) -> i128 {
    let ax = (a.0 - o.0) as i128;
    let ay = (a.1 - o.1) as i128;
    let bx = (b.0 - o.0) as i128;
    let by = (b.1 - o.1) as i128;
    ax * by - ay * bx
}

#[snippet("距離_2乗")]
#[snippet("2点間_距離の_2乗")]
/// 2 点間の距離の 2 乗を `i128` で返す。
///
/// 平方根を取らないので誤差なしで距離を比較できる。差を取ってから `i128` に広げるため、
/// 座標が `±10^18` でも桁あふれしない。実距離が要るときは `(d as f64).sqrt()`。
pub fn 距離2乗(a: (isize, isize), b: (isize, isize)) -> i128 {
    let dx = (a.0 - b.0) as i128;
    let dy = (a.1 - b.1) as i128;
    dx * dx + dy * dy
}

#[snippet("偏角_ソート")]
#[snippet("偏角")]
/// x 軸正方向を起点に反時計回りで点を並べる偏角比較。
///
/// 原点 `(0, 0)` は常に先頭（他の全点より `Less`）。
/// 偏角が同じ点同士は `Equal` を返す（ノルムでは比較しない）。
///
/// `i32`・`usize`・`isize` など `TryInto<isize>` な整数型のタプルに対応。座標の絶対値が 2×10^9 以下で安全。
///
/// ```ignore
/// points.sort_by(|a, b| a.cmp_偏角(b));
/// ```
trait 偏角Ord {
    fn cmp_偏角(&self, other: &Self) -> Ordering;
}

#[snippet("偏角_ソート")]
#[snippet("偏角")]
impl<T> 偏角Ord for (T, T)
where
    T: Copy + TryInto<isize> + Default + PartialOrd,
{
    fn cmp_偏角(&self, other: &Self) -> Ordering {
        let z = T::default();
        let za = self.0 == z && self.1 == z;
        let zb = other.0 == z && other.1 == z;
        match (za, zb) {
            (true, true) => return Ordering::Equal,
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            (false, false) => {}
        }
        let ha = 偏角グループ(*self);
        let hb = 偏角グループ(*other);
        if ha != hb {
            return ha.cmp(&hb);
        }
        let c = 外積(*self, *other);
        if c > 0 {
            Ordering::Less
        } else if c < 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 外積3点の符号が向きを表す() {
        assert!(外積3点((0, 0), (1, 0), (1, 1)) > 0); // 左折
        assert!(外積3点((0, 0), (1, 0), (1, -1)) < 0); // 右折
        assert_eq!(外積3点((0, 0), (1, 0), (2, 0)), 0); // 一直線
        // 絶対値は平行四辺形の面積
        assert_eq!(外積3点((0, 0), (3, 0), (0, 4)), 12);
    }

    #[test]
    fn 外積3点は大きな座標でも桁あふれしない() {
        let m = 1_000_000_000_000_000_000isize;
        assert_eq!(
            外積3点((-m, -m), (m, -m), (-m, m)),
            4 * (m as i128) * (m as i128)
        );
    }

    #[test]
    fn 距離2乗が三平方の定理と合う() {
        assert_eq!(距離2乗((0, 0), (3, 4)), 25);
        assert_eq!(距離2乗((1, 1), (1, 1)), 0);
        assert_eq!(距離2乗((-2, -3), (1, 1)), 25);
        let m = 1_000_000_000_000_000_000isize;
        assert_eq!(距離2乗((-m, 0), (m, 0)), 4 * (m as i128) * (m as i128));
    }

    #[test]
    fn 偏角ソートの基本順序() {
        let 正解 = vec![
            (1isize, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        let mut 入力 = vec![
            (-1, 0),
            (0, -1),
            (1, 1),
            (-1, -1),
            (1, 0),
            (0, 1),
            (1, -1),
            (-1, 1),
        ];
        入力.sort_by(|a, b| a.cmp_偏角(b));
        assert_eq!(入力, 正解);
    }

    #[test]
    fn 同一直線上はequal_stableソート() {
        let a = (1isize, 1);
        let b = (2isize, 2);
        let c = (3isize, 3);
        assert_eq!(a.cmp_偏角(&b), Ordering::Equal);
        assert_eq!(b.cmp_偏角(&c), Ordering::Equal);
        // stable sort: Equal な点は入力順を保つ
        let mut v = vec![b, a, c];
        v.sort_by(|x, y| x.cmp_偏角(y));
        assert_eq!(v, vec![b, a, c]);
    }

    #[test]
    fn 原点は必ず先頭() {
        let mut v = vec![(1isize, 0), (0, 0), (-1, -1), (0, 1)];
        v.sort_by(|a, b| a.cmp_偏角(b));
        assert_eq!(v[0], (0, 0));
        assert_eq!((0isize, 0).cmp_偏角(&(1, 0)), Ordering::Less);
        assert_eq!((0isize, 0).cmp_偏角(&(-1, -1)), Ordering::Less);
        assert_eq!((1isize, 0).cmp_偏角(&(0, 0)), Ordering::Greater);
        assert_eq!((0isize, 0).cmp_偏角(&(0, 0)), Ordering::Equal);
    }

    #[test]
    fn i32でも動く() {
        let mut v = vec![(0i32, 1), (1, 0), (0, -1)];
        v.sort_by(|a, b| a.cmp_偏角(b));
        assert_eq!(v, vec![(1, 0), (0, 1), (0, -1)]);
    }

    #[test]
    fn 外積の符号() {
        assert!(外積((1isize, 0), (0, 1)) > 0); // 反時計回り
        assert!(外積((1isize, 0), (0, -1)) < 0); // 時計回り
        assert_eq!(外積((1isize, 1), (2, 2)), 0); // 平行
        // i32 でも動く
        assert!(外積((1i32, 0), (0, 1)) > 0);
    }
}
