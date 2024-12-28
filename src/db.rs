use sqlx::sqlite::SqlitePool;
use crate::model::{UpdateTodo, Todo};

pub async fn initialize_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id VARCHAR(255) PRIMARY KEY,
            text TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_all_todos(pool: &SqlitePool, offset: i32, limit: i32) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, text, completed
        FROM todos
        LIMIT ? OFFSET ?;
        "#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

pub async fn create_todo(pool: &SqlitePool, todo: &Todo) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO todos (id, text, completed)
        VALUES (?1, ?2, ?3);
        "#,
        todo.id,
        todo.text,
        todo.completed
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_todo(
    pool: &SqlitePool,
    id: String,
    input: UpdateTodo,
) -> Result<Todo, sqlx::Error> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, text, completed
        FROM todos
        WHERE id = ?1;
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    let mut todo = todo.ok_or(sqlx::Error::RowNotFound)?;

    let new_text = input.text.unwrap_or(todo.text);
    let new_completed = input.completed.unwrap_or(todo.completed);

    sqlx::query!(
        r#"
        UPDATE todos
        SET text = ?1, completed = ?2
        WHERE id = ?3;
        "#,
        new_text,
        new_completed,
        todo.id
    )
    .execute(pool)
    .await?;

    todo.text = new_text;
    todo.completed = new_completed;

    Ok(todo)
}

pub async fn delete_todo(pool: &SqlitePool, id: &String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM todos
        WHERE id = ?1;
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}