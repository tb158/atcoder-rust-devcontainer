use cargo_snippet::snippet;

#[snippet("スライド_最小値")]
#[snippet("窓の_最小値")]
#[snippet("単調_デック")]
#[snippet("スライディング_ウィンドウ_最小")]
/// スライド最小値。長さ `窓` の連続する各区間の最小値を、単調デックで O(N) に列挙する。
/// 戻り値の長さは `a.len() + 1 - 窓`（`窓 == 0` または `窓 > a.len()` のとき空）。
/// 戻り値 `res[i]` は区間 `a[i..i+窓]` の最小値。
fn スライド最小値<T: Ord + Copy>(a: &[T], 窓: usize) -> Vec<T> {
    use std::collections::VecDeque;
    let mut res = Vec::new();
    if 窓 == 0 || 窓 > a.len() {
        return res;
    }
    let mut dq: VecDeque<usize> = VecDeque::new(); // 値が単調増加になる添字列
    for (i, &x) in a.iter().enumerate() {
        while let Some(&後) = dq.back() {
            if a[後] >= x {
                dq.pop_back();
            } else {
                break;
            }
        }
        dq.push_back(i);
        if i + 1 >= 窓 {
            let 左端 = i + 1 - 窓;
            while let Some(&先) = dq.front() {
                if 先 < 左端 {
                    dq.pop_front();
                } else {
                    break;
                }
            }
            res.push(a[*dq.front().unwrap()]);
        }
    }
    res
}

#[snippet("スライド_最大値")]
#[snippet("窓の_最大値")]
#[snippet("単調_デック")]
#[snippet("スライディング_ウィンドウ_最大")]
/// スライド最大値。長さ `窓` の連続する各区間の最大値を単調デックで O(N) に列挙する。
/// 仕様は [`スライド最小値`] と同じで、デック内が単調減少になる点だけが異なる。
fn スライド最大値<T: Ord + Copy>(a: &[T], 窓: usize) -> Vec<T> {
    use std::collections::VecDeque;
    let mut res = Vec::new();
    if 窓 == 0 || 窓 > a.len() {
        return res;
    }
    let mut dq: VecDeque<usize> = VecDeque::new();
    for (i, &x) in a.iter().enumerate() {
        while let Some(&後) = dq.back() {
            if a[後] <= x {
                dq.pop_back();
            } else {
                break;
            }
        }
        dq.push_back(i);
        if i + 1 >= 窓 {
            let 左端 = i + 1 - 窓;
            while let Some(&先) = dq.front() {
                if 先 < 左端 {
                    dq.pop_front();
                } else {
                    break;
                }
            }
            res.push(a[*dq.front().unwrap()]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 窓3の最小最大() {
        let a = [4, 2, 5, 1, 3, 6, 2];
        // 区間: [4,2,5],[2,5,1],[5,1,3],[1,3,6],[3,6,2]
        assert_eq!(スライド最小値(&a, 3), vec![2, 1, 1, 1, 2]);
        assert_eq!(スライド最大値(&a, 3), vec![5, 5, 5, 6, 6]);
    }

    #[test]
    fn 窓が長さと等しい() {
        let a = [3, 1, 2];
        assert_eq!(スライド最小値(&a, 3), vec![1]);
        assert_eq!(スライド最大値(&a, 3), vec![3]);
    }

    #[test]
    fn 窓が大きすぎる場合は空() {
        let a = [1, 2];
        assert!(スライド最小値(&a, 5).is_empty());
    }
}
