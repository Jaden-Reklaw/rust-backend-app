use crate::model::user::User;
use mongodb::sync::{Client, Collection, Database};


pub struct DBClient {
    pub user_col: Collection<User>,
}

impl DBClient {
    pub fn init() -> Self {
        let uri = "mongodb://root:example@localhost:27017";
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let user_col: Collection<User> = db.collection("User");
        DBClient { user_col }
    }

}
