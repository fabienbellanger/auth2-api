pub mod adapters;
pub mod config;
pub mod domain;
pub mod infrastructure;

#[macro_use]
extern crate tracing;

#[macro_export]
macro_rules! err {
    ( $error:expr ) => {
        $error
    };
    
    ( $error:expr, $message:expr ) => {
        error!("in macro => {}", $message);
        $error
    }
}
