use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::Employee;
use rocket::http::Status;
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

pub fn create_employee_ez(conn: &mut PgConnection, first: &str, last: &str) -> Status {
    use crate::schema::employees;

    let new_uuid: Uuid = Uuid::new_v4();
    let new_employee: NewEmployee = NewEmployee {
        id: &new_uuid,
        first_name: first,
        last_name: last,
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
