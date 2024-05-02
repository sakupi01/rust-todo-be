mod domain;
use crate::domain::todo::Todo;

pub trait TodoDataAccess {
    fn create(Todo) -> i32;
}
