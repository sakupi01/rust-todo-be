use std::sync::{Mutex, OnceLock};

use super::ram_zatsu_user_db::RamZatsuUserDb;

// global mutable variable
static USER_DB: OnceLock<Mutex<RamZatsuUserDb>> = OnceLock::new();

pub fn get_user_db() -> &'static Mutex<RamZatsuUserDb> {
    USER_DB.get_or_init(|| Mutex::new(RamZatsuUserDb::new()))
}
