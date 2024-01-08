use crate::todos::repo::TodoRepo;

#[derive(Clone)]
pub struct AppState {
    pub todo_repo: TodoRepo,
}
