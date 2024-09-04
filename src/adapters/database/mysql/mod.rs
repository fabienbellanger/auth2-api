//! MySQL adapter

use crate::adapters::database::{DatabaseError, GenericDb};
use crate::config::Config;
use crate::domain::utils::query_sort::{Filter, Sorts};
use crate::domain::value_objects::pagination::PAGINATION_MAX_LIMIT;
use async_trait::async_trait;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::sync::Arc;
use std::time::Duration;

pub mod repositories;

/// MySQL database adapter
#[derive(Debug, Clone)]
pub struct Db {
    pub pool: Arc<Pool<MySql>>,
}

#[async_trait]
impl GenericDb for Db {
    type Db = Db;

    async fn new(config: &Config) -> Result<Self::Db, DatabaseError> {
        let url = &config.database_url;
        let max_connections = config.database_max_connections;
        let min_connections = config.database_min_connections;
        let max_lifetime = config.database_max_lifetime;
        let connect_timeout = config.database_connect_timeout;
        let idle_timeout = config.database_idle_timeout;

        let pool = MySqlPoolOptions::new()
            .max_connections(max_connections)
            .min_connections(min_connections)
            .max_lifetime(Some(Duration::from_secs(max_lifetime)))
            .acquire_timeout(Duration::from_secs(connect_timeout))
            .idle_timeout(Duration::from_secs(idle_timeout))
            .test_before_acquire(true)
            .connect(url)
            .await
            .map_err(|err| DatabaseError::CreationPoolError(err.to_string()))?;

        if config.database_auto_migration {
            info!("Run database migrations");
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .map_err(|err| DatabaseError::AutoMigrationError(err.to_string()))?
        }

        Ok(Self { pool: Arc::new(pool) })
    }
}

/// Pagination and sort for MySQL queries
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaginationSort {
    pub page: u32,
    pub limit: u32,
    pub offset: u32,
    pub sorts: Sorts,
}

impl From<Filter> for PaginationSort {
    fn from(filter: Filter) -> Self {
        // Page & limit
        let (page, limit) = match filter.pagination {
            Some(pagination) => {
                let page = match pagination.page() >= 1 {
                    true => pagination.page(),
                    false => 1,
                };

                let limit = match (1..=PAGINATION_MAX_LIMIT).contains(&pagination.limit()) {
                    true => pagination.limit(),
                    false => PAGINATION_MAX_LIMIT,
                };

                (page, limit)
            }
            None => (1, PAGINATION_MAX_LIMIT),
        };

        // Offset
        let offset = (page - 1) * limit;

        // Sorts
        let sorts = filter.sorts.unwrap_or_default();

        Self {
            page,
            limit,
            offset,
            sorts,
        }
    }
}

impl PaginationSort {
    /// SQL code for pagination
    pub fn get_pagination_sql(&self) -> String {
        format!(" LIMIT {} OFFSET {}", self.limit, self.offset)
    }

    /// SQL code for sorts (ORDER BY)
    pub fn get_sorts_sql(&self, valid_fields: Option<&[&str]>) -> String {
        let mut s = String::new();
        let mut i = 0;

        for (field, sort) in self.sorts.0.iter() {
            match &valid_fields {
                Some(valid_fields) => {
                    if valid_fields.contains(&field.as_str()) {
                        if i == 0 {
                            s.push_str(" ORDER BY ");
                        } else {
                            s.push_str(", ");
                        }
                        s.push_str(&format!("{field} {sort}"));

                        i += 1;
                    }
                }
                None => {
                    if i == 0 {
                        s.push_str(" ORDER BY ");
                    } else {
                        s.push_str(", ");
                    }
                    s.push_str(&format!("{field} {sort}"));

                    i += 1;
                }
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::utils::query_sort::SortDirection;
    use crate::domain::value_objects::pagination::Pagination;

    #[test]
    fn test_from_filter_to_pagination_sort_pagination() {
        let pagination_sort = PaginationSort::from(Filter {
            pagination: None,
            sorts: None,
        });

        assert_eq!(pagination_sort.page, 1);
        assert_eq!(pagination_sort.limit, PAGINATION_MAX_LIMIT);
        assert_eq!(pagination_sort.offset, 0);

        let pagination_sort = PaginationSort::from(Filter {
            pagination: Some(Pagination::new(1, 10)),
            sorts: None,
        });

        assert_eq!(pagination_sort.page, 1);
        assert_eq!(pagination_sort.limit, 10);
        assert_eq!(pagination_sort.offset, 0);

        let pagination_sort = PaginationSort::from(Filter {
            pagination: Some(Pagination::new(1, 1000)),
            sorts: None,
        });

        assert_eq!(pagination_sort.page, 1);
        assert_eq!(pagination_sort.limit, PAGINATION_MAX_LIMIT);
        assert_eq!(pagination_sort.offset, 0);

        let pagination_sort = PaginationSort::from(Filter {
            pagination: Some(Pagination::new(2, 1000)),
            sorts: None,
        });

        assert_eq!(pagination_sort.page, 2);
        assert_eq!(pagination_sort.limit, PAGINATION_MAX_LIMIT);
        assert_eq!(pagination_sort.offset, PAGINATION_MAX_LIMIT);
    }

    #[test]
    fn test_get_pagination_sql() {
        let pagination_sort = PaginationSort::from(Filter {
            pagination: Some(Pagination::new(1, 50)),
            sorts: None,
        });
        assert_eq!(String::from(" LIMIT 50 OFFSET 0"), pagination_sort.get_pagination_sql());

        let pagination_sort = PaginationSort::from(Filter {
            pagination: Some(Pagination::new(3, 50)),
            sorts: None,
        });
        assert_eq!(
            String::from(" LIMIT 50 OFFSET 100"),
            pagination_sort.get_pagination_sql()
        );
    }

    #[test]
    fn test_get_sorts_sql_without_sort() {
        let pagination_sort = PaginationSort::from(Filter {
            pagination: None,
            sorts: None,
        });
        assert_eq!(String::new(), pagination_sort.get_sorts_sql(None));
    }

    #[test]
    fn test_get_sorts_sql_without_valid_fields() {
        let mut valid_fields: Option<&[&str]> = Some(&[]);

        let mut pagination_sort = PaginationSort::from(Filter {
            pagination: Some(Pagination::new(1, 50)),
            sorts: None,
        });
        assert_eq!(String::new(), pagination_sort.get_sorts_sql(valid_fields));

        pagination_sort.sorts.0 = vec![
            ("id".to_owned(), SortDirection::Asc),
            ("name".to_owned(), SortDirection::Desc),
        ];
        assert_eq!(String::new(), pagination_sort.get_sorts_sql(valid_fields));

        valid_fields = None;
        assert_eq!(
            String::from(" ORDER BY id ASC, name DESC"),
            pagination_sort.get_sorts_sql(valid_fields)
        );
    }

    #[test]
    fn test_get_sorts_sql_with_valid_fields() {
        let valid_fields: Option<&[&str]> = Some(&["id", "name"]);

        let mut pagination_sort = PaginationSort::from(Filter {
            pagination: Some(Pagination::new(1, 50)),
            sorts: None,
        });
        assert_eq!(String::new(), pagination_sort.get_sorts_sql(valid_fields));

        pagination_sort.sorts.0 = vec![
            ("id".to_owned(), SortDirection::Asc),
            ("name".to_owned(), SortDirection::Desc),
        ];
        assert_eq!(
            " ORDER BY id ASC, name DESC".to_owned(),
            pagination_sort.get_sorts_sql(valid_fields)
        );

        let valid_fields: Option<&[&str]> = Some(&["name"]);
        assert_eq!(
            " ORDER BY name DESC".to_owned(),
            pagination_sort.get_sorts_sql(valid_fields)
        );

        let valid_fields: Option<&[&str]> = Some(&["id", "name"]);
        pagination_sort.sorts.0 = vec![
            ("idz".to_owned(), SortDirection::Asc),
            ("name".to_owned(), SortDirection::Desc),
        ];
        assert_eq!(
            " ORDER BY name DESC".to_owned(),
            pagination_sort.get_sorts_sql(valid_fields)
        );

        let valid_fields: Option<&[&str]> = Some(&["id", "name"]);
        pagination_sort.sorts.0 = vec![
            ("id".to_owned(), SortDirection::Asc),
            ("namee".to_owned(), SortDirection::Desc),
        ];
        assert_eq!(
            " ORDER BY id ASC".to_owned(),
            pagination_sort.get_sorts_sql(valid_fields)
        );

        let valid_fields: Option<&[&str]> = Some(&["id", "name"]);
        pagination_sort.sorts.0 = vec![
            ("idz".to_owned(), SortDirection::Asc),
            ("namee".to_owned(), SortDirection::Desc),
        ];
        assert_eq!("".to_owned(), pagination_sort.get_sorts_sql(valid_fields));
    }

    #[test]
    fn test_get_sorts_sql_with_valid_fields_and_table_prefix() {
        let valid_fields: Option<&[&str]> = Some(&["user.id", "role.name"]);
        let pagination_sort = PaginationSort::from(Filter {
            pagination: Some(Pagination::new(1, 50)),
            sorts: Some(Sorts(vec![
                ("user.id".to_owned(), SortDirection::Asc),
                ("role.name".to_owned(), SortDirection::Desc),
            ])),
        });
        assert_eq!(
            " ORDER BY user.id ASC, role.name DESC".to_owned(),
            pagination_sort.get_sorts_sql(valid_fields)
        );
    }
}
