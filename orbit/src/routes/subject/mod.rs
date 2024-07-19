use rocket::Route;

pub mod create_subject;
pub mod list_subject;
pub mod delete_subject;

pub fn subject_routes() -> Vec<Route> {
    routes![
        create_subject::req,
        list_subject::req,
        delete_subject::req,
    ]
}
