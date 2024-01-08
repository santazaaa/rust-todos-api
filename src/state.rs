use crate::todos::{model::Db, repo::TodoRepo};

#[derive(Clone)]
pub struct AppState {
    pub todo_repo: TodoRepo,
    pub todo_db: Db,
}
