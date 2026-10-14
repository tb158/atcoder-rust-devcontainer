use cargo_snippet::snippet;
#[snippet("Eertree")]
#[snippet("PalindromicTree")]
#[snippet("回文の_数")]
#[snippet("回文_木")]
#[snippet("相異なる_回文")]
use std::collections::HashMap;

#[snippet("回文_木")]
#[snippet("PalindromicTree")]
#[snippet("Eertree")]
#[snippet("回文の_数")]
#[snippet("相異なる_回文")]
/// 回文木（Eertree / Palindromic Tree）の 1 ノード。`len` は表す回文の長さ
/// （根は -1 と 0）、`suff` は最長の真回文接尾辞へのリンク、`次` は文字ごとの遷移、
/// `出現` はその回文が「最長回文接尾辞」として現れた回数。
struct 回文ノード {
    len: i64,
    suff: usize,
    次: HashMap<u8, usize>,
    出現: usize,
}

#[snippet("回文_木")]
#[snippet("PalindromicTree")]
#[snippet("Eertree")]
#[snippet("回文の_数")]
#[snippet("相異なる_回文")]
/// 回文木（Eertree）。文字列の全「相異なる回文部分文字列」を O(N) 個のノードで表す。構築 O(N)
/// （アルファベットは `HashMap` 管理）。`manacher` は中心ごとの最大回文半径、`suffix_automaton` は
/// 全部分文字列を扱うが、回文部分文字列の列挙・計数にはこの構造が向く。
///
/// - [`Self::相異なる回文数`] : 空でない相異なる回文部分文字列の個数。
/// - [`Self::回文部分文字列の総数`] : 回文となる部分文字列 `(i, j)` の総数（重複含む）。
struct 回文木 {
    ノード: Vec<回文ノード>,
    最後: usize,
    s: Vec<u8>,
}

#[snippet("回文_木")]
#[snippet("PalindromicTree")]
#[snippet("Eertree")]
#[snippet("回文の_数")]
#[snippet("相異なる_回文")]
impl 回文木 {
    fn new() -> Self {
        // ノード0: len=-1（虚根）, ノード1: len=0
        let 虚根 = 回文ノード {
            len: -1,
            suff: 0,
            次: HashMap::new(),
            出現: 0,
        };
        let 空根 = 回文ノード {
            len: 0,
            suff: 0,
            次: HashMap::new(),
            出現: 0,
        };
        Self {
            ノード: vec![虚根, 空根],
            最後: 1,
            s: Vec::new(),
        }
    }

    /// バイト列から構築する。
    fn 構築(s: &[u8]) -> Self {
        let mut t = Self::new();
        for &c in s {
            t.追加(c);
        }
        t
    }

    /// `位置` の文字 `c` を末尾に追加したとき、`位置` で終わる最長回文接尾辞のノードを返す。
    fn リンクを辿る(&self, mut v: usize, 位置: usize, c: u8) -> usize {
        loop {
            let len = self.ノード[v].len;
            let i = 位置 as i64 - len - 1;
            if i >= 0 && self.s[i as usize] == c {
                return v;
            }
            v = self.ノード[v].suff;
        }
    }

    /// 末尾に 1 文字追加する。新しい相異なる回文が生まれたら true。
    fn 追加(&mut self, c: u8) -> bool {
        let 位置 = self.s.len();
        self.s.push(c);
        let 親 = self.リンクを辿る(self.最後, 位置, c);
        if let Some(&既存) = self.ノード[親].次.get(&c) {
            self.最後 = 既存;
            self.ノード[既存].出現 += 1;
            return false;
        }
        let 新 = self.ノード.len();
        let 新len = self.ノード[親].len + 2;
        let 新suff = if 新len == 1 {
            1 // 長さ1の回文の接尾辞リンクは空根
        } else {
            let t = self.リンクを辿る(self.ノード[親].suff, 位置, c);
            self.ノード[t].次[&c]
        };
        self.ノード.push(回文ノード {
            len: 新len,
            suff: 新suff,
            次: HashMap::new(),
            出現: 1,
        });
        self.ノード[親].次.insert(c, 新);
        self.最後 = 新;
        true
    }

    /// 空でない相異なる回文部分文字列の個数（2 つの根を除いたノード数）。
    fn 相異なる回文数(&self) -> usize {
        self.ノード.len() - 2
    }

    /// 回文となる部分文字列 `(i, j)` の総数（重複を含む）。O(ノード数)。
    fn 回文部分文字列の総数(&self) -> usize {
        let mut 出現: Vec<usize> = self.ノード.iter().map(|n| n.出現).collect();
        // ノードは追加順（＝len 非減少ではないが suff は自分より小さい添字を指す）なので逆順で伝播
        for v in (2..self.ノード.len()).rev() {
            let s = self.ノード[v].suff;
            出現[s] += 出現[v];
        }
        // 根 2 つを除いた出現回数の総和
        出現[2..].iter().sum()
    }
}

#[snippet("回文_木")]
#[snippet("PalindromicTree")]
#[snippet("Eertree")]
#[snippet("回文の_数")]
#[snippet("相異なる_回文")]
impl Default for 回文木 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn 回文か(b: &[u8]) -> bool {
        let n = b.len();
        (0..n / 2).all(|i| b[i] == b[n - 1 - i])
    }

    fn 素朴相異なる回文数(s: &[u8]) -> usize {
        let mut 集合 = HashSet::new();
        for i in 0..s.len() {
            for j in i + 1..=s.len() {
                if 回文か(&s[i..j]) {
                    集合.insert(&s[i..j]);
                }
            }
        }
        集合.len()
    }

    fn 素朴総数(s: &[u8]) -> usize {
        let mut 数 = 0;
        for i in 0..s.len() {
            for j in i + 1..=s.len() {
                if 回文か(&s[i..j]) {
                    数 += 1;
                }
            }
        }
        数
    }

    #[test]
    fn 相異なる回文数が総当たりと一致() {
        for s in [
            &b"eertree"[..],
            b"aaaa",
            b"abacaba",
            b"mississippi",
            b"a",
            b"abcba",
            b"abccba",
        ] {
            let pt = 回文木::構築(s);
            assert_eq!(
                pt.相異なる回文数(),
                素朴相異なる回文数(s),
                "s={:?}",
                std::str::from_utf8(s).unwrap()
            );
        }
    }

    #[test]
    fn 回文部分文字列の総数が総当たりと一致() {
        for s in [
            &b"eertree"[..],
            b"aaaa",
            b"abacaba",
            b"mississippi",
            b"abccba",
        ] {
            let pt = 回文木::構築(s);
            assert_eq!(
                pt.回文部分文字列の総数(),
                素朴総数(s),
                "s={:?}",
                std::str::from_utf8(s).unwrap()
            );
        }
    }

    #[test]
    fn 擬似乱数で一致() {
        let mut seed = 0x5151_2727u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..50 {
            let n = (乱数() as usize % 20) + 1;
            let s: Vec<u8> = (0..n).map(|_| b'a' + (乱数() % 3) as u8).collect();
            let pt = 回文木::構築(&s);
            assert_eq!(pt.相異なる回文数(), 素朴相異なる回文数(&s));
            assert_eq!(pt.回文部分文字列の総数(), 素朴総数(&s));
        }
    }
}
