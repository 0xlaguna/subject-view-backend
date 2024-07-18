use rocket::Route;

pub mod create_user;

pub fn user_routes() -> Vec<Route> {
    routes![
        create_user::req,
    ]
}
