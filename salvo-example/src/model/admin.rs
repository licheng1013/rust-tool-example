use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Admin {
    /// 管理员id
    pub id: Option<i64>,
    /// 账号
    pub user_name: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 盐
    pub salt: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTime>,
    /// 昵称
    pub nick_name: Option<String>,
}

/// DTO 对象 - 由插件生成。不应该直接更改。
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq,Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdminDto {
    /// 管理员id
    pub id: Option<i64>,
    /// 账号
    #[validate(length(min = 3,max = 16, message  = "账号长度为3-16位"))]
    pub user_name: Option<String>,
    /// 密码
    #[validate(length(min = 3,max = 16, message  = "密码长度为3-16位"))]
    pub password: Option<String>,
    /// 盐
    pub salt: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTime>,
    /// 昵称
    pub nick_name: Option<String>,
}

impl From<Admin> for AdminDto {
    fn from(dto: Admin) -> Self {
        AdminDto {
            id: dto.id,
            user_name: dto.user_name,
            password: dto.password,
            salt: dto.salt,
            create_time: dto.create_time,
            nick_name: dto.nick_name,
        }
    }
}

impl From<AdminDto> for Admin {
    fn from(dto: AdminDto) -> Self {
        Admin {
            id: dto.id,
            user_name: dto.user_name,
            password: dto.password,
            salt: dto.salt,
            create_time: dto.create_time,
            nick_name: dto.nick_name,
        }
    }
}