use async_trait::async_trait;
use uuid::Uuid;

use crate::{domain::{entities::Person, mappers::Mapper, repository::PersonRepository}, infra::postgres::mapper::PgMapper};
use crate::infra::postgres::schema::{prs_main, prs_metadata};
use super::{models::models::{Person as InfraPerson, PersonMeta as InfraPersonMeta}, types::PgConnectionPool};
use diesel::prelude::*; // Para las funciones de consulta y operaciones básicas

pub struct PgPersonRepository {
    pub db_pool: PgConnectionPool,
}


#[async_trait]
impl PersonRepository for PgPersonRepository {
    async fn get_by_id(&mut self, id_to_search: Uuid) -> Result<Option<Person>, String> {

        let connection = &mut self.db_pool;

        // Obtener la persona por ID
        let infra_person: InfraPerson = match prs_main::table
            .filter(prs_main::id.eq(id_to_search))
            .first::<InfraPerson>(connection)
        {
            Ok(person) => person,
            Err(diesel::result::Error::NotFound) => return Ok(None),
            Err(e) => return Err(format!("Error al obtener la persona: {:?}", e)),
        };

        // Obtener los metadatos asociados a la persona
        let infra_metadata: Vec<InfraPersonMeta> = match prs_metadata::table
            .filter(prs_metadata::person_id.eq(id_to_search))
            .load::<InfraPersonMeta>(connection)
        {
            Ok(metadata) => metadata,
            Err(e) => return Err(format!("Error al obtener metadatos: {:?}", e)),
        };
        // Convertir a entidad de dominio
        let domain_person = PgMapper::to_domain(infra_person, infra_metadata);

        Ok(Some(domain_person))
    }

    async fn get_by_ids(&mut self, ids: Vec<Uuid>) -> Result<Vec<Person>, String> {
        // Obtener una conexión del pool
        let connection = &mut self.db_pool;

        // Obtener las personas por los IDs
        let infra_persons: Vec<InfraPerson> = match prs_main::table
            .filter(prs_main::id.eq_any(ids))  // Filtra por cualquier ID del vector
            .load::<InfraPerson>(connection)
        {
            Ok(persons) => persons,
            Err(e) => return Err(format!("Error al obtener las personas: {:?}", e)),
        };

        // Obtener los metadatos asociados a cada persona
        let mut result = Vec::new();
        for infra_person in infra_persons {
            let infra_metadata: Vec<InfraPersonMeta> = match prs_metadata::table
                .filter(prs_metadata::person_id.eq(infra_person.id))
                .load::<InfraPersonMeta>(connection)
            {
                Ok(metadata) => metadata,
                Err(e) => return Err(format!("Error al obtener metadatos para persona {}: {:?}", infra_person.id, e)),
            };

            // Convertir a entidad de dominio y agregar al resultado
            let domain_person = PgMapper::to_domain(infra_person, infra_metadata);
            result.push(domain_person);
        }

        Ok(result)
    }
    
}