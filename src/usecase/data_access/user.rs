use crate::domain::user::User;

pub trait UserDataAccess {
    fn create(&self, user: User) -> Result<(), String>;
    fn update_name(&self, name: String) -> Result<(), String>;
    fn delete(&self, id: String) -> Result<(), String>;
    fn get_all(&self) -> Result<Vec<User>, String>;
    fn get_by_id(&self, id: String) -> Result<User, String>;
}
