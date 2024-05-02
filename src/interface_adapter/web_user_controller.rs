use crate::domain::user::User;
struct WebUserController{
    userInputBoundary: UserInputBoundary,
}
impl WebUserController{
    fn create_user(user: &User)->Result<(), String>{
        userInputBoundary.create(user)
    }
    fn update_user_title(user: &User)->Result<(), String>{
        userInputBoundary.update_title(user.id, user.title)
    }
    fn update_user_content(user: &User)->Result<(), String>{
        userInputBoundary.update_content(user.id, user.content)
    }
    fn delete_user(user: &User)->Result<(), String>{
        userInputBoundary.delete(user.id)
    }
    fn get_all_user()->Result<Vec<User>, String>{
        userInputBoundary.get_all()
    }
    fn get_user_by_id(id: String)->Result<User, String>{
        userInputBoundary.get_by_id(id)
    }

}

