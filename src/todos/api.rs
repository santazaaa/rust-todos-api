use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;
use super::{model::{Todo, TodoListQuery}, repo::TodoRepo};
use serde::Deserialize;

pub async fn todos_index(
    query: Option<Query<TodoListQuery>>,
    State(todo_repo): State<TodoRepo>,
) -> impl IntoResponse {
    let Query(query) = query.unwrap_or_default();
    let todos = todo_repo.list(&query);
    Json(todos)
}

pub async fn todos_get(
    Path(id): Path<Uuid>,
    State(todo_repo): State<TodoRepo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = todo_repo.get(id)
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(todo))
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    text: String,
}

pub async fn todos_create(State(todo_repo): State<TodoRepo>, Json(input): Json<CreateTodo>) -> impl IntoResponse {
    let mut todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed_at: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    todo = todo_repo.create(&todo);

    (StatusCode::CREATED, Json(todo))
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

pub async fn todos_update(
    Path(id): Path<Uuid>,
    State(todo_repo): State<TodoRepo>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = todo_repo.get(id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(text) = input.text {
        todo.text = text;
    }

    if let Some(completed) = input.completed {
        if completed {
            todo.completed_at = Some(Utc::now().naive_utc());
        } else {
            todo.completed_at = None;
        }
    }

    todo.updated_at = Utc::now().naive_utc();

    todo_repo.update(&todo);
    Ok(Json(todo))
}

pub async fn todos_delete(Path(id): Path<Uuid>, State(todo_repo): State<TodoRepo>) -> impl IntoResponse {
    if todo_repo.get(id).is_none() {
        return StatusCode::NOT_FOUND
    }
    todo_repo.delete(id);
    StatusCode::NO_CONTENT
}

