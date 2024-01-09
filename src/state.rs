use axum::extract::FromRef;

use crate::todos::repo::TodoRepo;

#[derive(Clone)]
// An app state passed to every api routes
// which contains dependencies.
pub struct AppState {
    pub todo_repo: TodoRepo,
}

// Support converting an `AppState` in an `TodoRepo`
impl FromRef<AppState> for TodoRepo {
    fn from_ref(app_state: &AppState) -> TodoRepo {
        app_state.todo_repo.clone()
    }
}
