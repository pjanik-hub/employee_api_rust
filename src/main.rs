use controllers::employee_controller::{create_employee_db, get_all_employees, get_employee_by_id};
use employee_api_rust::DbConn;
use rocket::launch;

#[macro_use]
extern crate rocket;

mod controllers;

#[launch]
fn rocket_launch() -> _ {
    rocket::build().attach(DbConn::fairing()).mount(
        "/",
        routes![get_all_employees, create_employee_db, get_employee_by_id],
    )
}
