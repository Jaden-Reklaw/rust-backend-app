use rocket::{self, get, routes, serde::json::Json};

#[get("/")]
fn index() -> Json<String> {
    Json("Hello, world!".to_string())
}
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await
        .unwrap();
}