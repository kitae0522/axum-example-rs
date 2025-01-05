use serde::{Deserialize, Serialize};

pub mod user;
pub mod thread;
pub mod follow;
pub mod like;
pub mod view;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub hash_password: String,
    pub salt: String,
    pub name: Option<String>,
    pub handle: Option<String>,
    pub profile_pic_url: Option<String>,
    pub bio: Option<String>,
    pub is_profile_complete: bool,
    pub is_deleted: bool,
    pub deleted_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct Thread {
    pub id: i64,
    pub user_id: String,
    pub title: Option<String>,
    pub content: String,
    pub parent_thread: Option<i64>,
    pub is_deleted: bool,
    pub deleted_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct Follow {
    pub user_id: String,
    pub follower_id: String,
    pub created_at: String,   
}

#[derive(Deserialize, Serialize)]
pub struct Like {
    pub user_id: String,
    pub thread_id: String,
    pub is_deleted: bool,
    pub deleted_at: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct View {
    pub thread_id: String,
    pub views: i64,
}