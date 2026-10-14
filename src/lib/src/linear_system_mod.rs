use crate::mod_pow::累乗_mod;
use cargo_snippet::snippet;

#[snippet("mod_連立_一次_方程式", include = "累乗_mod")]
#[snippet("mod_連立_方程式")]
#[snippet("素数_mod_掃き出し")]
#[snippet("ガウス_消去_mod")]
/// 素数 `法` 上の連立一次方程式 `A x = b` を掃き出し（RREF）で解く。O(行数·変数数·min)。
///
/// `係数` は各式の係数行（全行同じ長さ）、`右辺` は各式の右辺。解が無ければ `None`、
/// 有れば解の一つ（自由変数は 0 とおく）を返す。`法` は**素数**であること。
/// XOR（mod 2）専用の高速版は [`crate::gf2_elimination`]、行列式だけなら
/// [`crate::matrix_det_mod`] を使う。
fn 連立一次方程式_mod(係数: &[Vec<u64>], 右辺: &[u64], 法: u64) -> Option<Vec<u64>> {
    let 行数 = 係数.len();
    let 変数数 = if 行数 == 0 { 0 } else { 係数[0].len() };
    // 拡大行列（末尾列が右辺）
    let mut a: Vec<Vec<u64>> = (0..行数)
        .map(|i| {
            let mut row: Vec<u64> = 係数[i].iter().map(|&x| x % 法).collect();
            row.push(右辺[i] % 法);
            row
        })
        .collect();
    let mut ピボット行 = vec![!0; 変数数];
    let mut r = 0usize;
    // c は列番号として複数の配列・範囲に使うため添字ループのまま扱う
    #[allow(clippy::needless_range_loop)]
    for c in 0..変数数 {
        let Some(sel) = (r..行数).find(|&i| a[i][c] != 0) else {
            continue;
        };
        a.swap(r, sel);
        let inv = 累乗_mod(a[r][c], 法 - 2, 法);
        for j in c..=変数数 {
            a[r][j] = a[r][j] * inv % 法;
        }
        for i in 0..行数 {
            if i != r && a[i][c] != 0 {
                let f = a[i][c];
                for j in c..=変数数 {
                    a[i][j] = (a[i][j] + 法 - f * a[r][j] % 法) % 法;
                }
            }
        }
        ピボット行[c] = r;
        r += 1;
        if r == 行数 {
            break;
        }
    }
    // 係数が全 0 で右辺が非 0 の行があれば解なし
    for row in &a {
        if row[..変数数].iter().all(|&v| v == 0) && row[変数数] != 0 {
            return None;
        }
    }
    let mut x = vec![0u64; 変数数];
    for (c, &pr) in ピボット行.iter().enumerate() {
        if pr != !0 {
            x[c] = a[pr][変数数];
        }
    }
    Some(x)
}

#[snippet("解の_個数_mod")]
#[snippet("mod_行列_ランク", include = "累乗_mod")]
#[snippet("RankMod")]
#[snippet("行列の_階数_mod")]
/// 素数 `法` 上での行列の**階数（ランク）**を掃き出しで求める。O(行数·列数·min(行,列))。
///
/// 行は長さがそろっていること。係数行列のランクは、線形独立な行（＝式）の本数や、
/// ベクトル集合の張る空間の次元を表す。`法` は**素数**。GF(2) 専用の速い版は
/// [`crate::gf2_elimination`]、解そのものは [`連立一次方程式_mod`]。
fn 行列の階数_mod(行列: &[Vec<u64>], 法: u64) -> usize {
    let 行数 = 行列.len();
    if 行数 == 0 {
        return 0;
    }
    let 列数 = 行列[0].len();
    let mut a: Vec<Vec<u64>> = 行列
        .iter()
        .map(|row| row.iter().map(|&x| x % 法).collect())
        .collect();
    let mut rank = 0usize;
    // c は列番号として複数の配列・範囲に使うため添字ループのまま扱う
    #[allow(clippy::needless_range_loop)]
    for c in 0..列数 {
        if rank == 行数 {
            break;
        }
        let Some(sel) = (rank..行数).find(|&i| a[i][c] != 0) else {
            continue;
        };
        a.swap(rank, sel);
        let inv = 累乗_mod(a[rank][c], 法 - 2, 法);
        for j in c..列数 {
            a[rank][j] = a[rank][j] * inv % 法;
        }
        for i in 0..行数 {
            if i != rank && a[i][c] != 0 {
                let f = a[i][c];
                for j in c..列数 {
                    a[i][j] = (a[i][j] + 法 - f * a[rank][j] % 法) % 法;
                }
            }
        }
        rank += 1;
    }
    rank
}

#[snippet("mod_行列_ランク", include = "累乗_mod")]
#[snippet("RankMod")]
#[snippet("解の_個数_mod")]
/// 素数 `法` 上の連立一次方程式 `A x = b` の解の**自由度**（自由変数の個数）を返す。
/// 矛盾（解なし）なら `None`。解の総数は `法^自由度`。O(行数·列数·min)。
///
/// 解を 1 つだけ欲しいときは [`連立一次方程式_mod`]、解の個数を数えたいときにこちらを使う。
fn 連立方程式の自由度_mod(係数: &[Vec<u64>], 右辺: &[u64], 法: u64) -> Option<usize> {
    let 変数数 = if 係数.is_empty() {
        0
    } else {
        係数[0].len()
    };
    let r = 行列の階数_mod(係数, 法);
    let 拡大: Vec<Vec<u64>> = 係数
        .iter()
        .zip(右辺)
        .map(|(row, &b)| {
            let mut row = row.clone();
            row.push(b);
            row
        })
        .collect();
    let ra = 行列の階数_mod(&拡大, 法);
    if ra > r {
        return None; // 矛盾
    }
    Some(変数数 - r)
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: u64 = 998_244_353;

    fn 検算(係数: &[Vec<u64>], 右辺: &[u64], x: &[u64], 法: u64) -> bool {
        係数.iter().zip(右辺).all(|(row, &b)| {
            let s = row
                .iter()
                .zip(x)
                .fold(0u64, |acc, (&a, &xi)| (acc + a % 法 * (xi % 法)) % 法);
            s == b % 法
        })
    }

    #[test]
    fn 一意解を持つ系() {
        // x + y = 3, x - y = 1 → x=2, y=1
        let a = vec![vec![1, 1], vec![1, P - 1]];
        let b = vec![3, 1];
        let x = 連立一次方程式_mod(&a, &b, P).unwrap();
        assert_eq!(x, vec![2, 1]);
        assert!(検算(&a, &b, &x, P));
    }

    #[test]
    fn 解なしを検出() {
        // x + y = 1, x + y = 2 は矛盾
        let a = vec![vec![1, 1], vec![1, 1]];
        let b = vec![1, 2];
        assert_eq!(連立一次方程式_mod(&a, &b, P), None);
    }

    #[test]
    fn 自由変数ありでも一解を返す() {
        // x + y = 5（y は自由）→ 何らかの解を返し検算が通る
        let a = vec![vec![1, 1]];
        let b = vec![5];
        let x = 連立一次方程式_mod(&a, &b, P).unwrap();
        assert!(検算(&a, &b, &x, P));
    }

    #[test]
    fn 小さい法で総当たり一致() {
        let 法 = 7u64;
        // ランダムっぽい固定の系をいくつか、全 x を総当たりして存在判定と突き合わせ
        let 系一覧: Vec<(Vec<Vec<u64>>, Vec<u64>)> = vec![
            (vec![vec![2, 3], vec![1, 1]], vec![1, 4]),
            (vec![vec![2, 4], vec![1, 2]], vec![3, 5]), // 平行 → 解なしのはず
            (vec![vec![2, 4], vec![1, 2]], vec![6, 3]), // 平行だが整合 → 解あり
            (vec![vec![1, 0, 1], vec![0, 1, 1]], vec![2, 3]),
        ];
        for (a, b) in 系一覧 {
            let 変数数 = a[0].len();
            // 総当たりで解の存在を判定
            let mut 存在 = false;
            let mut idx = vec![0u64; 変数数];
            'outer: loop {
                if 検算(&a, &b, &idx, 法) {
                    存在 = true;
                    break;
                }
                // 多進カウンタ
                let mut k = 0;
                loop {
                    if k == 変数数 {
                        break 'outer;
                    }
                    idx[k] += 1;
                    if idx[k] < 法 {
                        break;
                    }
                    idx[k] = 0;
                    k += 1;
                }
            }
            let 結果 = 連立一次方程式_mod(&a, &b, 法);
            assert_eq!(結果.is_some(), 存在, "a={a:?}, b={b:?}");
            if let Some(x) = 結果 {
                assert!(検算(&a, &b, &x, 法), "返した解が不正: x={x:?}");
            }
        }
    }

    #[test]
    fn 階数の既知例() {
        // 単位行列 3×3 → 3。
        assert_eq!(
            行列の階数_mod(&[vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]], P),
            3
        );
        // 2 行目が 1 行目の 2 倍 → 階数 1。
        assert_eq!(行列の階数_mod(&[vec![1, 2, 3], vec![2, 4, 6]], P), 1);
        // 零行列 → 0。
        assert_eq!(行列の階数_mod(&[vec![0, 0], vec![0, 0]], P), 0);
        // 縦長で独立 2 本 → 2。
        assert_eq!(行列の階数_mod(&[vec![1, 0], vec![0, 1], vec![1, 1]], P), 2);
    }

    // 法 上で A x = b の解の総数を総当たりで数える。
    fn 解数総当たり(係数: &[Vec<u64>], 右辺: &[u64], 法: u64) -> u64 {
        let 変数数 = if 係数.is_empty() {
            0
        } else {
            係数[0].len()
        };
        let mut 個数 = 0u64;
        let mut idx = vec![0u64; 変数数];
        loop {
            if 検算(係数, 右辺, &idx, 法) {
                個数 += 1;
            }
            let mut k = 0;
            loop {
                if k == 変数数 {
                    return 個数;
                }
                idx[k] += 1;
                if idx[k] < 法 {
                    break;
                }
                idx[k] = 0;
                k += 1;
            }
        }
    }

    #[test]
    fn 自由度から解数が総当たりと一致() {
        let 法 = 5u64;
        let mut seed = 0x8d31_77a0u64;
        let mut 乱数 = || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            seed >> 33
        };
        for _ in 0..2000 {
            let 行数 = (乱数() as usize % 3) + 1; // 1..=3
            let 変数数 = (乱数() as usize % 3) + 1; // 1..=3
            let 係数: Vec<Vec<u64>> = (0..行数)
                .map(|_| (0..変数数).map(|_| 乱数() % 法).collect())
                .collect();
            let 右辺: Vec<u64> = (0..行数).map(|_| 乱数() % 法).collect();
            let 実数 = 解数総当たり(&係数, &右辺, 法);
            match 連立方程式の自由度_mod(&係数, &右辺, 法) {
                None => assert_eq!(実数, 0, "係数={係数:?} 右辺={右辺:?}"),
                Some(自由度) => {
                    assert_eq!(実数, 法.pow(自由度 as u32), "係数={係数:?} 右辺={右辺:?}");
                }
            }
        }
    }
}
