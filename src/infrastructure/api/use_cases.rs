//! Use cases

use crate::adapters::database::mysql::repositories::user::UserMysqlRepository;
use crate::adapters::database::mysql::Db;
use crate::domain::use_cases::user::UserUseCases;
use crate::infrastructure::api::response::ApiError;

#[derive(Clone)]
pub struct AppUseCases {
    pub user: UserUseCases<UserMysqlRepository>,
}

impl AppUseCases {
    pub async fn new(db: Db) -> Result<Self, ApiError> {
        // User
        let user_repository = UserMysqlRepository::new(db.clone());
        let user_use_case = UserUseCases::new(user_repository);

        Ok(Self { user: user_use_case })
    }
}
