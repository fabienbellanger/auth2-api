//! Pagination value object representation
//!
//! # Example
//! ```
//! use auth2_api::domain::value_objects::pagination::{Pagination, PAGINATION_MAX_LIMIT};
//!
//! let pagination = Pagination::new(1, 10);
//! assert_eq!(pagination.page(), 1);
//! assert_eq!(pagination.limit(), 10);
//!
//! let pagination = Pagination::new(23, 1000);
//! assert_eq!(pagination.page(), 23);
//! assert_eq!(pagination.limit(), PAGINATION_MAX_LIMIT);
//! ```

/// Pagination max limit
pub const PAGINATION_MAX_LIMIT: u32 = 500;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Pagination {
    page: u32,
    limit: u32,
}

impl Pagination {
    /// Create a new pagination
    pub fn new(page: u32, limit: u32) -> Self {
        if limit > PAGINATION_MAX_LIMIT {
            Self {
                page,
                limit: PAGINATION_MAX_LIMIT,
            }
        } else {
            Self { page, limit }
        }
    }

    /// Get page
    pub fn page(&self) -> u32 {
        self.page
    }

    /// Get limit
    pub fn limit(&self) -> u32 {
        self.limit
    }
}
