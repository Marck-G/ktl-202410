use errors::database::data::not_found::DataNotFound;
use errors::database::DatabaseError;
use errors::database::{data::query::QueryError};
use diesel::{ExpressionMethods, SelectableHelper, PgConnection, QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use uuid::Uuid;
use crate::domain::{mappers::UserMapper, repositories::user_repository::UserRepository};
use crate::domain::user_entity::UserEntity;
use crate::infra::postgres::models::user::UserModel;
use super::mapper::PGUserMapper;
use super::models::user::UserMetadataModel;



pub struct PgUserRepository<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> PgUserRepository<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { connection: conn }
    }
}

impl<'a> UserRepository for PgUserRepository<'a> {
    /// Find a user by ID and map it to `UserEntity`
    fn find_one_by_id(&mut self, user_id: Uuid) -> Result<UserEntity, QueryError> {
        use crate::infra::postgres::schema::{usr_main, usr_metadata};

    let user_model: UserModel = usr_main::table
        .filter(usr_main::id.eq(user_id))  // Explicit table prefix
        .select(UserModel::as_select())
        .first::<UserModel>(self.connection)
        .map_err(|e| QueryError::new(e.to_string().as_str()))?;
    // TODO: use inner join
    let metadata_models: Vec<UserMetadataModel> = usr_metadata::table
        .filter(usr_metadata::user_id.eq(user_model.id))  // Explicit table prefix
        .select(UserMetadataModel::as_select())
        .load(self.connection)
        .map_err(|e| QueryError::new(e.to_string().as_str()))?;
    Ok(PGUserMapper::to_domain(user_model, metadata_models))
    }

    fn list(&mut self, page: i32, limit: i32) -> Result<Vec<UserEntity>, QueryError> {
        use crate::infra::postgres::schema::{usr_main, usr_metadata};
    
        let offset_value = (page - 1).max(0) * limit;
    
        let user_models: Vec<UserModel> = usr_main::table
            .select(UserModel::as_select())
            .limit(limit.into())
            .offset(offset_value.into())
            .load(self.connection)
            .map_err(|e| QueryError::new(e.to_string().as_str()))?;
    
        let user_ids: Vec<Uuid> = user_models.iter().map(|user| user.id).collect();
        // Use inner join
        let metadata_models: Vec<UserMetadataModel> = usr_metadata::table
            .filter(usr_metadata::user_id.eq_any(&user_ids))  // Explicit table prefix
            .select(UserMetadataModel::as_select())
            .load(self.connection)
            .map_err(|e| QueryError::new(e.to_string().as_str()))?;
    
        let mut user_entities: Vec<UserEntity> = Vec::new();
    
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

    fn create(&mut self, user: UserEntity) -> Result<bool, QueryError> {
        use crate::infra::postgres::schema::{usr_main, usr_metadata};
        use diesel::RunQueryDsl;
        use diesel::insert_into;

    // Convert `UserEntity` to `UserModel`
    let (user_model, _) = PGUserMapper::to_infrastructure(&user.clone());

    // Insert the user into the `usr_main` table
    insert_into(usr_main::table)
        .values(&user_model)
        .execute(self.connection)
        .map_err(|e| QueryError::new(e.to_string().as_str()))?;

    // Convert metadata into `UserMetadataModel`
    let metadata_models: Vec<UserMetadataModel> = user
        .metadata
        .into_iter()
        .map(|m| UserMetadataModel {
            id: m.id,
            key: m.key,
            value: Some(m.value),
            user_id: user_model.id,
            date_created: m.date_created,
            date_modified: m.date_modified,
        })
        .collect();

    // Batch insert metadata if there are any
    if !metadata_models.is_empty() {
        insert_into(usr_metadata::table)
            .values(&metadata_models) // Pass Vec as a slice `&[]`
            .execute(self.connection)
            .map_err(|e| QueryError::new(e.to_string().as_str()))?;
    }

    Ok(true)
    }

    fn update(&mut self, user: UserEntity) -> Result<bool, QueryError> {
        use crate::infra::postgres::schema::{usr_main, usr_metadata};
        use diesel::{update, RunQueryDsl};

        // Convert `UserEntity` to `UserModel`
        let (user_model, metadata_models) = PGUserMapper::to_infrastructure(&user);

        // Update the user in `usr_main`
        update(usr_main::table.filter(usr_main::id.eq(user_model.id)))
            .set((
                usr_main::email.eq(user_model.email),
                usr_main::verified.eq(user_model.verified),
                usr_main::person.eq(user_model.person),
                usr_main::date_modified.eq(user_model.date_modified),
            ))
            .execute(self.connection)
            .map_err(|e| QueryError::new(&e.to_string()))?;

        // Remove existing metadata for this user (optional: to prevent duplication)
        diesel::delete(usr_metadata::table.filter(usr_metadata::user_id.eq(user_model.id)))
            .execute(self.connection)
            .map_err(|e| QueryError::new(&e.to_string()))?;

        // Insert updated metadata
        if !metadata_models.is_empty() {
            diesel::insert_into(usr_metadata::table)
                .values(&metadata_models)
                .execute(self.connection)
                .map_err(|e| QueryError::new(&e.to_string()))?;
        }

        Ok(true)
    }

    fn change_password(&mut self, user_id: Uuid, new_password: String) -> Result<bool, DatabaseError> {
        use crate::infra::postgres::schema::usr_main;
        use diesel::{update, RunQueryDsl};
        let updated_rows = update(usr_main::table.filter(usr_main::id.eq(user_id)))
            .set((
                usr_main::password_token.eq(new_password),
                usr_main::date_modified.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(self.connection)
            .map_err(|e| DatabaseError::QueryError(QueryError::new(&e.to_string())))?;

        if updated_rows == 0 {
            return Err(DatabaseError::NotFound(DataNotFound::new("User not found")));
        }

        Ok(true)
    }

    fn filter(&mut self, user: UserEntity) -> Result<Vec<UserEntity>, QueryError> {
        use crate::infra::postgres::schema::{usr_main, usr_metadata};
        let mut query = usr_main::table.into_boxed();

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

    fn filter_by_metadata(&mut self, metadata: Vec<(String, String)>) -> Result<Vec<UserEntity>, QueryError> {
        use crate::infra::postgres::schema::{usr_main, usr_metadata};
        let mut query = usr_main::table
            .inner_join(usr_metadata::table.on(usr_main::id.eq(usr_metadata::user_id)))
            .into_boxed();

        for (key, value) in metadata.iter() {
            query = query.filter(usr_metadata::key.eq(key).and(usr_metadata::value.eq(value)));
        }

        let user_models: Vec<UserModel> = query
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

    fn update_metadata(&mut self, update_user_id: Uuid, update_key: String, update_value: Option<String>) -> Result<bool, QueryError> {
        use crate::infra::postgres::schema::usr_metadata::dsl::*;
    
        diesel::update(usr_metadata.filter(user_id.eq(update_user_id).and(key.eq(update_key))))
            .set(value.eq(update_value))
            .execute(self.connection)
            .map_err(|e| QueryError::new(e.to_string().as_str()))?;
    
        Ok(true)
    }
    
}