use crate::domain::user::User;
use crate::usecase::input_boundary::user::UserInputBoundary;

struct WebUserController {
    userInputBoundary: dyn UserInputBoundary,
}
impl WebUserController {
    fn create_user(&self, user: &User) -> Result<(), String> {
        self.userInputBoundary.create(user.clone())
    }
    fn update_user_name(&self, user: &User) -> Result<(), String> {
        self.userInputBoundary
            .update_name(user.id.clone(), user.name.clone())
    }
    fn delete_user(&self, user: &User) -> Result<(), String> {
        self.userInputBoundary.delete(user.id.clone())
    }
    fn get_all_user(&self) -> Result<Vec<User>, String> {
        self.userInputBoundary.get_all()
    }
    fn get_user_by_id(&self, id: String) -> Result<User, String> {
        self.userInputBoundary.get_by_id(id)
    }
}
