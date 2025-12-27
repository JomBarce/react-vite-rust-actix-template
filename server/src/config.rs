use dotenv::dotenv;
use std::env;

pub fn init() {
    dotenv().ok();
}

pub fn mongo_uri() -> String {
    env::var("MONGO_URI").expect("MONGO_URI not set")
}

pub fn mongo_db() -> String {
    env::var("MONGO_DB").expect("MONGO_DB not set")
}
