use async_trait::async_trait;
use chrono::Utc;
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

    async fn list(&mut self, page: u64, limit: u64) -> Result<Vec<Person>, String> {
        let connection = &mut self.db_pool;

        // Calcular el OFFSET según la página y el límite
        let offset = (page - 1) * limit;

        // Obtener las personas con LIMIT y OFFSET
        let infra_persons: Vec<InfraPerson> = match prs_main::table
            .limit(limit as i64)
            .offset(offset as i64)
            .load::<InfraPerson>(connection)
        {
            Ok(persons) => persons,
            Err(e) => return Err(format!("Error al obtener las personas: {:?}", e)),
        };

        // Obtener los metadatos para cada persona
        let mut result = Vec::new();
        for infra_person in infra_persons {
            let infra_metadata: Vec<InfraPersonMeta> = match prs_metadata::table
                .filter(prs_metadata::person_id.eq(infra_person.id))
                .load::<InfraPersonMeta>(connection)
            {
                Ok(metadata) => metadata,
                Err(e) => return Err(format!("Error al obtener metadatos para persona {}: {:?}", infra_person.id, e)),
            };

            let domain_person = PgMapper::to_domain(infra_person, infra_metadata);
            result.push(domain_person);
        }

        Ok(result)
    }    

    async fn get_by_metadata(&mut self, key: String, value: String) -> Result<Vec<Person>, String> {
        let connection = &mut self.db_pool;

        // Realizar la consulta con JOIN entre prs_main y prs_metadata
        let infra_persons: Vec<InfraPerson> = match prs_main::table
            .inner_join(prs_metadata::table)  // Hacemos un join entre prs_main y prs_metadata
            .filter(prs_metadata::key.eq(key)) // Filtramos por la clave
            .filter(prs_metadata::value.eq(value)) // Filtramos por el valor
            .select(prs_main::all_columns) // Seleccionamos las columnas de prs_main
            .load::<InfraPerson>(connection)
        {
            Ok(persons) => persons,
            Err(e) => return Err(format!("Error al obtener las personas: {:?}", e)),
        };

        // Recuperamos los metadatos asociados a cada persona y los convertimos en dominio
        let mut result = Vec::new();
        for infra_person in infra_persons {
            let infra_metadata: Vec<InfraPersonMeta> = match prs_metadata::table
                .filter(prs_metadata::person_id.eq(infra_person.id))
                .load::<InfraPersonMeta>(connection)
            {
                Ok(metadata) => metadata,
                Err(e) => return Err(format!("Error al obtener metadatos para persona {}: {:?}", infra_person.id, e)),
            };

            let domain_person = PgMapper::to_domain(infra_person, infra_metadata);
            result.push(domain_person);
        }

        Ok(result)
    }

    async fn create(&mut self, person: Person) -> Result<Person, String> {
        let connection = &mut self.db_pool;

        // Convertimos la persona de dominio a infraestructura
        let (infra_person, infra_metadata) = PgMapper::to_infrastructure(person.clone());

        // Insertar la persona en prs_main
        match diesel::insert_into(prs_main::table)
            .values(&infra_person)
            .execute(connection)
        {
            Ok(_) => {}
            Err(e) => return Err(format!("Error al insertar la persona: {:?}", e)),
        }

        // Insertar los metadatos en prs_metadata (si hay metadatos)
        if !infra_metadata.is_empty() {
            match diesel::insert_into(prs_metadata::table)
                .values(&infra_metadata)
                .execute(connection)
            {
                Ok(_) => {}
                Err(e) => return Err(format!("Error al insertar metadatos: {:?}", e)),
            }
        }

        Ok(person)
    }

    async fn update(&mut self, person: Person) -> Result<Person, String> {
        let connection = &mut self.db_pool;

        // Convertimos la entidad de dominio a los modelos de infraestructura
        let (infra_person, infra_metadata) = PgMapper::to_infrastructure(person.clone());

        // Actualizar la persona en `prs_main`
        match diesel::update(prs_main::table.filter(prs_main::id.eq(infra_person.id)))
            .set(&infra_person)
            .execute(connection)
        {
            Ok(0) => return Err(format!("Persona con ID {:?} no encontrada", infra_person.id)),
            Ok(_) => {}
            Err(e) => return Err(format!("Error al actualizar la persona: {:?}", e)),
        }

        // Eliminar metadatos existentes para la persona
        match diesel::delete(prs_metadata::table.filter(prs_metadata::person_id.eq(infra_person.id)))
            .execute(connection)
        {
            Ok(_) => {}
            Err(e) => return Err(format!("Error al eliminar metadatos anteriores: {:?}", e)),
        }

        // Insertar los nuevos metadatos (si existen)
        if !infra_metadata.is_empty() {
            match diesel::insert_into(prs_metadata::table)
                .values(&infra_metadata)
                .execute(connection)
            {
                Ok(_) => {}
                Err(e) => return Err(format!("Error al insertar nuevos metadatos: {:?}", e)),
            }
        }

        Ok(person)
    }

    async fn delete(&mut self, person_id: Uuid) -> Result<(), String> {
        let connection = &mut self.db_pool;

        // Iniciar una transacción para asegurar consistencia
        let transaction_result = connection.transaction::<_, diesel::result::Error, _>(|conn| {
            // Eliminar los metadatos asociados a la persona
            diesel::delete(prs_metadata::table.filter(prs_metadata::person_id.eq(person_id)))
                .execute(conn)?;

            // Eliminar la persona en `prs_main`
            let deleted_count = diesel::delete(prs_main::table.filter(prs_main::id.eq(person_id)))
                .execute(conn)?;

            if deleted_count == 0 {
                return Err(diesel::result::Error::NotFound);
            }

            Ok(())
        });

        match transaction_result {
            Ok(_) => Ok(()),
            Err(diesel::result::Error::NotFound) => Err(format!("Persona con ID {:?} no encontrada", person_id)),
            Err(e) => Err(format!("Error al eliminar la persona: {:?}", e)),
        }
    }

    async fn delete_metadata(&mut self, metadata_id: Uuid) -> Result<(), String> {
        let connection = &mut self.db_pool;

        let deleted_count = diesel::delete(prs_metadata::table.filter(prs_metadata::id.eq(metadata_id)))
            .execute(connection);

        match deleted_count {
            Ok(0) => Err(format!("Metadato con ID {:?} no encontrado", metadata_id)),
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error al eliminar el metadato: {:?}", e)),
        }
    }

    async fn create_metadata(&mut self, person_id: Uuid, key: String, value: Option<String>) -> Result<bool, String> {
        let connection = &mut self.db_pool;

        let new_metadata = InfraPersonMeta {
            id: Uuid::new_v4(),
            person_id,
            key,
            value,
            date_created: Some(Utc::now().naive_utc()),
            date_modified: Some(Utc::now().naive_utc()),
        };

        match diesel::insert_into(prs_metadata::table)
            .values(&new_metadata)
            .execute(connection)
        {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Error al insertar metadato: {:?}", e)),
        }
    }

    async fn update_metadata(&mut self, metadata_id: Uuid, key: String, value: Option<String>) -> Result<bool, String> {
        let connection = &mut self.db_pool;

        let updated_rows = diesel::update(prs_metadata::table.filter(prs_metadata::id.eq(metadata_id)))
            .set((
                prs_metadata::key.eq(key),
                prs_metadata::value.eq(value),
                prs_metadata::date_modified.eq(Utc::now().naive_utc()),
            ))
            .execute(connection);

        match updated_rows {
            Ok(0) => Err(format!("Metadato con ID {:?} no encontrado", metadata_id)),
            Ok(_) => {
                let updated_metadata = prs_metadata::table
                    .filter(prs_metadata::id.eq(metadata_id))
                    .first::<InfraPersonMeta>(connection)
                    .map_err(|e| format!("Error al obtener metadato actualizado: {:?}", e))?;

                Ok(true)
            }
            Err(e) => Err(format!("Error al actualizar metadato: {:?}", e)),
        }
    }
}