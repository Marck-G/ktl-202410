use uuid::Uuid;
use async_trait::async_trait;

use super::entities::Person;


#[async_trait]
pub trait PersonRepository {
    async fn get_by_id(&mut self, id_to_search: Uuid) -> Result<Option<Person>, String>;
    async fn get_by_ids(&mut self, ids: Vec<Uuid>) -> Result<Vec<Person>, String>;
    // async fn get_all(&self) -> Result<Vec<Person>, String>;
    // async fn save(&self, person: &Person) -> Result<(), String>;
    // async fn delete(&self, person_id: Uuid) -> Result<(), String>;
}