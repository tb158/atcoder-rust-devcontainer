use cargo_snippet::snippet;

#[snippet("GF2_掃き出し")]
#[snippet("XOR_連立_方程式")]
#[snippet("ビット_連立_方程式")]
#[snippet("xor_方程式")]
#[snippet("掃き出し_法_GF2")]
/// GF(2) 上の連立一次方程式 `A x = b` を掃き出し（ガウス消去）で解く。O(式数 · 変数数)。
///
/// 変数は `0..変数数`（`変数数 <= 63`）。各式は `(係数ビット集合, 右辺)` で表し、
/// 係数ビット集合の `k` ビット目が立っていれば変数 `x_k` を含む。右辺は 0/1。
///
/// 解が無ければ `None`。`Some(解)` は解の一つ（自由変数は 0 とおく）。
/// 実数の連立方程式は `nalgebra` を使うこと。これは XOR（mod 2）専用。
fn xor連立方程式(変数数: usize, 式: &[(u64, u8)]) -> Option<Vec<u8>> {
    let 右辺ビット = 1u64 << 変数数;
    // 拡大行列: 下位 変数数 ビットが係数、変数数 ビット目が右辺
    let mut 行: Vec<u64> = 式
        .iter()
        .map(|&(係数, b)| 係数 | ((b as u64 & 1) << 変数数))
        .collect();
    let mut ピボット行 = vec![!0; 変数数];
    let mut 行位置 = 0usize;
    // 列はビットシフト位置としても使うため添字ループのまま扱う
    #[allow(clippy::needless_range_loop)]
    for 列 in 0..変数数 {
        // この列のビットが立っている行を 行位置 以降から探す
        let Some(選択) = (行位置..行.len()).find(|&r| 行[r] >> 列 & 1 == 1) else {
            continue;
        };
        行.swap(行位置, 選択);
        let 基準 = 行[行位置];
        for (r, row) in 行.iter_mut().enumerate() {
            if r != 行位置 && *row >> 列 & 1 == 1 {
                *row ^= 基準;
            }
        }
        ピボット行[列] = 行位置;
        行位置 += 1;
    }
    // 係数が全 0 なのに右辺が 1 の行があれば矛盾
    if 行[行位置..].contains(&右辺ビット) {
        return None;
    }
    let mut 解 = vec![0u8; 変数数];
    for (列, &pr) in ピボット行.iter().enumerate() {
        if pr != !0 {
            解[列] = ((行[pr] >> 変数数) & 1) as u8;
        }
    }
    Some(解)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 検算(変数数: usize, 式: &[(u64, u8)], 解: &[u8]) -> bool {
        式.iter().all(|&(係数, b)| {
            let mut 和 = 0u8;
            for (k, &値) in 解.iter().enumerate().take(変数数) {
                if 係数 >> k & 1 == 1 {
                    和 ^= 値;
                }
            }
            和 == (b & 1)
        })
    }

    #[test]
    fn 可解な系() {
        // x0^x1=1, x1^x2=0, x0^x2=1
        let 式 = [(0b011u64, 1u8), (0b110, 0), (0b101, 1)];
        let 解 = xor連立方程式(3, &式).unwrap();
        assert!(検算(3, &式, &解));
    }

    #[test]
    fn 矛盾系は解なし() {
        // x0=0 かつ x0=1
        let 式 = [(0b001u64, 0u8), (0b001, 1)];
        assert_eq!(xor連立方程式(1, &式), None);
    }

    #[test]
    fn 自由変数あり() {
        // x0^x1=1 のみ（x2 は自由）
        let 式 = [(0b011u64, 1u8)];
        let 解 = xor連立方程式(3, &式).unwrap();
        assert!(検算(3, &式, &解));
    }
}
