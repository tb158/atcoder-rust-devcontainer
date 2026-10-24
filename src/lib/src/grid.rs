use cargo_snippet::snippet;

#[snippet]
/// グリッド内で特定の要素（target）が含まれる最小の長方形範囲（上下左右の境界インデックス）を返す。O(HW)。
/// 返り値: Option<(h_min, h_max, w_min, w_max)>
fn グリッドの境界頂点<T: PartialEq + Copy>(
    グリッド: &[Vec<T>],
    target: T,
) -> Option<(usize, usize, usize, usize)> {
    // 上下の境界（行の中に target が含まれるか直接判定）
    let h_min = グリッド.iter().position(|行| 行.contains(&target))?;
    let h_max = グリッド.iter().rposition(|行| 行.contains(&target))?;

    // 左右の境界（各行での最初/最後の出現位置を抽出し、その最小・最大をとる）
    let w_min = グリッド
        .iter()
        .filter_map(|行| 行.iter().position(|&マス| マス == target))
        .min()?;
    let w_max = グリッド
        .iter()
        .filter_map(|行| 行.iter().rposition(|&c| c == target))
        .max()?;

    Some((h_min, h_max, w_min, w_max))
}

#[snippet("上下左右_方向")]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum 方向 {
    下 = 0,
    右 = 1,
    上 = 2,
    左 = 3,
}

#[snippet("上下左右_方向")]
use 方向::*;

#[snippet("上下左右_方向")]
impl 方向 {
    const 全方向: [方向; 4] = [下, 右, 上, 左];

    fn 変位(self) -> (usize, usize) {
        match self {
            下 => (1, 0),
            右 => (0, 1),
            上 => (!0, 0),
            左 => (0, !0),
        }
    }

    fn 文字(self) -> char {
        match self {
            下 => 'D',
            右 => 'R',
            上 => 'U',
            左 => 'L',
        }
    }
}
