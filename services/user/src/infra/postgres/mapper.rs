use std::any::Any;

use crate::domain::{mappers::UserMapper, user_entity::{Metadata, UserEntity}};

use super::models::user::{UserMetadataModel, UserModel};

pub struct PGUserMapper{}

impl UserMapper<UserModel, UserMetadataModel> for PGUserMapper {
    /// Convert `UserModel` + `Vec<UserMetadataModel>` to `UserEntity`
    fn to_domain(model: UserModel, metadata_models: Vec<UserMetadataModel>) -> UserEntity {
        let mut entity = UserEntity::new(model.id, model.email, model.password_token);
        entity.date_created = model.date_created;
        entity.date_modified = model.date_modified;
        entity.person = model.person;
        entity.verified = model.verified;

        // Convert metadata models to domain objects
        entity.metadata = metadata_models
            .into_iter()
            .map(|m| Metadata {
                id: m.id,
                key: m.key,
                value: m.value.unwrap_or_default(),
                date_created: m.date_created,
                date_modified: m.date_modified,
            })
            .collect();

        return entity;
    }

    /// Convert `UserEntity` to `UserModel` + `Vec<UserMetadataModel>`
    fn to_infrastructure(entity: &UserEntity) -> (UserModel, Vec<UserMetadataModel>) {
        let user_model = UserModel {
            id: entity.get_id(),
            email: entity.get_email(),
            password_token: entity.get_password(),
            verified: entity.verified,
            person: entity.person,
            date_created: entity.date_created,
            date_modified: entity.date_modified,
        };

        // Convert metadata to infrastructure models
        let metadata_models = entity.metadata.iter().map(|m| UserMetadataModel {
            id: m.id,
            key: m.key.clone(),
            value: Some(m.value.clone()),
            user_id: entity.get_id(),
            date_created: m.date_created,
            date_modified: m.date_modified,
        }).collect();

        (user_model, metadata_models)
    }
}