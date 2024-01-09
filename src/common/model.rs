use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct PaginationQuery {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}
