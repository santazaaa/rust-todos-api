use super::model::Todo;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

#[derive(Clone)]
pub struct TodoRepo {
    dbpool: Pool<ConnectionManager<PgConnection>>,
}

impl TodoRepo {
    pub fn new(dbpool: Pool<ConnectionManager<PgConnection>>) -> Self {
        return TodoRepo { dbpool };
    }

    pub fn list(&self) -> Vec<Todo> {
        use crate::db::schema::todos::dsl::*;
        let conn = &mut self.dbpool.get().unwrap();
        let results = todos
            .select(Todo::as_select())
            .load(conn)
            .expect("Error loading todos");
        return results;
    }
}
