use crate::db::schema::todos;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Debug, Serialize, Clone)]
#[diesel(table_name = todos)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub completed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
