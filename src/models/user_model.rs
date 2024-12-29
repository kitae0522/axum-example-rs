use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub age: i64,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    pub name: String,
    pub age: i64,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String
}