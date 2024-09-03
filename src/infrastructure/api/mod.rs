//! API module

mod errors;
mod extractors;
mod handlers;
mod layers;
mod logger;
pub mod response;
mod routes;
pub mod server;
mod user_cases;

use std::sync::LazyLock;
use tera::Tera;

pub static TEMPLATES: LazyLock<Result<Tera, tera::Error>> = LazyLock::new(|| {
    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec![".html", ".txt"]);
    Ok(tera)
});
