use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    /// 主键
    pub id: Option<i64>,
    /// 名称
    pub name: Option<String>,
    /// 昵称
    pub nickname: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTime>,
}


/// DTO 对象 - 由插件生成。不应该直接更改。
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoDto{
    /// 主键
    pub id: Option<i64>,
    /// 名称
    pub name: Option<String>,
    /// 昵称
    pub nickname: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTime>,
}

impl From<UserInfo> for UserInfoDto{
    fn from(dto: UserInfo) -> Self {
        UserInfoDto{
            id:dto.id,
            name:dto.name,
            nickname:dto.nickname,
            create_time:dto.create_time,
        }
    }
}

impl From<UserInfoDto> for UserInfo{
    fn from(dto: UserInfoDto) -> Self {
        UserInfo{
            id:dto.id,
            name:dto.name,
            nickname:dto.nickname,
            create_time:dto.create_time,
        }
    }
}