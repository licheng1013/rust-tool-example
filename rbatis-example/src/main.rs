use rbatis::{crud, impl_select, RBatis};
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::datetime::DateTime;
use serde_json::json;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct User {
    /// 用户id
    pub id: Option<i32>,
    /// 用户昵称
    pub name: Option<String>,
    /// 手机号
    pub tel: Option<String>,
    /// 账号金额: 分
    pub money: Option<i64>,
    /// 创建时间
    pub create_time: Option<DateTime>,
}


crud!(User {},"t_user"); // impl_insert!($table {}) + impl_select!($table {}) + impl_update!($table {}) + impl_delete!($table {});

#[tokio::main]
pub async fn main() {
    let rb = RBatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:root@192.168.101.90/t_gorm").unwrap();

    let table = User {
        id: None,
        name: Some("2".into()),
        tel: Some("2".into()),
        money: Some(11),
        create_time: Some(DateTime::now()),
    };
    let tables = [table.clone(), {
        table.clone()
    }];

    let data = User::insert(&rb, &table).await;
    println!("insert = {}", json!(data));
    let data = User::insert_batch(&rb, &tables, tables.len().u64()).await;
    println!("insert_batch = {}", json!(data));
    let data = User::select_by_column(&rb, "name", 2).await;
    println!("select_in_column = {}", json!(data));
    let data = User::delete_by_column(&rb, "", 33).await;
    println!("delete_in_column = {}", json!(data));
    let v = User::select_by_column(&rb,"","");
    User::select_by_column(&rb,"U","");
}
