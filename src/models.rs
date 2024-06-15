use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::employees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employee {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}
