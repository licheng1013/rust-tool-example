
use std::cell::RefCell;
thread_local!(static USER_ID: RefCell<i64> = RefCell::new(0));

pub fn get_user_id() -> i64 {
    USER_ID.with(|f| *f.borrow())
}

pub fn set_user_id(value: i64) {
    USER_ID.with(|f| {
        *f.borrow_mut() = value;
    });
}

