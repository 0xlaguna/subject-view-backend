use rocket_dyn_templates::{context, Template};

/// Index
#[get("/")]
pub async fn req() -> Template {
    Template::render("index", context! {
        hi: "Hi",
    })
}
