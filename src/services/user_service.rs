use crate::storage::user_storage::UserStorage;
use crate::models::user_model::{CreateUser, LoginUser, User};

pub struct UserService {
    storage: UserStorage,
}

impl UserService {
    pub fn new(storage: UserStorage) -> Self {
        UserService { storage }
    }

    pub async fn get_user(&self, email: String) -> User {
        self.storage.get_user_from_db(email).await
    }

    pub async fn create_user(&self, user: &CreateUser) {
        self.storage.create_user(user).await;
    }

    pub async fn login_user(&self, user: &LoginUser) -> User {
        self.storage.login_user(user).await
    }
}
