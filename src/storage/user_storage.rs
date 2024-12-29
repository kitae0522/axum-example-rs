use crate::models::user_model::{User, CreateUser, LoginUser};
use sqlx::SqlitePool;

pub struct UserStorage {
    db_pool: SqlitePool,
}

impl UserStorage {
    pub fn new(db_pool: SqlitePool) -> Self {
        UserStorage { db_pool }
    }

    pub async fn get_user_from_db(&self, email: String) -> User {
        let row = sqlx::query!("SELECT id, name, email, age, password FROM users WHERE email = $1", email)
            .fetch_one(&self.db_pool)
            .await
            .unwrap();
        
        User {
            id: row.id,
            name: row.name,
            age: row.age,
            email: row.email,
            password: row.password
        }
    }

    pub async fn create_user(&self, user: &CreateUser) {
        sqlx::query!("INSERT INTO users (name, age, email, password) VALUES ($1, $2, $3, $4)", user.name, user.age, user.email, user.password)
            .execute(&self.db_pool)
            .await
            .unwrap();
    }

    pub async fn login_user(&self, user: &LoginUser) -> User {
        let row = sqlx::query!("SELECT * FROM users WHERE email = $1 and password = $2", user.email, user.password)
            .fetch_one(&self.db_pool)
            .await
            .unwrap();

        User {
            id: row.id,
            name: row.name,
            age: row.age,
            email: row.email,
            password: row.password
        }
    }
}
