use sqlx::SqlitePool;
use sqlx::Error;

use super::User;

pub async fn find_user_by_email(
    db_pool: &SqlitePool,
    email: String,
) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1 AND is_deleted = FALSE")
        .bind(email)
        .fetch_optional(db_pool)
        .await?;

    match user {
        Some(u) => Ok(u),
        None => Err(Error::RowNotFound),
    }
}

pub async fn list_users(
    db_pool: &SqlitePool
) -> Result<Vec<User>, Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users WHERE is_deleted = FALSE")
        .fetch_all(db_pool)
        .await?;

    return Ok(users);
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

pub async fn update_user_profile(
    db_pool: &SqlitePool,
    id: String,
    name: String,
    handle: String,
    profile_pic_url: String,
    bio: String
) -> Result<User, Error> {
    let affected_rows = sqlx::query!(
        "UPDATE users SET name = $1, handle = $2,  profile_pic_url = $3, bio = $4, is_profile_complete = TRUE WHERE id = $5",
        name, handle, profile_pic_url, bio, id
    )
        .execute(db_pool)
        .await
        .unwrap()
        .rows_affected();

    if affected_rows >= 1 {
        let user = sqlx::query_as::<_, User>("SELECT id, name, handle, profile_pic_url, bio FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(db_pool)
            .await?;
        Ok(user)
    } else {
        Err(Error::RowNotFound)
    }
}

pub async fn deactivated_user_by_id(
    db_pool: &SqlitePool,
    id: String,
) {
    sqlx::query!("UPDATE users SET is_deleted = TRUE, deleted_at = CURRENT_TIMESTAMP WHERE id = $1", id)
        .execute(db_pool)
        .await
        .unwrap();
}

pub async fn activated_user_by_id(
    db_pool: &SqlitePool,
    id: String,
) {
    sqlx::query!("UPDATE users SET is_deleted = FALSE, deleted_at = NULL WHERE id = $1", id)
        .execute(db_pool)
        .await
        .unwrap();
}

pub async fn delete_user_by_id(
    db_pool: &SqlitePool,
    id: String,
) {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(db_pool)
        .await
        .unwrap();
}