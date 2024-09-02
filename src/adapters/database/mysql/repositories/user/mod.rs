//! User MySQL repository

use crate::adapters::database::mysql::Db;
use crate::domain::repositories::user::dto::{CreateUserDtoRequest, CreateUserDtoResponse};
use crate::domain::repositories::user::UserRepository;
use crate::domain::use_cases::user::create_user::{CreateUserUseCaseResponse, UserCreationError};
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::id::Id;
use async_trait::async_trait;
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
        let user_id = Id::new().map_err(|err| UserCreationError::InvalidId(err.to_string()))?;
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
        .map_err(|err| UserCreationError::DatabaseError(err.to_string()))?;

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
}
