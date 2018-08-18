extern crate diesel;
extern crate basic_rust_crud_app;

use self::basic_rust_crud_app::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Make?");
    let mut make = String::new();
    stdin().read_line(&mut make).unwrap();
    let make = &make[..(make.len() - 1)]; // Drop newline character
    
    println!("Model?");
    let mut model = String::new();
    stdin().read_line(&mut model).unwrap();
    let model = &model[..(model.len() - 1)];

    println!("Color?");
    let mut color = String::new();
    stdin().read_line(&mut color).unwrap();
    let color = &color[..(color.len() - 1)];

    println!("Year?");
    let mut year = String::new();
    stdin().read_line(&mut year).unwrap();
    let year = &year[..(year.len() - 1)];

    let parsed_year = year.parse::<i32>().unwrap();

    let _ = create_car(&connection, make, model, color, &parsed_year);
    println!("Saved new car!");
}