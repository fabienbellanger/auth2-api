//! User MySQL repository

use crate::adapters::database::mysql::Db;
use crate::domain::repositories::user::dto::{
    CreateUserDtoRequest, CreateUserDtoResponse, GetAccessTokenInformationDtoRequest,
    GetAccessTokenInformationDtoResponse,
};
use crate::domain::repositories::user::UserRepository;
use crate::domain::use_cases::user::create_user::{CreateUserUseCaseResponse, UserCreationError};
use crate::domain::use_cases::user::get_access_token::GetAccessTokenError;
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::id::Id;
use crate::domain::value_objects::password::Password;
use async_trait::async_trait;
use std::str::FromStr;
use std::sync::Arc;

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
    async fn create_user(&self, req: CreateUserDtoRequest) -> Result<CreateUserDtoResponse, UserCreationError> {
        let user_id = Id::new().map_err(|err| {
            error!(error = %err, "Failed to create user ID");
            UserCreationError::InvalidId()
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
            UserCreationError::DatabaseError()
        })?;

        Ok(CreateUserDtoResponse(CreateUserUseCaseResponse {
            id: user_id,
            email: req.0.email,
            password: req.0.password,
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
    ) -> Result<Option<GetAccessTokenInformationDtoResponse>, GetAccessTokenError> {
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
            GetAccessTokenError::DatabaseError()
        })?;

        let response = match result {
            Some(row) => Some(GetAccessTokenInformationDtoResponse {
                id: Id::from_str(&row.id).map_err(|_| GetAccessTokenError::InvalidId())?,
                password: Password::new(&row.password, true).map_err(|_| GetAccessTokenError::InvalidPassword())?,
            }),
            None => None,
        };

        Ok(response)
    }
}
