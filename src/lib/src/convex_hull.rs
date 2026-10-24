use crate::geometry::外積3点;
use cargo_snippet::snippet;

#[snippet("凸包", include = "外積_3点")]
#[snippet("ConvexHull")]
#[snippet("convex_hull")]
/// 凸包（Andrew's monotone chain）。与えた点集合をすべて含む最小の凸多角形の
/// 頂点を、反時計回りの順で返す。O(N log N)。
///
/// 一直線上に並ぶ中間点（凸包の辺上の点）は頂点に含めない。
/// 点が2個以下のときは重複を除いた入力をそのまま返す。
pub fn 凸包(点群: &[(isize, isize)]) -> Vec<(isize, isize)> {
    let mut 点 = 点群.to_vec();
    点.sort();
    点.dedup();
    let n = 点.len();
    if n <= 2 {
        return 点;
    }

    let mut 下側: Vec<(isize, isize)> = Vec::new();
    for &p in &点 {
        while 下側.len() >= 2 && 外積3点(下側[下側.len() - 2], 下側[下側.len() - 1], p) <= 0
        {
            下側.pop();
        }
        下側.push(p);
    }

    let mut 上側: Vec<(isize, isize)> = Vec::new();
    for &p in 点.iter().rev() {
        while 上側.len() >= 2 && 外積3点(上側[上側.len() - 2], 上側[上側.len() - 1], p) <= 0
        {
            上側.pop();
        }
        上側.push(p);
    }

    下側.pop(); // 末尾は上側の先頭と重複
    上側.pop(); // 末尾は下側の先頭と重複
    下側.extend(上側);
    下側
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 正方形と内部点() {
        let 点 = vec![(0, 0), (2, 0), (2, 2), (0, 2), (1, 1)];
        let mut h = 凸包(&点);
        h.sort();
        assert_eq!(h, vec![(0, 0), (0, 2), (2, 0), (2, 2)]); // 内部点(1,1)は除外
    }

    #[test]
    fn 辺上の点は除外() {
        // (1,0) は (0,0)-(2,0) の辺上
        let 点 = vec![(0, 0), (1, 0), (2, 0), (1, 2)];
        let h = 凸包(&点);
        assert_eq!(h.len(), 3);
        assert!(!h.contains(&(1, 0)));
    }
}
