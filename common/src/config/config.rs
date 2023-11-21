use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    /// 服务端口
    pub port: u16,
    /// mysql连接地址
    pub mysql_url: String,
    /// 排除验证路径
    pub exclude_path: Option<Vec<String>>,
}

/// 配置文件
impl AppConfig {
    /// 读取配置文件
    pub fn new() -> AppConfig {
        let yaml = std::fs::read_to_string("config.yml").unwrap();
        let app: AppConfig = serde_yaml::from_str(&yaml).unwrap();
        return app;
    }
    /// 传入一个路径，判断是否需要验证
    pub fn is_exclude(&self, path: &str) -> bool {
        if let Some(exclude_path) = &self.exclude_path {
            for p in exclude_path {
                // 不接受模糊匹配
                if p == path {
                    return true;
                }
            }
        }
        // 接受模糊匹配
        if let Some(exclude_path) = &self.exclude_path {
            for p in exclude_path {
                if p.contains("*") {
                    let p = p.replace("*", "");
                    if path.starts_with(p.as_str()) {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}


#[cfg(test)]
mod tests {
    use crate::config::config::AppConfig;

    #[test]
    fn config() {
        // 测试模糊匹配
        let exclude_path = vec![
            "/api/v1/file/*".to_string(),
            "/api/v1/admin/".to_string(),
        ];
        let app = AppConfig {
            port: 8080,
            mysql_url: "mysql://root:123456@localhost:3306/test".to_string(),
            exclude_path: Some(exclude_path),
        };
        assert_eq!(app.is_exclude("/api/v1/file/upload"), true);
        assert_eq!(app.is_exclude("/api/v1/admin/"), true);
        assert_eq!(app.is_exclude("/api/v1/admin/xx"), false);
    }
}
