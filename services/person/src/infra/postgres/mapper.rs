
use crate::domain::mappers::Mapper;
use crate::infra::postgres::models::models::{Person as InfraPerson, PersonMeta as InfraPersonMeta};
use crate::domain::entities::{Metadata as DomainMetadata, Person as DomainPerson};

pub struct PgMapper {}
pub struct PgMetadataMapper {}

impl PgMapper {
    pub fn new () -> Self {
        Self {}
    }
}

impl PgMetadataMapper {
    pub fn new () -> Self {
        Self {}
    }
}

impl Mapper<InfraPerson, InfraPersonMeta> for PgMapper {
    fn to_domain(infra: InfraPerson, metadata: Vec<InfraPersonMeta>) -> DomainPerson {
        let metadata = metadata
            .into_iter()
            .map(|m| DomainMetadata {
                id: m.id,
                key: m.key,
                value: m.value,
                date_created: m.date_created,
                date_modified: m.date_modified,
            })
            .collect();

        DomainPerson {
            id: infra.id,
            customer_id: infra.customer_id,
            given_name: infra.given_name,
            family_name: infra.family_name,
            additional_name: infra.additional_name,
            birth_date: infra.birth_date,
            gender: infra.gender, 
            date_created: infra.date_created,
            date_modified: infra.date_modified,
            metadata,
        }
    }

    fn to_infrastructure(domain: DomainPerson) -> (InfraPerson, Vec<InfraPersonMeta>) {
        let infra_person = InfraPerson {
            id: domain.id,
            customer_id: domain.customer_id,
            given_name: domain.given_name,
            family_name: domain.family_name,
            additional_name: domain.additional_name,
            birth_date: domain.birth_date,
            gender: domain.gender,
            date_created: domain.date_created,
            date_modified: domain.date_modified,
        };

        let infra_metadata: Vec<InfraPersonMeta> = domain.metadata.into_iter().map(|m| InfraPersonMeta {
            id: m.id,
            person_id: domain.id, // Se mantiene la relación
            key: m.key,
            value: m.value,
            date_created: m.date_created,
            date_modified: m.date_modified,
        }).collect();

        (infra_person, infra_metadata)
    }
}

impl PgMetadataMapper {
    pub fn to_domain(infra: InfraPersonMeta) -> DomainMetadata {
        DomainMetadata {
            id: infra.id,
            key: infra.key,
            value: infra.value,
            date_created: infra.date_created,
            date_modified: infra.date_modified,
        }
    }

    // Mapper para convertir de Metadata (dominio) a InfraPersonMeta (infraestructura)
    pub fn to_infrastructure(domain: DomainMetadata) -> InfraPersonMeta {
        InfraPersonMeta {
            id: domain.id,
            person_id: domain.id, // Suponiendo que el ID de la persona se utiliza como person_id
            key: domain.key,
            value: domain.value,
            date_created: domain.date_created,
            date_modified: domain.date_modified,
        }
    }
}