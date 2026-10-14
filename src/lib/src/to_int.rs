use cargo_snippet::snippet;

#[snippet("to_i64")]
#[snippet("i64に_変換")]
pub fn to_i64(x: impl TryInto<i64>) -> i64 {
    x.try_into().ok().expect("i64 に収まりません")
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
        assert_eq!(to_i64(5usize), 5);
        assert_eq!(to_i64(-5i32), -5);
        assert_eq!(to_i64(5u32), 5);
        assert_eq!(to_i64(i64::MIN), i64::MIN);
        assert_eq!(to_u64(5usize), 5);
        assert_eq!(to_u64(5i32), 5);
        assert_eq!(to_u64(u64::MAX), u64::MAX);
    }

    #[test]
    #[should_panic(expected = "i64 に収まりません")]
    fn i64に収まらない値はpanic() {
        to_i64(u64::MAX);
    }

    #[test]
    #[should_panic(expected = "u64 に収まりません")]
    fn 負の値はu64に変換できない() {
        to_u64(-1i32);
    }
}
