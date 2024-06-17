use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use models::Employee;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use uuid::Uuid;

pub mod models;
pub mod schema;

//                -- typing this out each time hurts
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConn = PooledConnection<ConnectionManager<PgConnection>>;

#[database("employee_db")]
pub struct DbConn(diesel::PgConnection);

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
