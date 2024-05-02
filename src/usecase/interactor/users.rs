use crate::usecase::data_access::UserDataAccess;

struct input_user {
    userDataAccess: UserDataAccess,
}

impl UserInputBoundary for input_user {
    fn create(&self,user: User) -> Result<(), String> {
        self.userDataAccess.create(user)
    }
    
    fn update_name(&self, name: String) -> Result<(), String> {
        self.userDataAccess.update_name(name)
    }

    fn delete(&self, id: String) -> Result<(), String> {
        self.userDataAccess.delete(id)
    }

    fn get_all(&self) -> Result<Vec<User>, String> {
        self.userDataAccess.get_all()
    }

    fn get_by_id(&self, id: String) -> Result<User, String> {
        self.userDataAccess.get_by_id(id)
    }

}