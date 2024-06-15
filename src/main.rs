use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};
use employee_api_rust::{establish_connection, models::Employee, schema::employees};
use rocket::{launch, serde::json::Json};

#[macro_use]
extern crate rocket;

#[get("/employees")]
fn get_all_employees() -> Json<Vec<Employee>> {
    let conn = &mut establish_connection();

    employees::table
        .select(Employee::as_select())
        .load(conn)
        .map(Json)
        .expect("Error parsing employees into JSON")
}

#[launch]
fn rocket_launch() -> _ {
    rocket::build().mount("/", routes![get_all_employees])
}
