use crate::usecase::data_access::UsersDataAccess;

struct input_users {
    usersDataAccess: UsersDataAccess,
}

impl UsersInputBoundary for input_users {
    fn create(&self,users: Users) -> Result<(), String> {
        self.usersDataAccess.create(users)
    }
    
    fn update_name(&self, name: String) -> Result<(), String> {
        self.usersDataAccess.update_name(name)
    }

    fn delete(&self, id: String) -> Result<(), String> {
        self.usersDataAccess.delete(id)
    }

    fn get_all(&self) -> Result<Vec<Users>, String> {
        self.usersDataAccess.get_all()
    }

    fn get_by_id(&self, id: String) -> Result<Users, String> {
        self.usersDataAccess.get_by_id(id)
    }

}