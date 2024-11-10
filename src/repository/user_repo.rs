use mongodb::{
    sync::Collection,
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};

use crate::{data::db_client::DBClient, model::user::User};

pub struct UserRepo {
    col: Collection<User>,
}

impl UserRepo {
    pub fn new(db_client: &DBClient) -> Self {
        UserRepo {
            col: db_client.user_col.clone(),
        }
    }
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            first_name: new_user.first_name,
            last_name: new_user.last_name,
            email: new_user.email,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error Creating User");
        Ok(user)
    }

    pub fn get_user(&self, id: &str) -> Result<User, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id" : object_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user info");
        Ok(user_detail.unwrap())
    }

    pub fn update_user(&self, id: &str, new_user: User) -> Result<UpdateResult, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id" : object_id};
        let new_doc = doc! {
            "$set": {
                "id" : new_user.id,
                "first_name" : new_user.first_name,
                "last_name" : new_user.last_name,
                "email" : new_user.email
            }
        };
        let update_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(update_doc)
    }

    pub fn delete_user(&self, id: &str) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }
}
