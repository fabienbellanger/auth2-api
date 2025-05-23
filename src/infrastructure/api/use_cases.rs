//! Use cases

use crate::adapters::database::mysql::Db;
use crate::adapters::database::mysql::repositories::application::ApplicationMysqlRepository;
use crate::adapters::database::mysql::repositories::external_link::ExternalLinkMysqlRepository;
use crate::adapters::database::mysql::repositories::password_reset::PasswordResetMysqlRepository;
use crate::adapters::database::mysql::repositories::refresh_token::RefreshTokenMysqlRepository;
use crate::adapters::database::mysql::repositories::scope::ScopeMysqlRepository;
use crate::adapters::database::mysql::repositories::user::UserMysqlRepository;
use crate::adapters::email::EmailAdapter;
use crate::domain::use_cases::application::ApplicationUseCases;
use crate::domain::use_cases::external_link::ExternalLinkUseCases;
use crate::domain::use_cases::scope::ScopeUseCases;
use crate::domain::use_cases::user::UserUseCases;
use crate::infrastructure::api::response::ApiError;

#[derive(Clone)]
pub struct AppUseCases {
    pub user:
        UserUseCases<UserMysqlRepository, RefreshTokenMysqlRepository, PasswordResetMysqlRepository, EmailAdapter>,
    pub application: ApplicationUseCases<ApplicationMysqlRepository>,
    pub scope: ScopeUseCases<ScopeMysqlRepository>,
    pub external_link: ExternalLinkUseCases<ExternalLinkMysqlRepository>,
}

impl AppUseCases {
    pub async fn new(db: Db, email_service: EmailAdapter) -> Result<Self, ApiError> {
        // User
        let user_repository = UserMysqlRepository::new(db.clone());
        let refresh_token_repository = RefreshTokenMysqlRepository::new(db.clone());
        let password_reset_repository = PasswordResetMysqlRepository::new(db.clone());
        let user_use_case = UserUseCases::new(
            user_repository,
            refresh_token_repository,
            password_reset_repository,
            email_service,
        );

        // Application
        let application_repository = ApplicationMysqlRepository::new(db.clone());
        let application_use_case = ApplicationUseCases::new(application_repository);

        // Scope
        let scope_repository = ScopeMysqlRepository::new(db.clone());
        let scope_use_case = ScopeUseCases::new(scope_repository);

        // External link
        let external_link_repository = ExternalLinkMysqlRepository::new(db.clone());
        let external_link_use_case = ExternalLinkUseCases::new(external_link_repository);

        Ok(Self {
            user: user_use_case,
            application: application_use_case,
            scope: scope_use_case,
            external_link: external_link_use_case,
        })
    }
}
