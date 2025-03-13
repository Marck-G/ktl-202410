use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub id: Uuid,
    pub key: String,
    pub value: String,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime
}

impl Metadata {
    pub fn new (id: Uuid, key: String, value: String) -> Metadata{
        Metadata {
            id: id,
            key: key.to_string(),
            value: value.to_string(),
            date_created: Utc::now().naive_utc(),
            date_modified: Utc::now().naive_utc()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserEntity {
    id: Uuid,
    email: String,
    password: String,
    pub verified: bool,
    pub person: Option<Uuid>,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
    pub metadata: Vec<Metadata>,
}

impl UserEntity {
    pub fn new(id: Uuid, email: String, password_token: String) -> UserEntity {
        UserEntity {
            id: id,
            email: email,
            verified: false,
            person: Some(Uuid::nil()),
            date_created: Utc::now().naive_utc(),
            date_modified: Utc::now().naive_utc(),
            password: password_token,
            metadata: Vec::new()
        }
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

}
