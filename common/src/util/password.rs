pub struct PasswdUtil {}

impl PasswdUtil {
    /// 获取密码, 密钥加盐
    pub fn password(password: &str, salt: &str) -> String {
        let pd = format!("{}{}", password, salt);
        return format!("{:x}", md5::compute(pd))
    }
}

#[cfg(test)]
mod tests {
    use crate::util::password::PasswdUtil;

    #[test]
    fn test_empty_str() {
        let pd = PasswdUtil::password("123", "123");
        assert_eq!(pd, "4297f44b13955235245b2497399d7a93")
    }
}