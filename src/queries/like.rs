use sqlx::SqlitePool;

use super::Like;

pub async fn like_thread() {
    // INSERT INTO likes (user_id, thread_id) VALUES (?, ?);
}

pub async fn unlike_thread() {
    // UPDATE likes SET is_deleted = TRUE, deleted_at = CURRENT_TIMESTAMP WHERE id = ?;

}

pub async fn list_likes_thread() {
    // SELECT * FROM likes WHERE user_id = ? AND is_deleted = FALSE;

}