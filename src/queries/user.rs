use sqlx::SqlitePool;

use super::User;

pub async fn find_user_by_email(
    db_pool: &SqlitePool,
    email: String,
) {
    // SELECT * FROM users WHERE email=$1 AND is_deleted=FALSE
    /*
    let row = sqlx::query!("SELECT * FROM users WHERE email=$1 AND is_deleted=FALSE", email)
        .fetch_one(db_pool)
        .await
        .unwrap();

    User {
        id: row.id,
        email: row.email,
        password: row.password
    }
     */
}

pub async fn list_users(
    db_pool: &SqlitePool
) {
    // select * form users where is_deleted=false
}

pub async fn create_user(
    db_pool: &SqlitePool,
    email: String,
    hash_password: String,
    salt: String,
) {
    sqlx::query!("INSERT INTO users (email, hash_password, salt) VALUES ($1, $2, $3)", email, hash_password, salt)
        .execute(db_pool)
        .await
        .unwrap();
}

pub async fn create_user_profile() {
    // INSERT INTO users (name, handle, profile_pic_url, bio) VALUES ($1, $2, $2)
}

pub async fn update_user_by_id(

) {
    // UPDATE Users SET username = ?, email = ?, hashed_password = ?, bio = ?, profile_pic_url = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND is_deleted = FALSE;

}

pub async fn deactivated_user_by_id(

) {
    // UPDATE Users SET is_deleted = TRUE, deleted_at = CURRENT_TIMESTAMP WHERE id = ?;

}

pub async fn activated_user_by_id(

) {
    // UPDATE Users SET is_deleted = FALSE, deleted_at = NULL WHERE id = ?;

}

pub async fn delete_user_by_id(

) {
    // DELETE FROM users WHERE id = ?;
}