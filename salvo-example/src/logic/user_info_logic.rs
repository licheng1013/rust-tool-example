use rbatis::{crud, impl_select_page};
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use crate::model::user_info::{UserInfo, UserInfoDto};
use crate::RB;
use common::util::page::{PageParam, PageResult};
use crate::middleware::error::AppResult;
use crate::util::result::{JsonResult, ok_data, ok_msg};

const TABLE_NAME: &str = "t_user_info";

crud!(UserInfo{},TABLE_NAME);
impl_select_page!(UserInfo{page(where_str:&str) => "${where_str}"},TABLE_NAME);

pub async fn list(page: PageParam, model: UserInfo) -> AppResult<JsonResult<PageResult<Vec<UserInfoDto>>>> {
    let condition = where_condition(model);
    println!("{condition:?}");
    let result = UserInfo::page(
        &mut RB.clone(),
        &PageRequest::new(page.page.unwrap_or(1)
                          , page.size.unwrap_or(10)), &condition).await.unwrap();

    // 记录转换为dto
    let mut list = vec![];
    for item in result.records {
        list.push(UserInfoDto::from(item));
    }
    Ok(ok_data(PageResult {
        total: result.total,
        list,
    }))
}

pub async fn update(model: UserInfo) -> AppResult<JsonResult<()>> {
    UserInfo::update_by_column(&mut RB.clone(), &model, "id").await.unwrap();
    Ok(ok_msg("修改成功".to_string()))
}

pub async fn delete(model: UserInfo) -> AppResult<JsonResult<()>> {
    UserInfo::delete_by_column(&mut RB.clone(), "id", model.id).await.unwrap();
    Ok(ok_msg("删除成功".to_string()))
}

pub async fn insert(mut model: UserInfo) -> AppResult<JsonResult<()>> {
    model.create_time = Some(DateTime::now());
    UserInfo::insert(&mut RB.clone(), &model).await.unwrap();
    Ok(ok_msg("插入成功".to_string()))
}

pub fn where_condition(model: UserInfo) -> String {
    let mut where_str = String::from("");
    if model.id != None {
        where_str.push_str(format!("and id = \"{}\"", model.id.unwrap()).as_str())
    }
    if model.name != None {
        where_str.push_str(format!("and name = \"{}\"", model.name.unwrap()).as_str())
    }
    if model.nickname != None {
        where_str.push_str(format!("and nickname = \"{}\"", model.nickname.unwrap()).as_str())
    }
    if model.create_time != None {
        where_str.push_str(format!("and create_time = \"{}\"", model.create_time.unwrap()).as_str())
    }
    if where_str.len() == 0 {
        return "".to_string();
    }
    where_str = where_str[3..where_str.len()].to_string();
    format!("where{}",where_str)
}