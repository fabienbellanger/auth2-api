//! Filter for pagination and sorting

// TODO: Add filtering (https://www.moesif.com/blog/technical/api-design/REST-API-Design-Filtering-Sorting-and-Pagination/)
// Try something like this: /applications/test?price=gte:10,lte:200

use crate::domain::value_objects::pagination::Pagination;
use crate::domain::value_objects::query_sort::QuerySorts;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FilterRequest {
    #[serde(rename(deserialize = "p"))]
    pub page: Option<u32>,

    #[serde(rename(deserialize = "l"))]
    pub limit: Option<u32>,

    #[serde(rename(deserialize = "s"))]
    pub sort: Option<String>,
}

impl FilterRequest {
    pub fn pagination(&self) -> Pagination {
        match (self.page, self.limit) {
            (Some(page), Some(limit)) => Pagination::new(page, limit),
            _ => Pagination::default(),
        }
    }

    pub fn sorts(&self) -> Option<QuerySorts> {
        self.sort.as_ref().map(|sort| QuerySorts::from(sort.as_str()))
    }
}
