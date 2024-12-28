use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use db::{initialize_db, get_all_todos, create_todo, delete_todo, update_todo};
use model::{Pagination, CreateTodo, UpdateTodo, Todo};

mod db;
mod model;

#[tokio::main]
async fn main() {
    let db_pool = SqlitePool::connect("todos.db").await.unwrap();
    initialize_db(&db_pool).await.unwrap();

    let app = Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/{id}", patch(todos_update).delete(todos_delete))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// curl "http://127.0.0.1:3000/todos?offset=0&limit=10"
async fn todos_index(
    pagination: Query<Pagination>,
    State(db_pool): State<SqlitePool>
) -> impl IntoResponse {
    let todos = get_all_todos(
        &db_pool,
        pagination.offset.unwrap_or(0) as i32,
        pagination.limit.unwrap_or(100) as i32
    )
    .await
    .unwrap();

    Json(todos)
}

// curl -X POST "http://127.0.0.1:3000/todos" \
//  -H "Content-Type: application/json" \
//  -d '{"text": "Learn Rust with Axum"}'
async fn todos_create(
    State(db_pool): State<SqlitePool>,
    Json(input): Json<CreateTodo>
) -> impl IntoResponse {
    let todo = Todo {
        id: Some(Uuid::new_v4().to_string()),
        text: input.text,
        completed: false,
    };

    create_todo(&db_pool, &todo).await.unwrap();
    (StatusCode::CREATED, Json(todo))
}

// curl -X PATCH "http://127.0.0.1:3000/todos/{id}" \
//  -H "Content-Type: application/json" \
//  -d '{"text": "Updated todo text", "completed": true}'
async fn todos_update(
    Path(id): Path<String>,
    State(db_pool): State<SqlitePool>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = update_todo(&db_pool, id, input).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(todo))
}

// curl -X DELETE "http://127.0.0.1:3000/todos/{id}"
async fn todos_delete(
    Path(id): Path<String>,
    State(db_pool): State<SqlitePool>
) -> impl IntoResponse {
    if delete_todo(&db_pool, &id).await.is_ok() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}