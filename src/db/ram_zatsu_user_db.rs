use std::collections::HashMap;

use crate::domain::user::User;
use crate::usecase::data_access::user::UserDataAccess;

pub struct RamZatsuUserDb {
    db: HashMap<String, User>,
}

impl RamZatsuUserDb {
    pub fn new() -> RamZatsuUserDb {
        RamZatsuUserDb { db: HashMap::new() }
    }
}

impl UserDataAccess for RamZatsuUserDb {
    fn create(&mut self, user: User) -> Result<(), String> {
        if self.db.insert(user.id.clone(), user).is_none() {
            Ok(())
        } else {
            Err("fail to insert user".to_string())
        }
    }
    fn update_name(&mut self, id: String, name: String) -> Result<(), String> {
        if let Some(x) = self.db.get_mut(&id) {
            x.name = name;
            Ok(())
        } else {
            Err("fail to update name".to_string())
        }
    }
    fn delete(&mut self, id: String) -> Result<(), String> {
        if self.db.remove(&id).is_some() {
            Ok(())
        } else {
            Err("fail to delete user".to_string())
        }
    }
    fn get_all(&self) -> Result<Vec<User>, String> {
        Ok(self.db.values().cloned().collect())
    }
    fn get_by_id(&self, id: String) -> Result<User, String> {
        if let Some(x) = self.db.get(&id) {
            Ok(x.clone())
        } else {
            Err("not found".to_string())
        }
    }
}
