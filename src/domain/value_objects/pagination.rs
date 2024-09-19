//! Pagination value object representation

/// Pagination max limit
pub const PAGINATION_MAX_LIMIT: u32 = 500;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pagination {
    page: u32,
    limit: u32,
}

impl Pagination {
    /// Create a new pagination
    ///
    /// # Examples
    ///
    /// ```
    /// use auth2_api::domain::value_objects::pagination::{Pagination, PAGINATION_MAX_LIMIT};
    ///
    /// let pagination = Pagination::new(1, 100);
    /// assert_eq!(pagination.page(), 1);
    /// assert_eq!(pagination.limit(), 100);
    ///
    /// // Invalid page
    /// let pagination = Pagination::new(0, 100);
    /// assert_eq!(pagination.page(), 1);
    /// assert_eq!(pagination.limit(), 100);
    ///
    /// // Limit too high
    /// let pagination = Pagination::new(2, 1_000);
    /// assert_eq!(pagination.page(), 2);
    /// assert_eq!(pagination.limit(), PAGINATION_MAX_LIMIT);
    /// ```
    pub fn new(page: u32, limit: u32) -> Self {
        let page = if page == 0 { 1 } else { page };
        let limit = if limit > PAGINATION_MAX_LIMIT {
            PAGINATION_MAX_LIMIT
        } else {
            limit
        };

        Self { page, limit }
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

impl Default for Pagination {
    /// Default pagination
    ///
    /// ```
    /// use auth2_api::domain::value_objects::pagination::{Pagination, PAGINATION_MAX_LIMIT};
    ///
    /// let pagination = Pagination::default();
    /// assert_eq!(pagination.page(), 1);
    /// assert_eq!(pagination.limit(), PAGINATION_MAX_LIMIT);
    /// ```
    fn default() -> Self {
        Self::new(1, PAGINATION_MAX_LIMIT)
    }
}
