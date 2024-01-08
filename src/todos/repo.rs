use super::model::Todo;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct TodoRepo {
    dbpool: Pool<ConnectionManager<PgConnection>>,
}

impl TodoRepo {
    pub fn new(dbpool: Pool<ConnectionManager<PgConnection>>) -> Self {
        return TodoRepo { dbpool };
    }

    pub fn create(&self, todo: &Todo) -> Todo {
        use crate::db::schema::todos::dsl::*;
        let conn = &mut self.dbpool.get().unwrap();
        let result = diesel::insert_into(todos)
            .values(todo)
            .returning(Todo::as_returning())
            .get_result(conn)
            .expect("Error saving todo");
        return result;
    }

    pub fn get(&self, id: Uuid) -> Option<Todo> {
        use crate::db::schema::todos::dsl::todos;
        let conn = &mut self.dbpool.get().unwrap();
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

    pub fn list(&self) -> Vec<Todo> {
        use crate::db::schema::todos::dsl::todos;
        let conn = &mut self.dbpool.get().unwrap();
        let results = todos
            .select(Todo::as_select())
            .load(conn)
            .expect("Error loading todos");
        return results;
    }

    pub fn update(&self, todo: &Todo) {
        use crate::db::schema::todos::dsl::*;
        let conn = &mut self.dbpool.get().unwrap();
        let _ = diesel::update(todos.find(todo.id))
            .set((
                completed_at.eq(todo.completed_at),
                updated_at.eq(todo.updated_at),
            ))
            .execute(conn);
    }

    pub fn delete(&self, id: Uuid) {
        use crate::db::schema::todos::dsl::todos;
        let conn = &mut self.dbpool.get().unwrap();
        let _ = diesel::delete(todos.find(id)).execute(conn);
    }
}
