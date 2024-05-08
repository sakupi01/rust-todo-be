use crate::domain::user::User;
use crate::usecase::input_boundary::user::UserInputBoundary;

pub struct WebUserController<T> {
    pub userInputBoundary: T,
}
impl<T> WebUserController<T>
where
    T: UserInputBoundary,
{
    pub fn create_user(&mut self, user: &User) -> Result<(), String> {
        self.userInputBoundary.create(user.clone())
    }
    pub fn update_user_name(&mut self, user: &User) -> Result<(), String> {
        self.userInputBoundary
            .update_name(user.id.clone(), user.name.clone())
    }
    pub fn delete_user(&mut self, user: &User) -> Result<(), String> {
        self.userInputBoundary.delete(user.id.clone())
    }
    pub fn get_all_user(&self) -> Result<Vec<User>, String> {
        self.userInputBoundary.get_all()
    }
    pub fn get_user_by_id(&self, id: String) -> Result<User, String> {
        self.userInputBoundary.get_by_id(id)
    }
}
