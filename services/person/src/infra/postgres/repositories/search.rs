use crate::{domain::{entities::Person, mappers::Mapper, repository::SearchRepository}, infra::postgres::{mapper::PgMapper, types::PgConnectionPool}};

use crate::infra::postgres::models::models::{Person as InfraPerson, PersonMeta as InfraPersonMeta};
use crate::infra::postgres::schema::{prs_main, prs_metadata};
use diesel::prelude::*; // Para las funciones de consulta y operaciones básicas

pub struct PgSearchRepository {
    pub db_pool: PgConnectionPool
}

impl SearchRepository for PgSearchRepository {
    async fn search_by_name(&mut self, search_term: String) -> Result<Vec<Person>, String> {
        let connection = &mut self.db_pool;

        let pattern = format!("%{}%", search_term); // Formato para LIKE

        let results = prs_main::table
            .filter(
                prs_main::given_name.ilike(&pattern)
                    .or(prs_main::family_name.ilike(&pattern))
                    .or(prs_main::additional_name.ilike(&pattern)),
            )
            .load::<InfraPerson>(connection);

        match results {
            Ok(persons) => Ok(persons.into_iter().map(|person| PgMapper::to_domain(person, vec![])).collect()),
            Err(e) => Err(format!("Error al buscar por nombre: {:?}", e)),
        }
    }

    async fn search_by_metadata_key(&mut self, key: String) -> Result<Vec<Person>, String> {
        let connection = &mut self.db_pool;

        let key_pattern = format!("%{}%", key); // Filtro LIKE para la clave

        // Construimos la consulta base
        let query = prs_main::table
            .inner_join(prs_metadata::table.on(prs_main::id.eq(prs_metadata::person_id))) // Unión con la tabla de metadatos
            .filter(prs_metadata::key.ilike(&key_pattern)); // Filtro por clave con LIKE


        // Ejecutamos la consulta
        let results = query
            .select((prs_main::all_columns, prs_metadata::all_columns)) // Seleccionamos todas las columnas
            .load::<(InfraPerson, InfraPersonMeta)>(connection); // Cargamos los resultados

        match results {
            Ok(persons) => {
                // Agrupamos la persona con sus metadatos asociados
                let mut domain_people = Vec::new();

                for (person, metadata) in persons {
                    let mut domain_person = PgMapper::to_domain(person, vec![metadata]);
                    domain_people.push(domain_person);
                }

                Ok(domain_people)
            },
            Err(e) => Err(format!("Error al buscar por metadatos: {:?}", e)),
        }
    }
}