use diesel::{ExpressionMethods, PgConnection, SelectableHelper};
use errors::database::{data::query::QueryError};
use uuid::Uuid;
use crate::domain::{mappers::UserMapper, repositories::user_repository::UserRepository};
use crate::domain::user_entity::UserEntity;
use crate::infra::postgres::models::user::UserModel;
use super::mapper::PGUserMapper;
use super::models::user::UserMetadataModel;
use diesel::{QueryDsl, RunQueryDsl};



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
}