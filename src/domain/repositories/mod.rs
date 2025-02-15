//! Repositories represent the data access layer of the application.
//! They are responsible for abstracting the data access logic and providing
//! a clean API to interact with the database or external resources.

pub mod application;
pub mod external_link;
pub mod password_reset;
pub mod refresh_token;
pub mod scope;
pub mod user;
