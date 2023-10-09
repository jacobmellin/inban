use super::schema::books;
use diesel::{Queryable, Insertable};
use serde::Serialize;


#[derive(Serialize, Queryable)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

