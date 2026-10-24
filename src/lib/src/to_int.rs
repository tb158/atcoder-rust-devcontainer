use cargo_snippet::snippet;

#[snippet("to_isize")]
#[snippet("isizeに_変換")]
pub fn to_isize(x: impl TryInto<isize>) -> isize {
    x.try_into().ok().expect("isize に収まりません")
}

#[snippet("to_u64")]
#[snippet("u64に_変換")]
pub fn to_u64(x: impl TryInto<u64>) -> u64 {
    x.try_into().ok().expect("u64 に収まりません")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 各種の整数型を受け取れる() {
        assert_eq!(to_isize(5usize), 5);
        assert_eq!(to_isize(-5i32), -5);
        assert_eq!(to_isize(5u32), 5);
        assert_eq!(to_isize(isize::MIN), isize::MIN);
        assert_eq!(to_u64(5usize), 5);
        assert_eq!(to_u64(5i32), 5);
        assert_eq!(to_u64(u64::MAX), u64::MAX);
    }

    #[test]
    #[should_panic(expected = "isize に収まりません")]
    fn isizeに収まらない値はpanic() {
        to_isize(u64::MAX);
    }

    #[test]
    #[should_panic(expected = "u64 に収まりません")]
    fn 負の値はu64に変換できない() {
        to_u64(-1i32);
    }
}
