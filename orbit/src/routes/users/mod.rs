use rocket::Route;

pub mod create_user;
pub mod me;

pub fn user_routes() -> Vec<Route> {
    routes![
        create_user::req,
        me::req,
    ]
}
