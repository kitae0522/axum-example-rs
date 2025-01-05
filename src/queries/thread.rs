use sqlx::SqlitePool;

use super::Thread;

pub async fn create_thread() {
    // INSERT INTO thread (user_id, title, content, parent_thread) VALUES (?, ?, ?, ?);
}

pub async fn get_thread_by_id() {
    // SELECT * FROM thread WHERE id = ? AND is_deleted = FALSE;

}

pub async fn get_thread_by_user_id() {
    // SELECT * FROM thread WHERE user_id = ? AND is_deleted = FALSE;

}

pub async fn list_thread() {
    // SELECT * FROM thread WHERE is_deleted = FALSE;

}

pub async fn update_thread_by_id() {
    // UPDATE thread SET title = ?, content = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND is_deleted = FALSE;

}

pub async fn deactivated_thread_by_id() {
    // UPDATE thread SET is_deleted = TRUE, deleted_at = CURRENT_TIMESTAMP WHERE id = ?;

}

pub async fn activated_thread_by_id() {
    // UPDATE thread SET is_deleted = FALSE, deleted_at = NULL WHERE id = ?;

}

pub async fn delete_thread_by_id() {
    // DELETE FROM thread WHERE id = ?;

}