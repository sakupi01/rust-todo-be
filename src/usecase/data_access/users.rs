mod domain;
use crate::domain::users::Users;

pub trait UsersDataAccess {
    fn create(&self, users: Users) -> Result<(), String>;
    fn update(&self, users: Users) -> Result<(), String>;
    fn delete(&self, id: String) -> Result<(), String>;
    fn get_all(&self) -> Result<Vec<Users>, String>;
    fn get_by_id(&self, id: String) -> Result<Users, String>;
}