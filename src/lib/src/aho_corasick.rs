use cargo_snippet::snippet;
#[snippet("AhoCorasick")]
#[snippet("エイホ_コラシック")]
#[snippet("トライ_法_オートマトン")]
#[snippet("多重_文字列_検索")]
#[snippet("複数_パターン_照合")]
use std::collections::{HashMap, VecDeque};

#[snippet("AhoCorasick")]
#[snippet("エイホ_コラシック")]
#[snippet("複数_パターン_照合")]
#[snippet("多重_文字列_検索")]
#[snippet("トライ_法_オートマトン")]
/// Aho-Corasick 法による複数パターン同時照合。バイト列（`&[u8]`）を対象とする。
///
/// トライ + fail リンクを構築し、テキストを 1 度走査して各パターンの**出現回数**を
/// まとめて数える。重なる出現も全て数える。
struct エイホコラシック {
    遷移: Vec<HashMap<u8, usize>>,
    fail: Vec<usize>,
    bfs順: Vec<usize>,
    パターン終端: Vec<usize>,
}

#[snippet("AhoCorasick")]
#[snippet("エイホ_コラシック")]
#[snippet("複数_パターン_照合")]
#[snippet("多重_文字列_検索")]
#[snippet("トライ_法_オートマトン")]
impl エイホコラシック {
    /// パターン群から構築する。O(Σパターン長)。空パターンは終端＝根として扱う。
    fn new(パターン群: &[&[u8]]) -> Self {
        let mut 遷移: Vec<HashMap<u8, usize>> = vec![HashMap::new()];
        let mut パターン終端 = vec![0usize; パターン群.len()];
        for (pi, pat) in パターン群.iter().enumerate() {
            let mut cur = 0usize;
            for &c in *pat {
                if let Some(&nx) = 遷移[cur].get(&c) {
                    cur = nx;
                } else {
                    let 新 = 遷移.len();
                    遷移.push(HashMap::new());
                    遷移[cur].insert(c, 新);
                    cur = 新;
                }
            }
            パターン終端[pi] = cur;
        }
        // fail リンクを BFS で構築
        let mut fail = vec![0usize; 遷移.len()];
        let mut bfs順 = Vec::new();
        let mut 探索 = VecDeque::new();
        let 直下: Vec<usize> = 遷移[0].values().copied().collect();
        for next in 直下 {
            fail[next] = 0;
            bfs順.push(next);
            探索.push_back(next);
        }
        while let Some(now) = 探索.pop_front() {
            let 子: Vec<(u8, usize)> = 遷移[now].iter().map(|(&c, &next)| (c, next)).collect();
            for (c, next) in 子 {
                // fail[next] = goto(fail[now], c)
                let mut f = fail[now];
                let nf = loop {
                    if let Some(&nx) = 遷移[f].get(&c) {
                        break nx;
                    }
                    if f == 0 {
                        break 0;
                    }
                    f = fail[f];
                };
                fail[next] = nf;
                bfs順.push(next);
                探索.push_back(next);
            }
        }
        Self {
            遷移,
            fail,
            bfs順,
            パターン終端,
        }
    }

    /// 文字 `c` での遷移（fail リンクを辿るオートマトン遷移）。
    fn 進む(&self, mut v: usize, c: u8) -> usize {
        loop {
            if let Some(&nx) = self.遷移[v].get(&c) {
                return nx;
            }
            if v == 0 {
                return 0;
            }
            v = self.fail[v];
        }
    }

    /// `テキスト` 中での各パターンの出現回数を、`new` に渡した順で返す。
    /// O(テキスト長 + マッチ数)。
    fn 出現回数(&self, テキスト: &[u8]) -> Vec<usize> {
        let mut 訪問 = vec![0usize; self.遷移.len()];
        let mut v = 0usize;
        for &c in テキスト {
            v = self.進む(v, c);
            訪問[v] += 1;
        }
        // fail 木に沿って深い順に集約（bfs順の逆）
        for &node in self.bfs順.iter().rev() {
            訪問[self.fail[node]] += 訪問[node];
        }
        self.パターン終端.iter().map(|&node| 訪問[node]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 素朴出現回数(テキスト: &[u8], パターン: &[u8]) -> usize {
        if パターン.is_empty() || パターン.len() > テキスト.len() {
            return 0;
        }
        (0..=テキスト.len() - パターン.len())
            .filter(|&i| &テキスト[i..i + パターン.len()] == パターン)
            .count()
    }

    #[test]
    fn 古典例() {
        let パターン: Vec<&[u8]> = vec![b"he", b"she", b"his", b"hers"];
        let ac = エイホコラシック::new(&パターン);
        let 結果 = ac.出現回数(b"ushers");
        // he=1, she=1, his=0, hers=1
        assert_eq!(結果, vec![1, 1, 0, 1]);
    }

    #[test]
    fn 重なる出現も数える() {
        let パターン: Vec<&[u8]> = vec![b"aa", b"aaa"];
        let ac = エイホコラシック::new(&パターン);
        // "aaaa": aa は 3 回、aaa は 2 回
        assert_eq!(ac.出現回数(b"aaaa"), vec![3, 2]);
    }

    #[test]
    fn 総当たりと一致() {
        let テキスト = b"abracadabraabracadabra";
        let パターン: Vec<&[u8]> = vec![
            b"abra",
            b"cad",
            b"a",
            b"ab",
            b"bra",
            b"dab",
            b"xyz",
            b"abracadabra",
        ];
        let ac = エイホコラシック::new(&パターン);
        let 結果 = ac.出現回数(テキスト);
        for (i, pat) in パターン.iter().enumerate() {
            assert_eq!(結果[i], 素朴出現回数(テキスト, pat), "パターン {i}");
        }
    }
}
