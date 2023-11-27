use rbatis::{crud, impl_select, impl_select_page};
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use serde_json::{Map, Value};
use common::util::jwt::JwtUtil;
use common::util::page::{PageParam, PageResult};
use common::util::password::PasswdUtil;
use crate::middleware::error::{AppError, AppResult};
use crate::model::admin::{Admin, AdminDto};
use crate::RB;
use crate::util::assert::As;


const TABLE_NAME: &str = "t_admin";

crud!(Admin{},TABLE_NAME);
impl_select_page!(Admin{page(where_str:&str) => "${where_str}"},TABLE_NAME);
impl_select!(Admin{select_by_user_name(val:&str) -> Option => "`where user_name = #{val} limit 1`"},TABLE_NAME);
impl_select!(Admin{select_by_id(val:i64) -> Option => "`where id = #{val} limit 1`"},TABLE_NAME);

//@Rbatis(Admin)

pub async fn list(page: PageParam, model: Admin) -> PageResult<Vec<AdminDto>> {
    let condition = where_condition(model);
    let result = Admin::page(
        &mut RB.clone(),
        &PageRequest::new(page.page.unwrap_or(1)
                          , page.size.unwrap_or(10)), &condition).await.unwrap();

    // 记录转换为dto
    let mut list = vec![];
    for item in result.records {
        list.push(AdminDto::from(item));
    }
    return PageResult {
        total: result.total,
        list,
    };
}

pub async fn update(model: Admin) {
    let result = Admin::update_by_column(&mut RB.clone()
                                         , &model, "id").await.unwrap();
    println!("{result:?}")
}

pub async fn delete(ids: Vec<i64>) {
    if true {
        println!("{:?}", "不允许批量删除-或数值为0");
        return;
    }
    let id = ids[0];
    Admin::delete_by_column(&mut RB.clone(), "id", id).await.unwrap();
}

pub async fn insert(mut model: Admin) {
    model.create_time = Some(DateTime::now());
    let result = Admin::insert(&mut RB.clone(), &model).await.unwrap();
    println!("{result:?}")
}

pub fn where_condition(model: Admin) -> String {
    let mut where_str = String::from("");
    if model.id != None {
        where_str.push_str(format!("and id = \"{}\"", model.id.unwrap()).as_str())
    }
    if model.nick_name != None {
        where_str.push_str(format!("and nick_name = \"{}\"", model.nick_name.unwrap()).as_str())
    }
    if model.user_name != None {
        where_str.push_str(format!("and user_name = \"{}\"", model.user_name.unwrap()).as_str())
    }
    if model.salt != None {
        where_str.push_str(format!("and salt = \"{}\"", model.salt.unwrap()).as_str())
    }
    if model.password != None {
        where_str.push_str(format!("and password = \"{}\"", model.password.unwrap()).as_str())
    }
    if model.create_time != None {
        where_str.push_str(format!("and create_time = \"{}\"", model.create_time.unwrap()).as_str())
    }
    if where_str.len() == 0 {
        return "".to_string();
    }
    where_str = where_str[3..where_str.len()].to_string();
    return format!("where{}", where_str);
}


pub async fn login(admin: Admin) -> AppResult<Map<String, Value>> {
    println!("login = {:?}", admin);
    let err_msg = "账号或密码错误";
    As::in_range_str(admin.user_name.clone(), 3, 12, "账号长度不正确")?;
    As::in_range_str(admin.password.clone(), 3, 12, "密码长度不正确")?;
    let data = Admin::select_by_user_name(&mut RB.clone(), &admin.user_name.unwrap()).await.unwrap();
    As::in_none(data.clone(),err_msg)?;
    let one = data.unwrap();
    let passwd = PasswdUtil::password(admin.password.unwrap().as_str(), one.salt.unwrap().as_str());
    As::is_true(passwd != one.password.unwrap(),err_msg)?;
    // 构建一个map结构
    let mut map = serde_json::Map::new();
    map.insert("token".to_string(), JwtUtil::token(one.id.unwrap()).into());
    return Ok(map);
}

pub(crate) async fn get(user_id: i64) -> Option<Admin> {
    let data = Admin::select_by_id(&mut RB.clone(), user_id).await.unwrap();
    return data;
}


pub(crate) async fn user_info(admin: Admin) -> Map<String, Value> {
    // 角色map
    let mut roles = serde_json::Map::new();
    roles.insert("roleName".to_string(), 1.into());
    roles.insert("value".to_string(), "管理员".into());

    // 构建一个map结构
    let mut map = serde_json::Map::new();
    map.insert("realName".to_string(), "管理员".into());
    map.insert("userName".to_string(), "管理员".into());
    map.insert("userId".to_string(), admin.id.unwrap().into());
    map.insert("roles".to_string(), vec![roles].into());
    return map;
}