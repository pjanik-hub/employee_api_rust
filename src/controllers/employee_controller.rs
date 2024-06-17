use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};
use employee_api_rust::{
    create_employee_ez, get_employee_id, models::Employee, schema::employees, DbConn,
};
use rocket::{http::Status, serde::json::Json};
use uuid::Uuid;

#[get("/employees")]
pub async fn get_all_employees(conn: DbConn) -> Json<Vec<Employee>> {
    conn.run(|c| {
        employees::table
            .select(Employee::as_select())
            .load(c)
            .map(Json)
            .expect("Error parsing employees into JSON")
    })
    .await
}

#[get("/employees/<id>")]
pub async fn get_employee_by_id(conn: DbConn, id: &str) -> Result<Json<Employee>, Status> {
    let employee_id: Uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Status::NotAcceptable),
    };

    conn.run(move |c| get_employee_id(c, employee_id)).await
}

#[post("/employees/create", format = "application/json", data = "<employee>")]
pub async fn create_employee_db(conn: DbConn, employee: Employee) -> Status {
    conn.run(move |c| create_employee_ez(c, employee.id, &employee.first_name, &employee.last_name))
        .await
}
