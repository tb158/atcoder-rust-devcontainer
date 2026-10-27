use cargo_snippet::snippet;

#[snippet("トライ_木")]
#[snippet("Trie")]
#[snippet("トライ")]
/// 英小文字（a–z）用のトライ木。文字列集合への挿入・完全一致検索・
/// 接頭辞を共有する単語数の取得ができる。
///
/// 各操作の計算量は文字列長に比例。文字種を増やすときは配列幅 26 を変更する。
struct トライ木 {
    /// 各ノードの子（`子[ノード][c]` = 文字 c の子ノード番号）
    子: Vec<[Option<usize>; 26]>,
    /// そのノードを通過する単語数（= そのノードが表す接頭辞を持つ単語数）
    通過数: Vec<usize>,
    /// そのノードで終わる単語数
    終端数: Vec<usize>,
}

#[snippet("トライ_木")]
#[snippet("Trie")]
#[snippet("トライ")]
impl Default for トライ木 {
    fn default() -> Self {
        Self::new()
    }
}

#[snippet("トライ_木")]
#[snippet("Trie")]
#[snippet("トライ")]
impl トライ木 {
    /// 空のトライ木を作る（根ノードのみ）。
    fn new() -> Self {
        Self {
            子: vec![[None; 26]],
            通過数: vec![0],
            終端数: vec![0],
        }
    }

    /// 単語を挿入する。同じ単語の重複挿入も数えられる。
    fn 挿入(&mut self, 単語: &str) {
        let mut 現在 = 0;
        for c in 単語.bytes() {
            let idx = (c - b'a') as usize;
            現在 = match self.子[現在][idx] {
                Some(next) => next,
                None => {
                    let 新 = self.子.len();
                    self.子.push([None; 26]);
                    self.通過数.push(0);
                    self.終端数.push(0);
                    self.子[現在][idx] = Some(新);
                    新
                }
            };
            self.通過数[現在] += 1;
        }
        self.終端数[現在] += 1;
    }

    /// `単語` がちょうど登録されているか（完全一致）。
    fn 含む(&self, 単語: &str) -> bool {
        matches!(self.末尾ノード(単語), Some(n) if self.終端数[n] > 0)
    }

    /// `接頭辞` を接頭辞に持つ登録単語の個数（空文字列を渡したときは 0）。
    fn 接頭辞数(&self, 接頭辞: &str) -> usize {
        match self.末尾ノード(接頭辞) {
            Some(n) if n != 0 => self.通過数[n],
            _ => 0,
        }
    }

    /// 文字列 `s` を根からたどった末尾ノード番号。途中で枝が無ければ `None`。
    fn 末尾ノード(&self, s: &str) -> Option<usize> {
        let mut 現在 = 0;
        for c in s.bytes() {
            let idx = (c - b'a') as usize;
            現在 = self.子[現在][idx]?;
        }
        Some(現在)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 挿入と検索() {
        let mut t = トライ木::new();
        t.挿入("apple");
        t.挿入("app");
        t.挿入("apply");
        assert!(t.含む("apple"));
        assert!(t.含む("app"));
        assert!(!t.含む("ap")); // 接頭辞だが単語ではない
        assert!(!t.含む("banana"));
        assert_eq!(t.接頭辞数("app"), 3); // app, apple, apply
        assert_eq!(t.接頭辞数("appl"), 2); // apple, apply
        assert_eq!(t.接頭辞数("xyz"), 0);
    }
}
