extern crate diesel;
extern crate basic_rust_crud_app;

use self::models::*;
use diesel::prelude::*;
use basic_rust_crud_app::*;

fn main() {
    use self::schema::cars::dsl::*;

    let connection = establish_connection();
    let results = cars
        .load::<Car>(&connection)
        .expect("Error loading cars");

    println!("Displaying {} cars", results.len());
    for car in results {
        println!("----------------------");
        println!("{}", car.make);
        println!("{}", car.model);
        println!("{}", car.color);
        println!("{}", car.year);
    }
}