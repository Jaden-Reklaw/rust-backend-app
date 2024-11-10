mod api;
mod model;
mod repository;
mod data;

use api::user_api::get_user_routes;
use repository::user_repo;
use rocket::{launch, build};
use data::db_client::DBClient;

#[launch]
fn rocket() -> _ {
    let db = DBClient::init();
    let user_repo = user_repo::UserRepo::new(&db);
    build()
    .manage(db)
    .manage(user_repo)
        .mount("/", get_user_routes())
}