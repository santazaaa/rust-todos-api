use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
}

pub type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;
