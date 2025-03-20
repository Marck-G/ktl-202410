use std::sync::Arc;

use app::user_services::UserService;
use axum::Router;
use dotenvy::dotenv;
use infra::{axum::routes::user::user_routes, postgres::{crypto::PgPasswordTools, establish_connection, repositories::PgUserRepository}};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};


mod domain;
mod infra;
mod app;

// fn main() {
//     dotenv().ok();
//     // let connection = &mut establish_connection();
//     // let metadata: Vec<Metadata> = vec![
//     //     Metadata::new(Uuid::new_v4(), String::from("genero"), String::from("mujer")),
//     //     Metadata::new(Uuid::new_v4(), String::from("edad"), String::from("32"))
//     // ];
//     // let crypto_helper = CryptoHelper::new("./private.pem").unwrap();
//     // let encrypt_password = crypto_helper.encrypt_password(String::from("Estafeta,13")).unwrap();
//     // // CryptoHelper::generate_keys("./private.pem", "./pub.cert").expect("Error while generating files");
//     // let mut user: UserEntity = UserEntity::new(Uuid::new_v4(), 
//     //     "lilba@outlook.es".to_string(), 
//     //     encrypt_password);
//     // user.metadata = metadata;
//     // let mut repo = PgUserRepository::new(connection);
//     // let response =  repo.create(user);
//     // match response {
//     //     Ok(r) => println!("Created: {}", r),
//     //     Err(q) => println!("{}", q)
//     // }
    
// }

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut connection: Arc<Mutex<diesel::PgConnection>> = establish_connection();
    let mut tools: &mut PgPasswordTools = &mut PgPasswordTools::new("./private.pem".to_string());
    let repo = Arc::new(Mutex::new(PgUserRepository::new( connection, tools)));
    // Initialize services
    let user_service = Arc::new(Mutex::new(UserService::new(repo).await));

    // Create the router with all routes
    let app = Router::new()
        .merge(user_routes(user_service.clone())) // Mount user routes
        .layer(CorsLayer::new().allow_origin(Any)); // Allow CORS for all requests

    // Define server address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running at http://0.0.0.0:3000");

    // Start the server
    axum::serve(listener, app).await.unwrap();
}