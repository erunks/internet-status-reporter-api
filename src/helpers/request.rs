use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct PaginatedRequest {
    pub page_offset: usize,
    pub per_page: usize,
}

impl Default for PaginatedRequest {
    fn default() -> Self {
        Self {
            page_offset: 0,
            per_page: 10,
        }
    }
}
