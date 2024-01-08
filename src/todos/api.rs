use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
use crate::state::AppState;
use super::model::Todo;


// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub async fn todos_index(
    _pagination: Option<Query<Pagination>>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // let Query(pagination) = pagination.unwrap_or_default();

    let todos = state.todo_repo.list();

    Json(todos)
}

pub async fn todos_get(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = state.todo_repo.get(id)
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(todo))
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    text: String,
}

pub async fn todos_create(State(state): State<AppState>, Json(input): Json<CreateTodo>) -> impl IntoResponse {
    let mut todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed_at: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    todo = state.todo_repo.create(&todo);

    (StatusCode::CREATED, Json(todo))
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

pub async fn todos_update(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = state.todo_repo.get(id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(text) = input.text {
        todo.text = text;
    }

    if let Some(completed) = input.completed {
        if completed {
            todo.completed_at = Some(Utc::now().naive_utc());
        } else {
            todo.completed_at = None
        }
    }

    todo.updated_at = Utc::now().naive_utc();

    state.todo_repo.update(&todo);

    Ok(Json(todo))
}

pub async fn todos_delete(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    state.todo_repo.delete(id);
    StatusCode::NO_CONTENT
}

