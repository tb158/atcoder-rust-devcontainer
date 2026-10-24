use cargo_snippet::snippet;
#[snippet("SAM")]
#[snippet("SuffixAutomaton")]
#[snippet("接尾辞_オートマトン")]
#[snippet("相異なる_部分_文字列")]
#[snippet("部分_文字列の_数")]
use std::collections::HashMap;

#[snippet("接尾辞_オートマトン")]
#[snippet("SuffixAutomaton")]
#[snippet("SAM")]
#[snippet("相異なる_部分_文字列")]
#[snippet("部分_文字列の_数")]
/// 接尾辞オートマトンの 1 状態。`len` は同値類の最長文字列長、`link` は suffix link
/// （根は -1）、`next` は文字ごとの遷移先。
struct 状態 {
    len: usize,
    link: isize,
    next: HashMap<u8, usize>,
}

#[snippet("接尾辞_オートマトン")]
#[snippet("SuffixAutomaton")]
#[snippet("SAM")]
#[snippet("相異なる_部分_文字列")]
#[snippet("部分_文字列の_数")]
/// 接尾辞オートマトン（SAM）。文字列の全部分文字列を O(N) 状態で表す。構築 O(N)。
///
/// - [`Self::相異なる部分文字列数`] : 相異なる（空でない）部分文字列の個数。
/// - [`Self::含む`] : ある文字列が部分文字列として現れるか。
struct 接尾辞オートマトン {
    states: Vec<状態>,
    last: usize,
}

#[snippet("接尾辞_オートマトン")]
#[snippet("SuffixAutomaton")]
#[snippet("SAM")]
#[snippet("相異なる_部分_文字列")]
#[snippet("部分_文字列の_数")]
impl 接尾辞オートマトン {
    fn new() -> Self {
        Self {
            states: vec![状態 {
                len: 0,
                link: -1,
                next: HashMap::new(),
            }],
            last: 0,
        }
    }

    /// バイト列から構築する。
    fn 構築(s: &[u8]) -> Self {
        let mut sam = Self::new();
        for &c in s {
            sam.追加(c);
        }
        sam
    }

    /// 末尾に 1 文字追加する（オンライン構築）。
    fn 追加(&mut self, c: u8) {
        let cur = self.states.len();
        self.states.push(状態 {
            len: self.states[self.last].len + 1,
            link: -1,
            next: HashMap::new(),
        });
        let mut p = self.last as isize;
        while p != -1 && !self.states[p as usize].next.contains_key(&c) {
            self.states[p as usize].next.insert(c, cur);
            p = self.states[p as usize].link;
        }
        if p == -1 {
            self.states[cur].link = 0;
        } else {
            let q = self.states[p as usize].next[&c];
            if self.states[p as usize].len + 1 == self.states[q].len {
                self.states[cur].link = q as isize;
            } else {
                let clone = self.states.len();
                self.states.push(状態 {
                    len: self.states[p as usize].len + 1,
                    link: self.states[q].link,
                    next: self.states[q].next.clone(),
                });
                while p != -1 && self.states[p as usize].next.get(&c) == Some(&q) {
                    self.states[p as usize].next.insert(c, clone);
                    p = self.states[p as usize].link;
                }
                self.states[q].link = clone as isize;
                self.states[cur].link = clone as isize;
            }
        }
        self.last = cur;
    }

    /// 相異なる（空でない）部分文字列の個数。O(状態数)。
    fn 相異なる部分文字列数(&self) -> usize {
        let mut 合計 = 0usize;
        for v in 1..self.states.len() {
            let link = self.states[v].link;
            let 親len = if link < 0 {
                0
            } else {
                self.states[link as usize].len
            };
            合計 += self.states[v].len - 親len;
        }
        合計
    }

    /// `t` が部分文字列として現れるか。O(|t|)。
    fn 含む(&self, t: &[u8]) -> bool {
        let mut v = 0usize;
        for &c in t {
            match self.states[v].next.get(&c) {
                Some(&nx) => v = nx,
                None => return false,
            }
        }
        true
    }
}

#[snippet("接尾辞_オートマトン")]
#[snippet("SuffixAutomaton")]
#[snippet("SAM")]
#[snippet("相異なる_部分_文字列")]
#[snippet("部分_文字列の_数")]
impl Default for 接尾辞オートマトン {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn 素朴相異なる部分文字列数(s: &[u8]) -> usize {
        let mut 集合 = HashSet::new();
        for i in 0..s.len() {
            for j in i + 1..=s.len() {
                集合.insert(&s[i..j]);
            }
        }
        集合.len()
    }

    #[test]
    fn 相異なる部分文字列数が総当たりと一致() {
        for s in [
            &b"abcbc"[..],
            b"aaaa",
            b"abracadabra",
            b"mississippi",
            b"a",
            b"abab",
        ] {
            let sam = 接尾辞オートマトン::構築(s);
            assert_eq!(
                sam.相異なる部分文字列数(),
                素朴相異なる部分文字列数(s),
                "s={:?}",
                std::str::from_utf8(s).unwrap()
            );
        }
    }

    #[test]
    fn 含む判定() {
        let s = b"mississippi";
        let sam = 接尾辞オートマトン::構築(s);
        for t in [&b"issi"[..], b"ppi", b"miss", b"sip"] {
            assert!(
                sam.含む(t),
                "{:?} を含むはず",
                std::str::from_utf8(t).unwrap()
            );
        }
        for t in [&b"issu"[..], b"xyz", b"ppp", b"mississippii"] {
            assert!(
                !sam.含む(t),
                "{:?} は含まないはず",
                std::str::from_utf8(t).unwrap()
            );
        }
    }

    #[test]
    fn 全部分文字列を含むと判定する() {
        let s = b"abcbc";
        let sam = 接尾辞オートマトン::構築(s);
        // 全ての真の部分文字列で 含む が真
        for i in 0..s.len() {
            for j in i + 1..=s.len() {
                assert!(sam.含む(&s[i..j]));
            }
        }
    }
}
