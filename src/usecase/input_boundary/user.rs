use crate::usecase::output_data::user::User;

pub trait UserInputBoundary {
    fn create(&self, user: User) -> Result<(), String>;
    fn update_name(&self, id: String, name: String) -> Result<(), String>;
    fn delete(&self, id: String) -> Result<(), String>;
    fn get_all(&self) -> Result<Vec<User>, String>;
    fn get_by_id(&self, id: String) -> Result<User, String>;
}
