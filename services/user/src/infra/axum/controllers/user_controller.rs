use axum::{
    extract::{Json, Path, Query},
    response::IntoResponse,
};
use uuid::Uuid;
use axum::http::StatusCode;

use crate::app::{dtos::{request::api::{user_create::CreateUserDTO, user_list::PaginationParams, user_update::UpdateUserDTO}, response::api::response::ResponseDto}, user_services::UserService};

pub struct UserController {
    user_service: UserService<'static>,
}

impl UserController {
    pub fn new(user_service: UserService<'static>) -> Self {
        Self { user_service }
    }

    // **Create a new user**
    pub async fn create_user(
        &mut self,
        Json(dto): Json<CreateUserDTO>,
    ) -> impl IntoResponse {
        match self.user_service.create_user(dto) {
            Ok(user) => (
                StatusCode::CREATED,
                Json(ResponseDto::success("User created successfully", user))
            ).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseDto::<()>::error(&format!("Error: {}", e)))
            ).into_response(),
        }
    }

    // **Get user by ID**
    pub async fn get_user(
        &mut self,
        Path(user_id): Path<Uuid>,
    ) -> impl IntoResponse {
        match self.user_service.get_user_by_id(user_id) {
            Ok(user) => (
                StatusCode::OK,
                Json(ResponseDto::success("User found", user))
            ).into_response(),
            Err(e) => (
                StatusCode::NOT_FOUND,
                Json(ResponseDto::<()>::error(&format!("Error: {}", e)))
            ).into_response(),
        }
    }

    // **List all users with pagination**
    pub async fn list_users(
        &mut self,
        Query(pagination): Query<PaginationParams>,
    ) -> impl IntoResponse {
        let page = pagination.page.unwrap_or(1);
        let limit = pagination.limit.unwrap_or(10);

        match self.user_service.list_users(page, limit) {
            Ok(users) => (
                StatusCode::OK,
                Json(ResponseDto::success("Users retrieved successfully", users))
            ).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ResponseDto::<()>::error(&format!("Error: {}", e)))
            ).into_response(),
        }
    }

    // **Update user details**
    pub async fn update_user(
        &mut self,
        Path(user_id): Path<Uuid>,
        Json(dto): Json<UpdateUserDTO>,
    ) -> impl IntoResponse {
        match self.user_service.update_user(user_id, dto) {
            Ok(_) => (
                StatusCode::OK,
                Json(ResponseDto::<()>::success("User updated successfully", ()))
            ).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ResponseDto::<()>::error(&format!("Error: {}", e)))
            ).into_response(),
        }
    }

    // **Delete user**
    pub async fn delete_user(
        &mut self,
        Path(user_id): Path<Uuid>,
    ) -> impl IntoResponse {
        match self.user_service.delete_user(user_id) {
            Ok(_) => (
                StatusCode::OK,
                Json(ResponseDto::<()>::success("User deleted successfully", ()))
            ).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ResponseDto::<()>::error(&format!("Error: {}", e)))
            ).into_response(),
        }
    }
}
