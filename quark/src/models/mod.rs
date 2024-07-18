mod users {
    pub mod user;
    pub mod session;
}

pub use users::*;
pub use user::Model as User;
pub use session::Model as Session;


mod subjects {
    pub mod subject;
}

pub use subjects::*;
