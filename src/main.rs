use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};
use employee_api_rust::{
    create_employee_ez, establish_connection, get_employee_id, models::Employee, schema::employees,
};
use rocket::{http::Status, launch, serde::json::Json};
use uuid::Uuid;

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

#[get("/employees/<id>")]
fn get_employee_by_id(id: &str) -> Result<Json<Employee>, Status> {
    let conn = &mut establish_connection();

    let employee_id: Uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Status::NotAcceptable),
    };

    get_employee_id(conn, employee_id)
}

#[post("/employees/create", format = "application/json", data = "<employee>")]
fn create_employee(employee: Employee) -> Status {
    let conn = &mut establish_connection();

    create_employee_ez(conn, employee.id, &employee.first_name, &employee.last_name)
}

#[launch]
fn rocket_launch() -> _ {
    rocket::build().mount(
        "/",
        routes![get_all_employees, create_employee, get_employee_by_id],
    )
}
