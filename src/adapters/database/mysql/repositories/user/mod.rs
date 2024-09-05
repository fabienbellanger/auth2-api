//! User MySQL repository

use crate::adapters::database::mysql::{Db, PaginationSort};
use crate::domain::entities::user::UserId;
use crate::domain::repositories::user::dto::{
    CountUsersDtoRequest, CountUsersDtoResponse, CreateUserDtoRequest, CreateUserDtoResponse, DeleteUserDtoRequest,
    DeleteUserDtoResponse, GetAccessTokenInformationDtoRequest, GetAccessTokenInformationDtoResponse,
    GetUserByIdDtoRequest, GetUserByIdDtoResponse, GetUsersDtoRequest, GetUsersDtoResponse,
};
use crate::domain::repositories::user::UserRepository;
use crate::domain::use_cases::user::delete_user::DeleteUserUseCaseResponse;
use crate::domain::use_cases::user::{UserUseCaseError, UserUseCaseResponse};
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::id::Id;
use crate::domain::value_objects::password::Password;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::mysql::MySqlRow;
use sqlx::Row;
use std::str::FromStr;
use std::sync::Arc;

impl From<sqlx::error::Error> for UserUseCaseError {
    fn from(err: sqlx::error::Error) -> Self {
        error!(error = %err, "Database error");
        UserUseCaseError::DatabaseError("Database error".to_string())
    }
}

impl TryFrom<MySqlRow> for UserUseCaseResponse {
    type Error = UserUseCaseError;

    fn try_from(row: MySqlRow) -> Result<Self, Self::Error> {
        Ok(UserUseCaseResponse {
            id: Id::from_str(row.try_get("id")?)?,
            email: Email::new(row.try_get("email")?)?,
            lastname: row.try_get("lastname")?,
            firstname: row.try_get("firstname")?,
            created_at: UtcDateTime::new(row.try_get("created_at")?),
            updated_at: UtcDateTime::new(row.try_get("updated_at")?),
        })
    }
}

/// User MySQL repository
#[derive(Debug, Clone)]
pub struct UserMysqlRepository {
    db: Arc<Db>,
}

impl UserMysqlRepository {
    /// Create a new repository
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl UserRepository for UserMysqlRepository {
    #[instrument(skip(self), name = "user_repository_create")]
    async fn create_user(&self, req: CreateUserDtoRequest) -> Result<CreateUserDtoResponse, UserUseCaseError> {
        let user_id = Id::new().map_err(|err| {
            error!(error = %err, "Failed to create user ID");
            UserUseCaseError::InvalidId()
        })?;
        let now = UtcDateTime::now();

        // Create user
        let _result = sqlx::query!(
            "
            INSERT INTO users (id, email, password, lastname, firstname, created_at, updated_at, deleted_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, NULL)
        ",
            user_id.clone().to_string(),
            req.0.email.to_string(),
            req.0.password.to_string(),
            req.0.lastname,
            req.0.firstname,
            now.value(),
            now.value()
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to create user");
            UserUseCaseError::DatabaseError("User creation error".to_string())
        })?;

        Ok(CreateUserDtoResponse(UserUseCaseResponse {
            id: user_id,
            email: req.0.email,
            lastname: req.0.lastname,
            firstname: req.0.firstname,
            created_at: now.clone(),
            updated_at: now,
        }))
    }

    #[instrument(skip(self), name = "user_repository_get_user_by_email")]
    async fn get_access_token_information(
        &self,
        req: GetAccessTokenInformationDtoRequest,
    ) -> Result<Option<GetAccessTokenInformationDtoResponse>, UserUseCaseError> {
        let result = sqlx::query!(
            "
            SELECT id, password
            FROM users
            WHERE email = ?
                AND deleted_at IS NULL",
            req.0.to_string()
        )
        .fetch_optional(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to get user by email");
            UserUseCaseError::DatabaseError("Failed to get user by email".to_string())
        })?;

        let response = match result {
            Some(row) => Some(GetAccessTokenInformationDtoResponse {
                id: Id::from_str(&row.id).map_err(|_| UserUseCaseError::InvalidId())?,
                password: Password::new(&row.password, true)
                    .map_err(|_| UserUseCaseError::InvalidPassword("Failed to generate user ID".to_string()))?,
            }),
            None => None,
        };

        Ok(response)
    }

    #[instrument(skip(self), name = "user_repository_get_users")]
    async fn get_users(&self, req: GetUsersDtoRequest) -> Result<GetUsersDtoResponse, UserUseCaseError> {
        let mut query = String::from(
            r#"
            SELECT id, email, lastname, firstname, created_at, updated_at
            FROM users
            WHERE deleted_at IS NULL
        "#,
        );

        let filter = PaginationSort::from(req.0.filter.unwrap_or_default());

        // Sorts and pagination
        query.push_str(&filter.get_sorts_sql(Some(&[
            "id",
            "lastname",
            "firstname",
            "created_at",
            "updated_at",
            "deleted_at",
        ])));
        query.push_str(&filter.get_pagination_sql());

        let users = sqlx::query(&query)
            .fetch_all(self.db.pool.clone().as_ref())
            .await
            .map_err(|err| {
                error!(error = %err, "Failed to get users");
                UserUseCaseError::DatabaseError("Failed to get users".to_string())
            })?
            .into_iter()
            .map(UserUseCaseResponse::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(GetUsersDtoResponse(users))
    }

    #[instrument(skip(self, _req), name = "user_repository_count_users")]
    async fn count_users(&self, _req: CountUsersDtoRequest) -> Result<CountUsersDtoResponse, UserUseCaseError> {
        let result = sqlx::query!("SELECT COUNT(*) AS total FROM users WHERE deleted_at IS NULL")
            .fetch_one(self.db.pool.clone().as_ref())
            .await
            .map_err(|err| {
                error!(error = %err, "Failed to count users");
                UserUseCaseError::DatabaseError("Failed to count users".to_string())
            })?;

        Ok(CountUsersDtoResponse(result.total))
    }

    #[instrument(skip(self, req), name = "user_repository_get_user")]
    async fn get_user_by_id(&self, req: GetUserByIdDtoRequest) -> Result<GetUserByIdDtoResponse, UserUseCaseError> {
        let result = sqlx::query!(
            "
            SELECT id, email, lastname, firstname, created_at, updated_at
            FROM users
            WHERE id = ?
                AND deleted_at IS NULL",
            req.0.user_id.to_string()
        )
        .fetch_optional(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to get user by ID");
            UserUseCaseError::DatabaseError("Failed to get user by ID".to_string())
        })?;

        let user = match result {
            Some(row) => UserUseCaseResponse {
                id: UserId::from_str(&row.id)?,
                email: Email::new(&row.email)?,
                lastname: row.lastname,
                firstname: row.firstname,
                created_at: UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(row.created_at, Utc)),
                updated_at: UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(row.updated_at, Utc)),
            },
            None => Err(UserUseCaseError::UserNotFound())?,
        };

        Ok(GetUserByIdDtoResponse(user))
    }

    #[instrument(skip(self, req), name = "user_repository_delete_user")]
    async fn delete_user(&self, req: DeleteUserDtoRequest) -> Result<DeleteUserDtoResponse, UserUseCaseError> {
        let result = sqlx::query!(
            "
            UPDATE users
            SET deleted_at = ?
            WHERE id = ?
                AND deleted_at IS NULL",
            Some(UtcDateTime::now().value()),
            req.0.user_id.to_string()
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to delete user");
            UserUseCaseError::DatabaseError("Failed to delete user".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(UserUseCaseError::UserNotFound())?;
        }

        Ok(DeleteUserDtoResponse(DeleteUserUseCaseResponse()))
    }
}
