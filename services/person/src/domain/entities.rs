use uuid::Uuid;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone)]
pub struct Metadata {
    pub id: Uuid,
    pub key: String,
    pub value: Option<String>,
    pub date_created: Option<NaiveDateTime>,
    pub date_modified: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
pub struct Person {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub given_name: String,
    pub family_name: String,
    pub additional_name: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<String>,
    pub date_created: Option<NaiveDateTime>,
    pub date_modified: Option<NaiveDateTime>,
    pub metadata: Vec<Metadata>, // Relación con metadata
}
