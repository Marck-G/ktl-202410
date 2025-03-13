use diesel::prelude::*;
use diesel::Queryable;
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::infra::postgres::schema::usr_main;
use crate::infra::postgres::schema::usr_metadata;

// Represents the `usr_main` table
#[derive(Debug, Queryable, Insertable, Selectable, Identifiable)]
#[diesel(table_name = usr_main)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub password_token: String,
    pub verified: bool,
    pub person: Option<Uuid>,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}

// Represents the `usr_metadata` table
#[derive(Debug, Queryable, Selectable, Insertable, Clone, Associations, Identifiable)]
#[diesel(table_name = usr_metadata)]
#[diesel(belongs_to(UserModel, foreign_key = user_id))]
pub struct UserMetadataModel {
    pub id: Uuid,
    pub key: String,
    pub value: Option<String>,
    pub user_id: Uuid,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}