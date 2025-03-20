use std::sync::Arc;

use super::crypto::PgPasswordTools;
use super::mapper::PGUserMapper;
use super::models::user::UserMetadataModel;
use crate::domain::crypto::PasswordTools;
use crate::domain::user_entity::UserEntity;
use crate::domain::{mappers::UserMapper, repositories::user_repository::UserRepository};
use crate::infra::postgres::models::user::UserModel;
use diesel::{insert_into, prelude::*};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use errors::database::data::not_found::DataNotFound;
use errors::database::data::query::QueryError;
use errors::database::{DatabaseError, InvalidData};
use tokio::sync::{Mutex, MutexGuard};
use uuid::Uuid;

pub struct PgUserRepository {
    connection: Arc<Mutex<PgConnection>>,
    password_helper: Arc<Mutex<PgPasswordTools>>, // Use Arc<Mutex<T>> instead of mutable reference
}
impl PgUserRepository {
    pub fn new(connection: Arc<Mutex<PgConnection>>, password_helper: Arc<Mutex<PgPasswordTools>>) -> Self {
        Self { connection, password_helper }
    }

    async fn get_connection(&self) -> Result<MutexGuard<'_, PgConnection>, QueryError> {
        Ok(self.connection.lock().await)
    }
}

impl UserRepository for PgUserRepository {
    async fn find_one_by_id(&mut self, user_id: Uuid) -> Result<UserEntity, QueryError> {
        use crate::infra::postgres::schema::{usr_main, usr_metadata};

       let mut conn = self.get_connection().await?;


        let user_model: UserModel = usr_main::table
            .filter(usr_main::id.eq(user_id))
            .filter(usr_main::deleted.eq(false))
            .first(&mut *conn)
            .map_err(|e| QueryError::new(&e.to_string()))?;

        let metadata_models: Vec<UserMetadataModel> = usr_metadata::table
            .filter(usr_metadata::user_id.eq(user_model.id))
            .load(&mut *conn)
            .map_err(|e| QueryError::new(&e.to_string()))?;

        Ok(PGUserMapper::to_domain(user_model, metadata_models))
    }

    async fn list(&mut self, page: i32, limit: i32) -> Result<Vec<UserEntity>, QueryError> {
        use crate::infra::postgres::schema::{usr_main, usr_metadata};

       let mut conn = self.get_connection().await?;

        let offset_value = (page - 1).max(0) * limit;

        let user_models: Vec<UserModel> = usr_main::table
            .filter(usr_main::deleted.eq(false))
            .limit(limit.into())
            .offset(offset_value.into())
            .load(&mut *conn)
            .map_err(|e| QueryError::new(&e.to_string()))?;

        let user_ids: Vec<Uuid> = user_models.iter().map(|user| user.id).collect();

        let metadata_models: Vec<UserMetadataModel> = usr_metadata::table
            .filter(usr_metadata::user_id.eq_any(&user_ids))
            .load(&mut *conn)
            .map_err(|e| QueryError::new(&e.to_string()))?;

        let user_entities = user_models
            .into_iter()
            .map(|user| {
                let metadata = metadata_models.iter()
                    .filter(|m| m.user_id == user.id)
                    .cloned()
                    .collect();
                PGUserMapper::to_domain(user, metadata)
            })
            .collect();

        Ok(user_entities)
    }

    async fn create(&mut self, user: UserEntity) -> Result<bool, QueryError> {
        use crate::infra::postgres::schema::{usr_main, usr_metadata};
        use diesel::insert_into;

       let mut conn = self.get_connection().await?;


        let (mut user_model, _) = PGUserMapper::to_infrastructure(&user);
        user_model.password_token = self.password_helper
            .lock()
            .unwrap()
            .encrypt(user.get_password())
            .map_err(|e| QueryError::new(&e.to_string()))?;

        insert_into(usr_main::table)
            .values(&user_model)
            .execute(&mut *conn)
            .map_err(|e| QueryError::new(&e.to_string()))?;

        let metadata_models: Vec<UserMetadataModel> = user.metadata.into_iter()
            .map(|m| UserMetadataModel {
                id: m.id, key: m.key, value: Some(m.value),
                user_id: user_model.id,
                date_created: m.date_created, date_modified: m.date_modified,
            })
            .collect();

        if !metadata_models.is_empty() {
            insert_into(usr_metadata::table)
                .values(&metadata_models)
                .execute(&mut *conn)
                .map_err(|e| QueryError::new(&e.to_string()))?;
        }

        Ok(true)
    }

    async fn update(&mut self, user: UserEntity) -> Result<bool, QueryError> {
        use crate::infra::postgres::schema::{usr_main, usr_metadata};
        use diesel::{update, delete};

       let mut conn = self.get_connection().await?;

        let (user_model, metadata_models) = PGUserMapper::to_infrastructure(&user);

        update(usr_main::table.filter(usr_main::id.eq(user_model.id)))
            .set((
                usr_main::email.eq(user_model.email),
                usr_main::verified.eq(user_model.verified),
                usr_main::person.eq(user_model.person),
                usr_main::date_modified.eq(user_model.date_modified),
            ))
            .execute(&mut *conn)
            .map_err(|e| QueryError::new(&e.to_string()))?;

        delete(usr_metadata::table.filter(usr_metadata::user_id.eq(user_model.id)))
            .execute(&mut *conn)
            .map_err(|e| QueryError::new(&e.to_string()))?;

        if !metadata_models.is_empty() {
            insert_into(usr_metadata::table)
                .values(&metadata_models)
                .execute(&mut *conn)
                .map_err(|e| QueryError::new(&e.to_string()))?;
        }

        Ok(true)
    }

    async fn change_password(&mut self, user_id: Uuid, new_password: String) -> Result<bool, DatabaseError> {
        use crate::infra::postgres::schema::usr_main;
        use diesel::update;
        
       let mut conn = self.get_connection().await.map_err(op);

        let encrypted_password = self.password_helper.lock().await.encrypt(new_password)
            .map_err(|e| DatabaseError::InvalidData(InvalidData::new(&e.to_string())))?;

        let updated_rows = update(usr_main::table.filter(usr_main::id.eq(user_id)))
            .set((
                usr_main::password_token.eq(encrypted_password),
                usr_main::date_modified.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut *conn)
            .map_err(|e| DatabaseError::QueryError(QueryError::new(&e.to_string())))?;

        if updated_rows == 0 {
            return Err(DatabaseError::NotFound(DataNotFound::new("User not found")));
        }

        Ok(true)
    }

    async fn soft_delete(&mut self, user_id: Uuid) -> Result<bool, QueryError> {
        use crate::infra::postgres::schema::usr_main;
        use diesel::update;
        
        let mut conn = self.get_connection().await?;

        update(usr_main::table.filter(usr_main::id.eq(user_id)))
            .set(usr_main::deleted.eq(true))
            .execute(&mut *conn)
            .map_err(|e| QueryError::new(&e.to_string()))?;

        Ok(true)
    }

    async fn restore(&mut self, user_id: Uuid) -> Result<bool, QueryError> {
        use crate::infra::postgres::schema::usr_main;
        use diesel::update;
        
        let mut conn = self.get_connection().await?;


        update(usr_main::table.filter(usr_main::id.eq(user_id)))
            .set(usr_main::deleted.eq(false))
            .execute(&mut *conn)
            .map_err(|e| QueryError::new(&e.to_string()))?;

        Ok(true)
    }

    fn filter(&mut self, user: UserEntity) -> Result<Vec<UserEntity>, QueryError> {
        use crate::infra::postgres::schema::{usr_main, usr_metadata};
        let mut query = usr_main::table.into_boxed();
        query = query.filter(usr_main::deleted.eq(false));
        if !user.get_email().is_empty() {
            query = query.filter(usr_main::email.eq(user.get_email()));
        }

        if let Some(person_id) = user.person {
            query = query.filter(usr_main::person.eq(person_id));
        }

        if user.verified {
            query = query.filter(usr_main::verified.eq(true));
        }

        let user_models: Vec<UserModel> = query
            .select(UserModel::as_select())
            .load(self.connection)
            .map_err(|e| QueryError::new(e.to_string().as_str()))?;

        let user_ids: Vec<Uuid> = user_models.iter().map(|u| u.id).collect();

        let metadata_models: Vec<UserMetadataModel> = usr_metadata::table
            .filter(usr_metadata::user_id.eq_any(user_ids))
            .select(UserMetadataModel::as_select())
            .load(self.connection)
            .map_err(|e| QueryError::new(e.to_string().as_str()))?;

        let user_entities = user_models
            .into_iter()
            .map(|user_model| {
                let user_metadata: Vec<UserMetadataModel> = metadata_models
                    .iter()
                    .filter(|meta| meta.user_id == user_model.id)
                    .cloned()
                    .collect();
                PGUserMapper::to_domain(user_model, user_metadata)
            })
            .collect();

        Ok(user_entities)
    }

    fn filter_by_metadata(
        &mut self,
        metadata: Vec<(String, String)>,
    ) -> Result<Vec<UserEntity>, QueryError> {
        use crate::infra::postgres::schema::{usr_main, usr_metadata};
        let mut query = usr_main::table
            .inner_join(usr_metadata::table.on(usr_main::id.eq(usr_metadata::user_id)))
            .into_boxed();

        for (key, value) in metadata.iter() {
            query = query.filter(usr_metadata::key.eq(key).and(usr_metadata::value.eq(value)));
        }

        let user_models: Vec<UserModel> = query
            .filter(usr_main::deleted.eq(false))
            .select(UserModel::as_select())
            .distinct()
            .load(self.connection)
            .map_err(|e| QueryError::new(e.to_string().as_str()))?;

        let user_ids: Vec<Uuid> = user_models.iter().map(|user| user.id).collect();

        let metadata_models: Vec<UserMetadataModel> = usr_metadata::table
            .filter(usr_metadata::user_id.eq_any(&user_ids))
            .select(UserMetadataModel::as_select())
            .load(self.connection)
            .map_err(|e| QueryError::new(e.to_string().as_str()))?;

        let mut user_entities = Vec::new();

        for user_model in user_models {
            let user_metadata: Vec<UserMetadataModel> = metadata_models
                .iter()
                .filter(|meta| meta.user_id == user_model.id)
                .cloned()
                .collect();

            user_entities.push(PGUserMapper::to_domain(user_model, user_metadata));
        }

        Ok(user_entities)
    }

    fn email_exists(&mut self, email_to_check: String) -> Result<bool, QueryError> {
        use crate::infra::postgres::schema::usr_main::dsl::*;

        let exists = usr_main
            .filter(email.eq(email_to_check))
            .select(id)
            .first::<Uuid>(self.connection)
            .optional()
            .map_err(|e| QueryError::new(e.to_string().as_str()))?
            .is_some();

        Ok(exists)
    }

    fn update_metadata(
        &mut self,
        update_user_id: Uuid,
        update_key: String,
        update_value: Option<String>,
    ) -> Result<bool, QueryError> {
        use crate::infra::postgres::schema::usr_metadata::dsl::*;

        diesel::update(usr_metadata.filter(user_id.eq(update_user_id).and(key.eq(update_key))))
            .set(value.eq(update_value))
            .execute(self.connection)
            .map_err(|e| QueryError::new(e.to_string().as_str()))?;

        Ok(true)
    }

}