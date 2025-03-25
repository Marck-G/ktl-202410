use uuid::Uuid;
use async_trait::async_trait;

use super::entities::Person;


#[async_trait]
pub trait PersonRepository {
    async fn get_by_id(&mut self, id_to_search: Uuid) -> Result<Option<Person>, String>;
    async fn get_by_ids(&mut self, ids: Vec<Uuid>) -> Result<Vec<Person>, String>;
    async fn list(&mut self, page: u64, limit: u64) -> Result<Vec<Person>, String>;
    async fn get_by_metadata(&mut self, key: String, value: String) -> Result<Vec<Person>, String>;
    async fn create(&mut self, person: Person) -> Result<Person, String>;
    async fn update(&mut self, person: Person) -> Result<Person, String>;
    async fn delete(&mut self, person_id: Uuid) -> Result<(), String>;
    async fn delete_metadata(&mut self, metadata_id: Uuid) -> Result<(), String>;
    async fn create_metadata(&mut self, person_id: Uuid, key: String, value: Option<String>) -> Result<bool, String>;
    async fn update_metadata(&mut self, metadata_id: Uuid, key: String, value: Option<String>) -> Result<bool, String>;
}