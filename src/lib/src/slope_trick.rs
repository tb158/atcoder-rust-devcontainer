use crate::to_int::to_isize;
use cargo_snippet::snippet;
#[snippet("SlopeTrick")]
#[snippet("スロープ_トリック")]
#[snippet("傾き_トリック")]
#[snippet("凸_関数_DP")]
use std::cmp::Reverse;
#[snippet("SlopeTrick")]
#[snippet("スロープ_トリック")]
#[snippet("傾き_トリック")]
#[snippet("凸_関数_DP")]
use std::collections::BinaryHeap;

#[snippet("スロープ_トリック")]
#[snippet("SlopeTrick")]
#[snippet("傾き_トリック")]
#[snippet("凸_関数_DP")]
/// スロープトリック。区分線形な凸関数 `f(x)` を、傾きが変化する点（折れ点）の多重集合として管理する。
///
/// 各操作 `O(log N)`。DP の遷移が「`f += max(0, x-a)`」「`f += |x-a|`」
/// 「累積 min（スライド最小値）」などで書けるとき計算量を落とせる。
///
/// - [`Self::絶対値加算`] : `f(x) += |x - a|`。
/// - [`Self::右側を平坦化`] / [`Self::左側を平坦化`] : 累積 min。
/// - [`Self::最小値`] / [`Self::評価`] : 値の取得。
struct スロープトリック {
    最小値: isize,
    左: BinaryHeap<isize>,          // 最大ヒープ（実値 = top + 加算左）
    右: BinaryHeap<Reverse<isize>>, // 最小ヒープ（実値 = top + 加算右）
    加算左: isize,
    加算右: isize,
}

#[snippet("スロープ_トリック", include = "to_isize")]
#[snippet("SlopeTrick")]
#[snippet("傾き_トリック")]
#[snippet("凸_関数_DP")]
impl スロープトリック {
    /// 恒等的に 0 の関数 `f(x) = 0` を作る。
    fn new() -> Self {
        Self {
            最小値: 0,
            左: BinaryHeap::new(),
            右: BinaryHeap::new(),
            加算左: 0,
            加算右: 0,
        }
    }

    fn 左頂点(&self) -> Option<isize> {
        self.左.peek().map(|&v| v + self.加算左)
    }
    fn 右頂点(&self) -> Option<isize> {
        self.右.peek().map(|&Reverse(v)| v + self.加算右)
    }
    fn 左へ(&mut self, a: isize) {
        self.左.push(a - self.加算左);
    }
    fn 右へ(&mut self, a: isize) {
        self.右.push(Reverse(a - self.加算右));
    }

    /// 現在の最小値 `min_x f(x)`。
    fn 最小値(&self) -> isize {
        self.最小値
    }

    /// `f(x) += c`（定数加算）。引数は任意の整数型でよい（他のメソッドも同様）。
    fn 定数加算(&mut self, c: impl TryInto<isize>) {
        self.最小値 += to_isize(c);
    }

    /// `f(x) += max(0, x - a)`（右上がりの折れを追加）。
    fn 右傾き加算(&mut self, a: impl TryInto<isize>) {
        let a = to_isize(a);
        if let Some(l) = self.左頂点()
            && l > a
        {
            self.最小値 += l - a;
        }
        self.左へ(a);
        let v = self.左.pop().unwrap() + self.加算左;
        self.右へ(v);
    }

    /// `f(x) += max(0, a - x)`（左上がりの折れを追加）。
    fn 左傾き加算(&mut self, a: impl TryInto<isize>) {
        let a = to_isize(a);
        if let Some(r) = self.右頂点()
            && r < a
        {
            self.最小値 += a - r;
        }
        self.右へ(a);
        let Reverse(v) = self.右.pop().unwrap();
        let v = v + self.加算右;
        self.左へ(v);
    }

    /// `f(x) += |x - a|`。
    fn 絶対値加算(&mut self, a: impl TryInto<isize>) {
        let a = to_isize(a);
        self.右傾き加算(a);
        self.左傾き加算(a);
    }

    /// 累積 min: `f(x) := min_{y <= x} f(y)`（右側の上がりを消し、右へ平坦化）。
    fn 右側を平坦化(&mut self) {
        self.右.clear();
    }

    /// 累積 min: `f(x) := min_{y >= x} f(y)`（左側の上がりを消し、左へ平坦化）。
    fn 左側を平坦化(&mut self) {
        self.左.clear();
    }

    /// 平行移動: `f(x) := f(x - a)`（全折れ点を `a` ずらす）。
    fn 平行移動(&mut self, a: impl TryInto<isize>) {
        let a = to_isize(a);
        self.加算左 += a;
        self.加算右 += a;
    }

    /// スライド最小値: `f(x) := min_{a <= d <= b} f(x - d)`（`a <= b`）。
    fn 窓最小(&mut self, a: impl TryInto<isize>, b: impl TryInto<isize>) {
        let (a, b) = (to_isize(a), to_isize(b));
        assert!(a <= b);
        self.加算左 += a;
        self.加算右 += b;
    }

    /// `f(x)` の値を O(N) で評価する。
    fn 評価(&self, x: impl TryInto<isize>) -> isize {
        let x = to_isize(x);
        let mut v = self.最小値;
        for &l in self.左.iter() {
            let l = l + self.加算左;
            if l > x {
                v += l - x;
            }
        }
        for &Reverse(r) in self.右.iter() {
            let r = r + self.加算右;
            if r < x {
                v += x - r;
            }
        }
        v
    }
}

#[snippet("スロープ_トリック")]
#[snippet("SlopeTrick")]
#[snippet("傾き_トリック")]
#[snippet("凸_関数_DP")]
impl Default for スロープトリック {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIM: isize = 60;

    // 総当たり: 各整数 x in [-LIM, LIM] での f(x) を配列で直接持つ。
    struct 素朴 {
        g: Vec<isize>, // g[(x + LIM) as usize] = f(x)
    }
    impl 素朴 {
        fn new() -> Self {
            素朴 {
                g: vec![0; (2 * LIM + 1) as usize],
            }
        }
        fn 各(&mut self, mut f: impl FnMut(isize, isize) -> isize) {
            for x in -LIM..=LIM {
                let i = (x + LIM) as usize;
                self.g[i] = f(x, self.g[i]);
            }
        }
        fn 定数加算(&mut self, c: isize) {
            self.各(|_, v| v + c);
        }
        fn 右傾き加算(&mut self, a: isize) {
            self.各(|x, v| v + (x - a).max(0));
        }
        fn 左傾き加算(&mut self, a: isize) {
            self.各(|x, v| v + (a - x).max(0));
        }
        fn 絶対値加算(&mut self, a: isize) {
            self.各(|x, v| v + (x - a).abs());
        }
        fn 右側を平坦化(&mut self) {
            let mut 最小 = isize::MAX;
            for i in 0..self.g.len() {
                最小 = 最小.min(self.g[i]);
                self.g[i] = 最小;
            }
        }
        fn 左側を平坦化(&mut self) {
            let mut 最小 = isize::MAX;
            for i in (0..self.g.len()).rev() {
                最小 = 最小.min(self.g[i]);
                self.g[i] = 最小;
            }
        }
        fn 最小値(&self) -> isize {
            *self.g.iter().min().unwrap()
        }
        fn 評価(&self, x: isize) -> isize {
            self.g[(x + LIM) as usize]
        }
    }

    #[test]
    fn 絶対値の和() {
        // f(x) = |x-2| + |x+1| + |x-5|, 最小値は中央値 x=2 で 1+3+3=... 計算
        let mut st = スロープトリック::new();
        for a in [2, -1, 5] {
            st.絶対値加算(a);
        }
        // 中央値 x=2: |0|+|3|+|3| = 6
        assert_eq!(st.最小値(), 6);
        assert_eq!(st.評価(2), 6);
        assert_eq!(st.評価(0), 2 + 1 + 5); // |-2|+|1|+|-5|=8
    }

    #[test]
    fn 平行移動と窓最小() {
        let mut st = スロープトリック::new();
        st.絶対値加算(0); // f(x) = |x|
        st.平行移動(3); // f(x) = |x-3|
        assert_eq!(st.評価(3), 0);
        assert_eq!(st.評価(0), 3);
        // 窓最小 [0,2]: f(x) = min_{0<=d<=2} |x-3-d|
        st.窓最小(0, 2);
        // 最小は x in [3,5] で 0
        assert_eq!(st.評価(4), 0);
        assert_eq!(st.評価(3), 0);
        assert_eq!(st.評価(5), 0);
        assert_eq!(st.評価(2), 1);
        assert_eq!(st.評価(7), 2);
    }

    #[test]
    fn 乱数で総当たりと一致() {
        let mut seed = 0x5a5a_3c3cu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..120 {
            let mut st = スロープトリック::new();
            let mut bf = 素朴::new();
            let 回数 = (乱数() as usize % 12) + 1;
            for _ in 0..回数 {
                let a = (乱数() % 41) as isize - 20; // [-20, 20]
                match 乱数() % 6 {
                    0 => {
                        let c = (乱数() % 11) as isize;
                        st.定数加算(c);
                        bf.定数加算(c);
                    }
                    1 => {
                        st.右傾き加算(a);
                        bf.右傾き加算(a);
                    }
                    2 => {
                        st.左傾き加算(a);
                        bf.左傾き加算(a);
                    }
                    3 => {
                        st.絶対値加算(a);
                        bf.絶対値加算(a);
                    }
                    4 => {
                        st.右側を平坦化();
                        bf.右側を平坦化();
                    }
                    _ => {
                        st.左側を平坦化();
                        bf.左側を平坦化();
                    }
                }
            }
            assert_eq!(st.最小値(), bf.最小値());
            // 内側の範囲だけ評価（端は平坦化で外側の影響を受けるため避ける）
            for x in -30..=30 {
                assert_eq!(st.評価(x), bf.評価(x), "x={x}");
            }
        }
    }
}
