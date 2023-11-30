use rbatis::async_trait;
use salvo::{Depot, Request, Response, Writer};
use salvo::prelude::Text::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResult<T>{
    pub code: i8,
    pub msg: String,
    pub data: T,
}

impl <T:Serialize>JsonResult<T> {
    pub fn to_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}


/// 正确消息返回
pub fn ok_msg(msg: String) -> JsonResult<()> {
    let r = JsonResult {
        code: 0,
        msg,
        data: (),
    };
    return r;
}

/// 正确结果返回
pub fn ok_data<T>(data: T) -> JsonResult<T> {
    let r = JsonResult {
        code: 0,
        msg: "".to_string(),
        data,
    };
    return r;
}

pub fn fail(msg: String) -> JsonResult<()> {
    let r = JsonResult {
        code: -1,
        msg,
        data: (),
    };
    return r;
}

#[async_trait]
impl<T: Send + Serialize> Writer for JsonResult<T> {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(self.to_string()));
    }
}



