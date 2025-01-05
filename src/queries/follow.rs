use sqlx::SqlitePool;

use super::Follow;

pub async fn follow_user_by_id() {
    // INSERT INTO follow (user_id, follower_id) VALUES (?, ?);
}

pub async fn unfollow_user_by_id() {
    // DELETE FROM follow WHERE id = ?;

}

pub async fn get_user_follower_list() {
    // SELECT * FROM follow WHERE follower_id = ?;

}

pub async fn get_user_following_list() {
    // SELECT * FROM follow WHERE user_id = ?;

}