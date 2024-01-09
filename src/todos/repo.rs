use super::model::{Todo, TodoListQuery};
use crate::common::config::DEFAULT_PAGINATION_LIMIT;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct TodoRepo {
    db_pool: Pool<ConnectionManager<PgConnection>>,
}

impl TodoRepo {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        TodoRepo { db_pool }
    }

    pub fn create(&self, todo: &Todo) -> Todo {
        use crate::common::db::schema::todos::dsl::*;
        let conn = &mut self.db_pool.get().unwrap();
        let result = diesel::insert_into(todos)
            .values(todo)
            .returning(Todo::as_returning())
            .get_result(conn)
            .expect("Error saving todo");
        return result;
    }

    pub fn get(&self, id: Uuid) -> Option<Todo> {
        use crate::common::db::schema::todos::dsl::todos;
        let conn = &mut self.db_pool.get().unwrap();
        let result = todos
            .find(id)
            .select(Todo::as_select())
            .first(conn)
            .optional();
        match result {
            Ok(val) => val,
            Err(_) => None,
        }
    }

    pub fn list(&self, query: &TodoListQuery) -> Vec<Todo> {
        use crate::common::db::schema::todos::dsl::todos;
        let conn = &mut self.db_pool.get().unwrap();
        let results = todos
            .select(Todo::as_select())
            .limit(query.limit.unwrap_or(DEFAULT_PAGINATION_LIMIT) as i64)
            .offset(query.offset.unwrap_or(0) as i64)
            .load(conn)
            .expect("Error loading todos");
        return results;
    }

    pub fn update(&self, todo: &Todo) {
        use crate::common::db::schema::todos::dsl::*;
        let conn = &mut self.db_pool.get().unwrap();
        let _ = diesel::update(todos.find(todo.id))
            .set((
                text.eq(todo.text.clone()),
                completed_at.eq(todo.completed_at),
                updated_at.eq(todo.updated_at),
            ))
            .execute(conn);
    }

    pub fn delete(&self, id: Uuid) {
        use crate::common::db::schema::todos::dsl::todos;
        let conn = &mut self.db_pool.get().unwrap();
        let _ = diesel::delete(todos.find(id)).execute(conn);
    }
}
