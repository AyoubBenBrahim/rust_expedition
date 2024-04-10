
use diesel::{ Queryable, Insertable, Selectable };
use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

// no longer needed with the api part
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}
// pub struct NewUser<'a> {
//     pub username: &'a str,
//     pub email: &'a str,
// }
