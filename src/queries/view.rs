use sqlx::SqlitePool;

use super::View;

pub async fn increase_thread_views() {
    // INSERT INTO views (thread_id) VALUES (?);
}

pub async fn get_thread_views() {
    // SELECT COUNT(*) AS view_count FROM views WHERE thread_id = ?;

}