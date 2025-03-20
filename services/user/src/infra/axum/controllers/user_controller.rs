use std::sync::{Arc};

use axum::extract::State;
use axum::http::StatusCode;
use axum::{
    extract::{Json, Path, Query},
    response::IntoResponse,
};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::app::user_services;
use crate::app::{
    dtos::{
        request::api::{
            user_create::CreateUserDTO, user_list::PaginationParams, user_update::UpdateUserDTO,
        },
        response::api::response::ResponseDto,
    },
    user_services::UserService,
};

pub struct UserController {
    user_service: Arc<Mutex<UserService>>,
}

impl UserController {
    pub fn new(user_service: Arc<Mutex<UserService>>) -> Self {
        Self { user_service }
    }
}

// **Create a new user**
pub async fn create_user(State(controller): State<Arc<UserController>>, Json(dto): Json<CreateUserDTO>) -> impl IntoResponse {
    let mut user_service = controller.user_service.lock().await;
    match user_service.create_user(dto).await {
        Ok(user) => (
            StatusCode::CREATED,
            Json(ResponseDto::success("User created successfully", user)),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseDto::<()>::error(&format!("Error: {}", e))),
        )
            .into_response(),
    }
}

// **Get user by ID**
pub async fn get_user(State(controller): State<Arc<UserController>>, Path(user_id): Path<Uuid>) -> impl IntoResponse {
    let mut user_service = controller.user_service.lock().await;
    match user_service.get_user_by_id(user_id).await {
        Ok(user) => (
            StatusCode::OK,
            Json(ResponseDto::success("User found", user)),
        )
            .into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ResponseDto::<()>::error(&format!("Error: {}", e))),
        )
            .into_response(),
    }
}

// **List all users with pagination**
pub async fn list_users(
    State(controller): State<Arc<UserController>>,
    Query(pagination): Query<PaginationParams>,
) -> impl IntoResponse {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let mut user_service = controller.user_service.lock().await;

    match user_service.list_users(page, limit).await {
        Ok(users) => (
            StatusCode::OK,
            Json(ResponseDto::success("Users retrieved successfully", users)),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ResponseDto::<()>::error(&format!("Error: {}", e))),
        )
            .into_response(),
    }
}

// **Update user details**
pub async fn update_user(
    State(controller): State<Arc<UserController>>,
    Path(user_id): Path<Uuid>,
    Json(dto): Json<UpdateUserDTO>,
) -> impl IntoResponse {
    let mut user_service = controller.user_service.lock().await;
    match user_service.update_user(user_id, dto).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ResponseDto::<()>::success("User updated successfully", ())),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ResponseDto::<()>::error(&format!("Error: {}", e))),
        )
            .into_response(),
    }
}

// **Delete user**
pub async fn delete_user(State(controller): State<Arc<UserController>>, Path(user_id): Path<Uuid>) -> impl IntoResponse {
    let mut user_service = controller.user_service.lock().await;
    match user_service.delete_user(user_id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ResponseDto::<()>::success("User deleted successfully", ())),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ResponseDto::<()>::error(&format!("Error: {}", e))),
        )
            .into_response(),
    }
}
