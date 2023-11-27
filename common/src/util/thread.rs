
use std::cell::RefCell;
thread_local!(static USER_ID: RefCell<i64> = RefCell::new(0));


pub fn get_user_id() -> i64 {
    let user_id = USER_ID.with(|f| *f.borrow());
    if user_id == 0 {
        println!("{:?}", "非登入上下文中");
    }
    user_id
}

/// 不适合使用，因为在框架中，可能存在一个线程处理多个请求的情况，从而导致user_id被覆盖，会导致错误
pub fn set_user_id(value: i64) {
    USER_ID.with(|f| {
        *f.borrow_mut() = value;
    });
}

