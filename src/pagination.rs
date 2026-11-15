use serde::{Deserialize, Serialize};

const DEFAULT_PER_PAGE: i64 = 20;
const MAX_PER_PAGE: i64 = 100;

/// Query-string params shared by every list endpoint:
/// `?page=1&per_page=20&q=pasta`
#[derive(Debug, Deserialize)]
pub struct ListParams {
    #[serde(default = "one")]
    page: i64,
    #[serde(default = "default_per_page")]
    per_page: i64,
    pub q: Option<String>,
}

fn one() -> i64 {
    1
}
fn default_per_page() -> i64 {
    DEFAULT_PER_PAGE
}

impl ListParams {
    pub fn page(&self) -> i64 {
        self.page.max(1)
    }
    pub fn per_page(&self) -> i64 {
        self.per_page.clamp(1, MAX_PER_PAGE)
    }
    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.per_page()
    }
}

/// Generic paginated response envelope, reusable for any resource.
#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, params: &ListParams, total: i64) -> Self {
        let per_page = params.per_page();
        Self {
            items,
            page: params.page(),
            per_page,
            total,
            total_pages: (total + per_page - 1) / per_page,
        }
    }
}
