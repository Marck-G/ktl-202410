use diesel::prelude::*;
use crate::infra::postgres::schema::{prs_main, prs_metadata};
// Modelo para la tabla prs_main
#[derive(Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "prs_main"]
pub struct Person {
    pub id: uuid::Uuid,
    pub customer_id: Option<uuid::Uuid>,
    pub given_name: String,
    pub family_name: String,
    pub additional_name: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub gender: Option<String>,
    pub date_created: Option<chrono::NaiveDateTime>,
    pub date_modified: Option<chrono::NaiveDateTime>,
}

// Modelo para la tabla prs_metadata
#[derive(Queryable, Insertable, AsChangeset, Associations, Debug)]
#[belongs_to(Person, foreign_key = "person_id")]
#[table_name = "prs_metadata"]
pub struct PersonMeta {
    pub id: uuid::Uuid,
    pub person_id: uuid::Uuid,
    pub key: String,
    pub value: Option<String>,
    pub date_created: Option<chrono::NaiveDateTime>,
    pub date_modified: Option<chrono::NaiveDateTime>,
}

