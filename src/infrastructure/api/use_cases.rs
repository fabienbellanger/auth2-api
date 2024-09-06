//! Use cases

use crate::adapters::database::mysql::repositories::refresh_token::RefreshTokenMysqlRepository;
use crate::adapters::database::mysql::repositories::user::UserMysqlRepository;
use crate::adapters::database::mysql::Db;
use crate::domain::use_cases::user::UserUseCases;
use crate::infrastructure::api::response::ApiError;

#[derive(Clone)]
pub struct AppUseCases {
    pub user: UserUseCases<UserMysqlRepository, RefreshTokenMysqlRepository>,
}

impl AppUseCases {
    pub async fn new(db: Db) -> Result<Self, ApiError> {
        // User
        let user_repository = UserMysqlRepository::new(db.clone());
        let refresh_token_repository = RefreshTokenMysqlRepository::new(db.clone());
        let user_use_case = UserUseCases::new(user_repository, refresh_token_repository);

        Ok(Self { user: user_use_case })
    }
}
