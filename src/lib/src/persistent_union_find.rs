use cargo_snippet::snippet;

#[snippet("永続_UnionFind")]
#[snippet("PersistentUnionFind")]
#[snippet("永続_DSU")]
#[snippet("完全_永続_UnionFind")]
/// 永続配列のノード。`子` は左右の子ノード添字（葉では未使用）、`値` は葉のときの格納値。
struct ノード {
    子: [usize; 2],
    値: i64,
}

#[snippet("永続_UnionFind")]
#[snippet("PersistentUnionFind")]
#[snippet("永続_DSU")]
#[snippet("完全_永続_UnionFind")]
/// 完全永続 Union-Find。任意の過去バージョンを起点に結合でき、各バージョンの連結状態を保持・参照できる。
///
/// 1 回の結合・1 回の根探索は O(log^2 N)。
///
/// [`crate::rollback_union_find`] は最新版を巻き戻すだけだが、こちらは任意バージョンへ分岐できる。
///
/// - [`Self::統合`] : 指定バージョンに辺を足した**新バージョン番号**を返す。
/// - [`Self::同じ`] : 指定バージョンでの連結判定。
struct 永続UnionFind {
    n: usize,
    ノード列: Vec<ノード>,
    版: Vec<usize>, // 版[v] = バージョン v の永続配列の根ノード
}

#[snippet("永続_UnionFind")]
#[snippet("PersistentUnionFind")]
#[snippet("永続_DSU")]
#[snippet("完全_永続_UnionFind")]
impl 永続UnionFind {
    /// 要素数 `n` で初期化する。全要素が孤立したバージョン 0 が作られる。
    fn new(n: usize) -> Self {
        let mut s = Self {
            n,
            ノード列: Vec::new(),
            版: Vec::new(),
        };
        let 根 = s.構築(0, n.max(1));
        s.版.push(根);
        s
    }

    /// 区間 `[l, r)` の永続配列を全値 `-1`（サイズ 1 の根）で構築し、根ノード添字を返す。
    fn 構築(&mut self, l: usize, r: usize) -> usize {
        if r - l == 1 {
            return self.葉(-1);
        }
        let m = (l + r) / 2;
        let c0 = self.構築(l, m);
        let c1 = self.構築(m, r);
        self.内部(c0, c1)
    }

    fn 葉(&mut self, v: i64) -> usize {
        let id = self.ノード列.len();
        self.ノード列.push(ノード {
            子: [!0; 2],
            値: v,
        });
        id
    }

    fn 内部(&mut self, c0: usize, c1: usize) -> usize {
        let id = self.ノード列.len();
        self.ノード列.push(ノード {
            子: [c0, c1],
            値: 0,
        });
        id
    }

    /// バージョン根 `根` の永続配列で添字 `i` の値を読む。
    fn 取得(&self, 根: usize, mut l: usize, mut r: usize, i: usize) -> i64 {
        let mut node = 根;
        while r - l > 1 {
            let m = (l + r) / 2;
            if i < m {
                node = self.ノード列[node].子[0];
                r = m;
            } else {
                node = self.ノード列[node].子[1];
                l = m;
            }
        }
        self.ノード列[node].値
    }

    /// 永続的に添字 `i` を値 `v` に書き換え、新しい根ノード添字を返す。
    fn 書込(&mut self, 根: usize, l: usize, r: usize, i: usize, v: i64) -> usize {
        if r - l == 1 {
            return self.葉(v);
        }
        let m = (l + r) / 2;
        let c0 = self.ノード列[根].子[0];
        let c1 = self.ノード列[根].子[1];
        if i < m {
            let 新 = self.書込(c0, l, m, i, v);
            self.内部(新, c1)
        } else {
            let 新 = self.書込(c1, m, r, i, v);
            self.内部(c0, 新)
        }
    }

    /// バージョン `版番号` での `x` の根。
    fn 根(&self, 版番号: usize, x: usize) -> usize {
        let 根 = self.版[版番号];
        let mut x = x;
        loop {
            let v = self.取得(根, 0, self.n.max(1), x);
            if v < 0 {
                return x;
            }
            x = v as usize;
        }
    }

    /// バージョン `版番号` での `x` の属する集合のサイズ。
    fn サイズ(&self, 版番号: usize, x: usize) -> usize {
        let r = self.根(版番号, x);
        let 根 = self.版[版番号];
        (-self.取得(根, 0, self.n.max(1), r)) as usize
    }

    /// バージョン `版番号` で `x` と `y` が同じ集合か。
    fn 同じ(&self, 版番号: usize, x: usize, y: usize) -> bool {
        self.根(版番号, x) == self.根(版番号, y)
    }

    /// バージョン `版番号` に辺 `x - y` を足し、結果の新しいバージョン番号を返す。
    /// 既に同じ集合でも新バージョン（同一内容）を作る。
    fn 統合(&mut self, 版番号: usize, x: usize, y: usize) -> usize {
        let mut a = self.根(版番号, x);
        let mut b = self.根(版番号, y);
        let 根 = self.版[版番号];
        if a == b {
            self.版.push(根);
            return self.版.len() - 1;
        }
        let n = self.n.max(1);
        let sa = -self.取得(根, 0, n, a);
        let sb = -self.取得(根, 0, n, b);
        // a を大きい方にする
        if sa < sb {
            std::mem::swap(&mut a, &mut b);
        }
        let 合計 = sa + sb;
        let r1 = self.書込(根, 0, n, b, a as i64); // b の親を a に
        let r2 = self.書込(r1, 0, n, a, -合計); // a のサイズ更新
        self.版.push(r2);
        self.版.len() - 1
    }

    /// 現在のバージョン数。
    fn 版数(&self) -> usize {
        self.版.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 総当たり: 各バージョンの親配列を丸ごと保持する素朴な完全永続 DSU。
    struct 素朴 {
        n: usize,
        版: Vec<Vec<i64>>, // 各バージョンの「親 or -サイズ」配列
    }
    impl 素朴 {
        fn new(n: usize) -> Self {
            素朴 {
                n,
                版: vec![vec![-1i64; n]],
            }
        }
        fn 根(&self, 版番号: usize, mut x: usize) -> usize {
            while self.版[版番号][x] >= 0 {
                x = self.版[版番号][x] as usize;
            }
            x
        }
        fn サイズ(&self, 版番号: usize, x: usize) -> usize {
            let r = self.根(版番号, x);
            (-self.版[版番号][r]) as usize
        }
        fn 統合(&mut self, 版番号: usize, x: usize, y: usize) -> usize {
            let mut 新 = self.版[版番号].clone();
            let mut a = self.根(版番号, x);
            let mut b = self.根(版番号, y);
            if a != b {
                if -新[a] < -新[b] {
                    std::mem::swap(&mut a, &mut b);
                }
                let 合計 = 新[a] + 新[b];
                新[b] = a as i64;
                新[a] = 合計;
            }
            self.版.push(新);
            self.版.len() - 1
        }
    }

    #[test]
    fn 分岐して別々に育つ() {
        let mut uf = 永続UnionFind::new(5);
        let v1 = uf.統合(0, 0, 1); // {0,1}
        let v2 = uf.統合(v1, 2, 3); // {0,1},{2,3}
        // v1 から別分岐
        let v3 = uf.統合(v1, 0, 4); // {0,1,4}
        assert!(uf.同じ(v2, 2, 3));
        assert!(!uf.同じ(v2, 0, 4));
        assert!(uf.同じ(v3, 0, 4));
        assert!(!uf.同じ(v3, 2, 3));
        // 元のバージョン 0 は不変
        assert!(!uf.同じ(0, 0, 1));
        assert_eq!(uf.サイズ(v3, 4), 3);
        assert_eq!(uf.サイズ(v2, 0), 2);
    }

    #[test]
    fn 乱数で素朴と一致() {
        let mut seed = 0x2468_ace0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..40 {
            let n = (乱数() as usize % 8) + 2;
            let mut uf = 永続UnionFind::new(n);
            let mut bf = 素朴::new(n);
            for _ in 0..40 {
                let 版 = 乱数() as usize % uf.版数();
                let x = 乱数() as usize % n;
                let y = 乱数() as usize % n;
                let nv = uf.統合(版, x, y);
                let bv = bf.統合(版, x, y);
                assert_eq!(nv, bv);
                // 全バージョン・全ペアで連結性とサイズを照合
                for v in 0..uf.版数() {
                    for a in 0..n {
                        assert_eq!(uf.サイズ(v, a), bf.サイズ(v, a), "v={v} a={a}");
                        for b in 0..n {
                            assert_eq!(
                                uf.同じ(v, a, b),
                                bf.根(v, a) == bf.根(v, b),
                                "v={v} a={a} b={b}"
                            );
                        }
                    }
                }
            }
        }
    }
}
