use diesel::prelude::*;
use rocket::data::{self, ByteUnit, Data, FromData};
use rocket::http::{ContentType, Status};
use rocket::outcome::Outcome;
use rocket::request::Request;
use rocket::tokio::io::AsyncReadExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::employees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employee {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug)]
pub enum EmployeeError {
    JsonError,
    TooLarge,
    NoColon,
    InvalidUuid,
    Io(std::io::Error),
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Employee {
    type Error = EmployeeError;

    /// required implementation for parsing the Employee data (via json)
    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use EmployeeError::*;

        // ensure content is the right type
        let content_type = ContentType::JSON;

        if req.content_type() != Some(&content_type) {
            return Outcome::Forward((data, Status::UnsupportedMediaType));
        }

        let limit = req.limits().get("employee").unwrap_or(ByteUnit::Byte(256));
        let limit_usize = limit.as_u64() as usize;

        // read the data into a string
        let mut buffer = String::with_capacity(limit_usize);
        if let Err(e) = data.open(limit).read_to_string(&mut buffer).await {
            return Outcome::Error((Status::InternalServerError, Io(e)));
        }

        // deserialize the string into a Employee struct
        let new_employee: Employee = match serde_json::from_str(&buffer) {
            Ok(emp) => emp,
            Err(_) => return Outcome::Error((Status::UnprocessableEntity, JsonError)),
        };

        Outcome::Success(new_employee)
    }
}
