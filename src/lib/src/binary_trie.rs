use crate::to_int::to_u64;
use cargo_snippet::snippet;

#[snippet("二進_トライ")]
#[snippet("BinaryTrie")]
#[snippet("XOR_トライ")]
#[snippet("ビット_トライ")]
#[snippet("最大_XOR")]
/// 二進トライ（binary trie）。非負整数の多重集合を二進表現の木で管理し、挿入・削除・
/// 「指定値 `x` との XOR の最大／最小」「k 番目に小さい値」「`x` 以下の個数」などを各 `O(ビット数)` で行う。
///
/// `xor_basis`（線形基底）は「部分集合の XOR で作れる最大値」を扱うのに対し、こちらは
/// **格納済み要素そのもの**との XOR 最適化や順位クエリを行う別物。
/// `挿入` した値の上位 `ビット数` ビットのみを見るので、扱う値域に合わせて `ビット数` を決める。
///
/// XOR を使わず k 番目・順位だけが目的なら [`ソート済み多重集合`](crate::sorted_multiset::ソート済み多重集合)
/// を使う（負の値や整数以外も載り、メモリも軽く速い）。
struct 二進トライ {
    ビット数: usize,
    子: Vec<[Option<usize>; 2]>,
    通過数: Vec<usize>, // その節点を通る（生存）要素数
}

#[snippet("二進_トライ", include = "to_u64")]
#[snippet("BinaryTrie")]
#[snippet("XOR_トライ")]
#[snippet("ビット_トライ")]
#[snippet("最大_XOR")]
impl 二進トライ {
    /// `ビット数` ビット幅（`0..2^ビット数` の値）の空トライを作る。
    fn new(ビット数: usize) -> Self {
        Self {
            ビット数,
            子: vec![[None, None]],
            通過数: vec![0],
        }
    }

    /// 値 `x` を 1 個追加する。
    fn 挿入(&mut self, x: impl TryInto<u64>) {
        let x = to_u64(x);
        let mut v = 0usize;
        self.通過数[0] += 1;
        for b in (0..self.ビット数).rev() {
            let ビット = ((x >> b) & 1) as usize;
            v = match self.子[v][ビット] {
                Some(next) => next,
                None => {
                    let 新 = self.子.len();
                    self.子.push([None, None]);
                    self.通過数.push(0);
                    self.子[v][ビット] = Some(新);
                    新
                }
            };
            self.通過数[v] += 1;
        }
    }

    /// 値 `x` を 1 個削除する（存在する前提）。木構造は保持し、通過数のみ減らす。
    fn 削除(&mut self, x: impl TryInto<u64>) {
        let x = to_u64(x);
        let mut v = 0usize;
        self.通過数[0] -= 1;
        for b in (0..self.ビット数).rev() {
            let ビット = ((x >> b) & 1) as usize;
            v = self.子[v][ビット].unwrap();
            self.通過数[v] -= 1;
        }
    }

    /// 格納されている要素の総数。
    fn 個数(&self) -> usize {
        self.通過数[0]
    }

    /// 値 `x` が格納されている個数。
    fn 含む個数(&self, x: impl TryInto<u64>) -> usize {
        let x = to_u64(x);
        let mut v = 0usize;
        for b in (0..self.ビット数).rev() {
            let ビット = ((x >> b) & 1) as usize;
            match self.子[v][ビット] {
                Some(c) if self.通過数[c] > 0 => v = c,
                _ => return 0,
            }
        }
        self.通過数[v]
    }

    /// 格納済みのいずれかの要素 `y` との `x ^ y` を最大化する `x ^ y` を返す（空でないこと）。
    fn 最大xor(&self, x: impl TryInto<u64>) -> usize {
        let x = to_u64(x);
        let mut v = 0usize;
        let mut 結果 = 0u64;
        for b in (0..self.ビット数).rev() {
            let ビット = ((x >> b) & 1) as usize;
            let 望み = ビット ^ 1; // XOR を 1 にしたいので反対ビットへ
            match self.子[v][望み] {
                Some(c) if self.通過数[c] > 0 => {
                    結果 |= 1 << b;
                    v = c;
                }
                _ => v = self.子[v][ビット].unwrap(),
            }
        }
        結果 as usize
    }

    /// 格納済みのいずれかの要素 `y` との `x ^ y` を最小化する `x ^ y` を返す（空でないこと）。
    fn 最小xor(&self, x: impl TryInto<u64>) -> usize {
        let x = to_u64(x);
        let mut v = 0usize;
        let mut 結果 = 0u64;
        for b in (0..self.ビット数).rev() {
            let ビット = ((x >> b) & 1) as usize;
            // 同じビットへ進めば XOR は 0
            match self.子[v][ビット] {
                Some(c) if self.通過数[c] > 0 => v = c,
                _ => {
                    結果 |= 1 << b;
                    v = self.子[v][ビット ^ 1].unwrap();
                }
            }
        }
        結果 as usize
    }

    /// 0 始まりで `k` 番目に小さい値を返す（多重度込み、`0 <= k < 個数`）。
    fn k番目(&self, mut k: usize) -> usize {
        let mut v = 0usize;
        let mut 結果 = 0u64;
        for b in (0..self.ビット数).rev() {
            let c0 = self.子[v][0];
            let 左 = c0.map_or(0, |c| self.通過数[c]);
            if k < 左 {
                v = c0.unwrap();
            } else {
                k -= 左;
                結果 |= 1 << b;
                v = self.子[v][1].unwrap();
            }
        }
        結果 as usize
    }

    /// `x` 以下の格納要素の個数（多重度込み）。
    fn 以下の個数(&self, x: impl TryInto<u64>) -> usize {
        let x = to_u64(x);
        let mut v = 0usize;
        let mut 個数 = 0usize;
        for b in (0..self.ビット数).rev() {
            let ビット = ((x >> b) & 1) as usize;
            if ビット == 1 {
                // このビットで 0 へ分岐する要素は（上位一致のもと）すべて x 未満。
                if let Some(c0) = self.子[v][0] {
                    個数 += self.通過数[c0];
                }
            }
            match self.子[v][ビット] {
                Some(c) if self.通過数[c] > 0 => v = c,
                _ => return 個数,
            }
        }
        個数 + self.通過数[v] // x と等しい要素
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 乱数で多重集合と一致() {
        let ビット数 = 16usize;
        let 上限 = 1u64 << ビット数;
        let mut seed = 0x51ed_c0deu64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..40 {
            let mut trie = 二進トライ::new(ビット数);
            let mut 集合: Vec<u64> = Vec::new(); // 昇順を保つ多重集合
            for _ in 0..300 {
                let 操作 = 乱数() % 5;
                if 操作 == 0 || 集合.is_empty() {
                    // 挿入
                    let x = 乱数() % 上限;
                    trie.挿入(x);
                    let pos = 集合.partition_point(|&y| y < x);
                    集合.insert(pos, x);
                } else if 操作 == 1 {
                    // 削除（存在する要素）
                    let idx = 乱数() as usize % 集合.len();
                    let x = 集合.remove(idx);
                    trie.削除(x);
                } else if 操作 == 2 {
                    // 最大・最小 XOR
                    let x = 乱数() % 上限;
                    let 最大 = 集合.iter().map(|&y| x ^ y).max().unwrap();
                    let 最小 = 集合.iter().map(|&y| x ^ y).min().unwrap();
                    assert_eq!(trie.最大xor(x), 最大 as usize);
                    assert_eq!(trie.最小xor(x), 最小 as usize);
                } else if 操作 == 3 {
                    // k 番目
                    let k = 乱数() as usize % 集合.len();
                    assert_eq!(trie.k番目(k), 集合[k] as usize);
                } else {
                    // 以下の個数 / 含む個数
                    let x = 乱数() % 上限;
                    let 以下 = 集合.iter().filter(|&&y| y <= x).count();
                    assert_eq!(trie.以下の個数(x), 以下);
                    let 含む = 集合.iter().filter(|&&y| y == x).count();
                    assert_eq!(trie.含む個数(x), 含む);
                }
                assert_eq!(trie.個数(), 集合.len());
            }
        }
    }
}
