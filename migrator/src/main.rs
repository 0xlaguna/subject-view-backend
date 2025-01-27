use sea_orm_migration::prelude::*;
use migrator::Migrator;

#[async_std::main]
async fn main() {
    let key = "DATABASE_URL";
    if std::env::var(key).is_err() {
        let figment = rocket::Config::figment();
        let database_url: String = figment
            .extract_inner("databases.sea_orm.url")
            .expect("Cannot find Database URL in Rocket.toml");
        std::env::set_var(key, database_url);
    }

    cli::run_cli(Migrator).await;
}
