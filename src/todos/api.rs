//! Provides a RESTful web server managing some Todos.
//!
//! API will be:
//!
//! - `GET /todos`: return a JSON list of Todos.
//! - `POST /todos`: create a new Todo.
//! - `PATCH /todos/:id`: update a specific Todo.
//! - `DELETE /todos/:id`: delete a specific Todo.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p example-todos
//! ```


use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{Utc};
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

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    text: String,
}

pub async fn todos_create(State(state): State<AppState>, Json(input): Json<CreateTodo>) -> impl IntoResponse {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed_at: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    state.todo_db.write().unwrap().insert(todo.id, todo.clone());

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
    let mut todo = state.todo_db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(text) = input.text {
        todo.text = text;
    }

    if input.completed.is_some() {
        todo.completed_at = Some(Utc::now().naive_utc());
    }

    state.todo_db.write().unwrap().insert(todo.id, todo.clone());

    Ok(Json(todo))
}

pub async fn todos_delete(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    if state.todo_db.write().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

