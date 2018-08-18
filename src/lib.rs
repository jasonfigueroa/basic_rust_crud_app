#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::NewCar;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_car(conn: &SqliteConnection, make: &str, model: &str, color: &str, year: &i32) -> usize {
    use schema::cars;

    let new_car = NewCar { make, model, color, year };

    diesel::insert_into(cars::table)
        .values(&new_car)
        .execute(conn)
        .expect("Error saving new car")
}