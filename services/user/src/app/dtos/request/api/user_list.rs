use serde::Deserialize;

// **Pagination Query Parameters DTO**
#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}
