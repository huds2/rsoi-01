pub mod person;
pub mod repository;
pub mod server;
pub use person::*;
pub use repository::*;
pub use server::*;
use custom_error::custom_error;

custom_error!{pub WebappError
    NotFoundError                            = "Person was not found",
    AlreadyExistsError                       = "Person already exists",
}

#[macro_export]
macro_rules! arc{
    ($a:expr)=>{
        {
            std::sync::Arc::new(tokio::sync::Mutex::new($a))
        }
    }
}
