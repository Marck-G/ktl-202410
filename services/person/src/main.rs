
use domain::repository::{PersonRepository, SearchRepository};
use infra::postgres::{establish_connection, repositories::{person::PgPersonRepository, search::PgSearchRepository}};
use uuid::Uuid;


mod infra;
mod domain;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let connection = establish_connection();
    let mut repo = PgPersonRepository{ db_pool: connection.get().expect("Error al traer la conexion")};
    // Intentar obtener la persona por ID
    // match repo.get_by_ids(vec![Uuid::parse_str("04054104-c631-430f-8dcc-e148643de0fc").unwrap(),
    //     Uuid::parse_str("97879521-9268-4f07-805b-15df9fc29292").unwrap()
    // ]).await {
    //     Ok(persons) => {
    //         println!("Persona encontrada: {}", serde_json::to_string_pretty(&persons).unwrap());
    //     }
    //     Err(e) => {
    //         println!("Error al buscar la persona: {}", e);
    //     }
    // }

    // let result = repo.search_by_name("Marck".to_string()).await.unwrap();
    let result = repo.get_metadata(Uuid::parse_str("04054104-c631-430f-8dcc-e148643de0fc").unwrap()).await.unwrap();
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
