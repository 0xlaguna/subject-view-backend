use rocket::Route;

pub mod create_subject;

pub fn subject_routes() -> Vec<Route> {
    routes![
        create_subject::req
    ]
}
