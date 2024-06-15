use diesel::prelude::*;
use employee_api_rust::establish_connection;
use employee_api_rust::models::*;

fn main() {
    use employee_api_rust::schema::employees::dsl::employees;
    let connection = &mut establish_connection();

    let results = employees
        .limit(5)
        .select(Employee::as_select())
        .load(connection)
        .expect("Error loading employees");

    println!("Displaying {} employees", results.len());
    for employee in results {
        println!(
            "{0}\n{1}, {2}",
            employee.id, employee.first_name, employee.last_name
        );
    }
}
