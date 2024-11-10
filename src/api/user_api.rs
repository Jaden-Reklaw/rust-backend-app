use rocket::{delete, get, post, put, routes, State, http::Status, serde::json::Json};
use mongodb::{results::{InsertOneResult, UpdateResult, DeleteResult}, bson::oid::ObjectId};
use crate::model::user::User;
use crate::repository::user_repo::UserRepo;


#[post("/users", data="<new_user>")]
pub fn create_user(
    db: &State<UserRepo>,
    new_user: Json<User>,) -> Result<Json<InsertOneResult>, Status>{
    let data = User {
        id: None,
        first_name: new_user.first_name.to_owned(),
        last_name: new_user.last_name.to_owned(),
        email: new_user.email.to_owned()
    };
    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users")]
fn list_users() -> &'static str {
    "List Users"
}

#[get("/users/<id>")]
fn get_user(id: usize) -> &'static str {
    "Get User"
}

#[put("/users/<id>")]
fn update_user(id: usize) -> &'static str {
    "Update User"
}

#[delete("/users/<id>")]
fn delete_user(id: usize) -> &'static str {
    "Delete User"
}

pub fn get_user_routes() -> Vec<rocket::Route> {
    routes![create_user, list_users, get_user, update_user, delete_user]
}