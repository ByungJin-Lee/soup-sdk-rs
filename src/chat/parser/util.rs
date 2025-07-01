pub fn is(flags: u32, flag: u32) -> bool {
    flags & flag == flag
}

#[inline]
pub fn normalize_user_id(user_id: &str) -> String {
    let len = user_id.len();
    if len >= 3 {
        let bytes = user_id.as_bytes();
        // 가장 빠른 검사부터 수행: 마지막 문자 → 숫자 확인 → 괄호 확인
        if bytes[len - 1] == b')' && bytes[len - 2].is_ascii_digit() && bytes[len - 3] == b'(' {
            return user_id[..len - 3].to_string();
        }
    }
    user_id.to_string()
}

#[inline]
pub fn parse_u32_or_default(s: &str) -> u32 {
    s.parse::<u32>().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_user_id() {
        assert_eq!(normalize_user_id("bemong"), "bemong");
        assert_eq!(normalize_user_id("bemong(1)"), "bemong");
        assert_eq!(normalize_user_id("bemong(2)"), "bemong");
        assert_eq!(normalize_user_id("bemong(9)"), "bemong");
        assert_eq!(normalize_user_id("user_name(5)"), "user_name");
        assert_eq!(normalize_user_id("test(a)"), "test(a)");
        assert_eq!(normalize_user_id("test()"), "test()");
        assert_eq!(normalize_user_id("ab"), "ab");
        assert_eq!(normalize_user_id(""), "");
    }
}
