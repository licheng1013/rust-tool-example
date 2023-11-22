pub struct As {}


/// 这里是断言工具
impl As {

    /// 空字符串验证
    pub fn empty_str(msg: Option<String>, err_msg: &str) {
        if msg.clone() == None || msg.unwrap().len() == 0 {
            panic!("{}", err_msg);
        }
    }
    /// 空数字，0也算空
    pub fn empty_num(msg: Option<i64>, err_msg: &str) {
        if msg.clone() == None || msg.unwrap() == 0 {
            panic!("{}", err_msg);
        }
    }

    /// 判断字符串是不是大范围内,不包含边界. 示例: 1,10, n < 1 或 n > 10 都会报错, 1-10个长度之间不会报错
    pub fn in_range_str(msg: Option<String>, min: i64, max: i64, err_msg: &str) {
        if msg.clone() == None || msg.clone().unwrap().len() < min as usize || msg.unwrap().len() > max as usize {
            panic!("{}", err_msg);
        }
    }

    /// 直接抛出错误
    pub fn error(err_msg: &str) {
            panic!("{}", err_msg);
    }
}

// 编写测试

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_str() {
        As::empty_str(Some("1".to_string()),"为空");
    }

    #[test]
    fn test_empty_num() {
        As::empty_num(Some(11),"数字空");
    }

    #[test]
    fn test_in_range_str() {
        As::in_range_str(Some("1".to_string()), 1, 10,"范围错误");
    }
}