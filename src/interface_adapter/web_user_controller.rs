use crate::domain::user::Users;
struct WebUserController{
    usersInputBoundary: UsersInputBoundary,
}
impl WebUserController{
    fn create_user(user: &Users)->Result<(), String>{
        userInputBoundary.create(user)
    }
    fn update_user_title(user: &Users)->Result<(), String>{
        userInputBoundary.update_title(user.id, user.title)
    }
    fn update_user_content(user: &Users)->Result<(), String>{
        userInputBoundary.update_content(user.id, user.content)
    }
    fn delete_user(user: &Users)->Result<(), String>{
        userInputBoundary.delete(user.id)
    }
    fn get_all_user()->Result<Vec<Users>, String>{
        userInputBoundary.get_all()
    }
    fn get_user_by_id(id: String)->Result<Users, String>{
        userInputBoundary.get_by_id(id)
    }

}

