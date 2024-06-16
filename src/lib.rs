use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::Employee;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::env;
use uuid::Uuid;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Could not connect to Postgres db."))
}

pub fn create_employee_ez(
    conn: &mut PgConnection,
    new_uuid: Uuid,
    first: &str,
    last: &str,
) -> Status {
    use crate::schema::employees;

    let new_employee: Employee = Employee {
        id: new_uuid,
        first_name: first.to_string(),
        last_name: last.to_string(),
    };

    let result = diesel::insert_into(employees::table)
        .values(&new_employee)
        .returning(Employee::as_returning())
        .get_result(conn);

    match result {
        Ok(_) => Status::Ok,
        Err(_) => Status::BadRequest,
    }
}

pub fn get_employee_id(
    conn: &mut PgConnection,
    employee_id: Uuid,
) -> Result<Json<Employee>, Status> {
    use crate::schema::employees;
    use schema::employees::id;

    let employee = employees::table
        .filter(id.eq(employee_id))
        .first::<Employee>(conn);

    match employee {
        Ok(employee) => Ok(Json(employee)),
        Err(_) => Err(Status::NotFound),
    }
}
