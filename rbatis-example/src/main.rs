use rbatis::executor::Executor;
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::datetime::DateTime;
use rbatis::{crud, html_sql, RBatis};
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

#[html_sql("rbatis-example/example.html")]
async fn select_by_condition2(rb: &dyn Executor, user: User) -> Vec<User> {
    impled!()
}

#[html_sql("rbatis-example/example.html")]
async fn select_by_condition3(rb: &dyn Executor, name: &str) -> Vec<User> {
    impled!()
}


#[html_sql(
    r#"<select id="select_by_condition">
        `select * from t_user`
        <where>
             ` and tel = #{user.tel}`
        </where>
  </select>"#
)]
async fn select_by_condition(rb: &dyn Executor, user: User) -> rbatis::Result<Vec<User>> {
    impled!()
}

crud!(User {}, "t_user"); 

#[tokio::main]
pub async fn main() {
    let rb = RBatis::new();
    rb.init(
        rbdc_mysql::driver::MysqlDriver {},
        "mysql://root:root@192.168.101.90/t_gorm",
    )
    .unwrap();

    let table = User {
        id: None,
        name: Some("2".into()),
        tel: Some("2".into()),
        money: Some(11),
        create_time: Some(DateTime::now()),
    };
    let mut k = table.clone();
    k.tel = Some("3".into());
    let tables = [table.clone(), k.clone()];

    let data = User::insert(&rb, &table).await;
    println!("insert = {}", json!(data));
    let data = User::insert_batch(&rb, &tables, tables.len().u64()).await;
    println!("insert_batch = {}", json!(data));
    let data = User::select_by_column(&rb, "name", 2).await;
    println!("select_in_column = {}", json!(data));
    let data = select_by_condition(&rb, table.clone()).await;
    println!("select_by_condition = {}", json!(data));
    let data = select_by_condition2(&rb, k.clone()).await;
    println!("select_by_condition2 = {}", json!(data));
    let data = select_by_condition3(&rb, "3").await;
    println!("select_by_condition3 = {}", json!(data));
    let data = User::delete_by_column(&rb, "name", 2).await;
    println!("delete_in_column = {}", json!(data));
}
