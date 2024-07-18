use rocket::{Build, Rocket};

pub mod index;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/", index::index_routes())
}
