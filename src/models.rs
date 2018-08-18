use super::schema::cars;

#[derive(Queryable)]
pub struct Car {
    pub id: i32,
    pub make: String,
    pub model: String,
    pub color: String,
    pub year: i32,
}

#[derive(Insertable)]
#[table_name = "cars"]
pub struct NewCar<'a> {
    pub make: &'a str,
    pub model: &'a str,
    pub color: &'a str,
    pub year: &'a i32,
}