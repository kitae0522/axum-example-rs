use serde::{Deserialize, Serialize};

pub mod login;
pub mod register;

#[derive(Deserialize, Serialize)]
pub struct RequestLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseLogin {
    pub id: String,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct RequestRegister {
    pub email: String,
    pub password: String,
    pub password_confirm: String
}

#[derive(Deserialize, Serialize)]
pub struct ResponseRegister {
    
}