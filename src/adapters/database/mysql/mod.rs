//! MySQL adapter

use crate::adapters::database::{DatabaseError, GenericDb};
use crate::config::Config;
use crate::domain::value_objects::pagination::{Pagination, PAGINATION_MAX_LIMIT};
use crate::domain::value_objects::query_sort::{QuerySort, QuerySorts};
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

/// Pagination for MySQL queries
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MysqlPagination {
    pub page: u32,
    pub limit: u32,
    pub offset: u32,
}

impl From<Pagination> for MysqlPagination {
    fn from(pagination: Pagination) -> Self {
        // Page & limit
        let page = match pagination.page() >= 1 {
            true => pagination.page(),
            false => 1,
        };

        let limit = match (1..=PAGINATION_MAX_LIMIT).contains(&pagination.limit()) {
            true => pagination.limit(),
            false => PAGINATION_MAX_LIMIT,
        };

        let offset = (page - 1) * limit;

        Self { page, limit, offset }
    }
}

impl MysqlPagination {
    /// Generate SQL code for pagination
    pub fn to_sql(&self) -> String {
        format!(" LIMIT {} OFFSET {}", self.limit, self.offset)
    }
}

/// Query sorts for MySQL queries
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MysqlQuerySorts(pub QuerySorts);

impl MysqlQuerySorts {
    /// Generate SQL code for sorts (ORDER BY)
    pub fn to_sql(&self, valid_fields: &[&str]) -> String {
        self.0
             .0
            .iter()
            .filter(|QuerySort { field, direction: _ }| valid_fields.contains(&field.as_str()))
            .enumerate()
            .map(|(i, QuerySort { field, direction: sort })| {
                if i == 0 {
                    format!(" ORDER BY {field} {sort}")
                } else {
                    format!(", {field} {sort}")
                }
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::pagination::Pagination;
    use crate::domain::value_objects::query_sort::QuerySortDirection;

    #[test]
    fn test_from_pagination_mysql_pagination() {
        let pagination_sort = MysqlPagination::from(Pagination::new(1, 10));
        assert_eq!(pagination_sort.limit, 10);
        assert_eq!(pagination_sort.offset, 0);

        let pagination_sort = MysqlPagination::from(Pagination::new(1, 1_000));
        assert_eq!(pagination_sort.limit, PAGINATION_MAX_LIMIT);
        assert_eq!(pagination_sort.offset, 0);

        let pagination_sort = MysqlPagination::from(Pagination::new(2, 1_000));
        assert_eq!(pagination_sort.limit, PAGINATION_MAX_LIMIT);
        assert_eq!(pagination_sort.offset, PAGINATION_MAX_LIMIT);
    }

    #[test]
    fn test_mysql_pagination_to_sql() {
        let pagination = MysqlPagination::from(Pagination::new(1, 50));
        assert_eq!(String::from(" LIMIT 50 OFFSET 0"), pagination.to_sql());

        let pagination = MysqlPagination::from(Pagination::new(3, 50));
        assert_eq!(String::from(" LIMIT 50 OFFSET 100"), pagination.to_sql());
    }

    #[test]
    fn test_mysql_query_sorts_without_sort() {
        let mysql_sorts = MysqlQuerySorts(QuerySorts::default());
        assert_eq!(String::new(), mysql_sorts.to_sql(&[]));
    }

    #[test]
    fn test_mysql_query_sorts_without_valid_fields() {
        let valid_fields: &[&str] = &[];

        let mut mysql_sorts = MysqlQuerySorts(QuerySorts::default());
        assert_eq!(String::new(), mysql_sorts.to_sql(valid_fields));

        mysql_sorts = MysqlQuerySorts(QuerySorts(vec![
            QuerySort::new("id".to_owned(), QuerySortDirection::Asc),
            QuerySort::new("name".to_owned(), QuerySortDirection::Desc),
        ]));
        assert_eq!(String::new(), mysql_sorts.to_sql(valid_fields));
    }

    #[test]
    fn test_mysql_query_sorts_with_valid_fields() {
        let valid_fields = &["id", "name"];

        let mut mysql_sorts = MysqlQuerySorts(QuerySorts::default());
        assert_eq!(String::new(), mysql_sorts.to_sql(valid_fields));

        mysql_sorts = MysqlQuerySorts(QuerySorts(vec![
            QuerySort::new("id".to_owned(), QuerySortDirection::Asc),
            QuerySort::new("name".to_owned(), QuerySortDirection::Desc),
        ]));
        assert_eq!(
            " ORDER BY id ASC, name DESC".to_owned(),
            mysql_sorts.to_sql(valid_fields)
        );

        let valid_fields: &[&str] = &["name"];
        assert_eq!(" ORDER BY name DESC".to_owned(), mysql_sorts.to_sql(valid_fields));

        let valid_fields: &[&str] = &["id", "name"];
        mysql_sorts = MysqlQuerySorts(QuerySorts(vec![
            QuerySort::new("idz".to_owned(), QuerySortDirection::Asc),
            QuerySort::new("name".to_owned(), QuerySortDirection::Desc),
        ]));
        assert_eq!(" ORDER BY name DESC".to_owned(), mysql_sorts.to_sql(valid_fields));

        let valid_fields: &[&str] = &["id", "name"];
        mysql_sorts = MysqlQuerySorts(QuerySorts(vec![
            QuerySort::new("id".to_owned(), QuerySortDirection::Asc),
            QuerySort::new("namee".to_owned(), QuerySortDirection::Desc),
        ]));
        assert_eq!(" ORDER BY id ASC".to_owned(), mysql_sorts.to_sql(valid_fields));

        let valid_fields: &[&str] = &["id", "name"];
        mysql_sorts = MysqlQuerySorts(QuerySorts(vec![
            QuerySort::new("idz".to_owned(), QuerySortDirection::Asc),
            QuerySort::new("namee".to_owned(), QuerySortDirection::Desc),
        ]));
        assert_eq!("".to_owned(), mysql_sorts.to_sql(valid_fields));
    }

    #[test]
    fn test_mysql_query_sorts_with_valid_fields_and_table_prefix() {
        let valid_fields: &[&str] = &["user.id", "role.name"];
        let mysql_sorts = MysqlQuerySorts(QuerySorts(vec![
            QuerySort::new("user.id".to_owned(), QuerySortDirection::Asc),
            QuerySort::new("role.name".to_owned(), QuerySortDirection::Desc),
        ]));
        assert_eq!(
            " ORDER BY user.id ASC, role.name DESC".to_owned(),
            mysql_sorts.to_sql(valid_fields)
        );
    }
}
