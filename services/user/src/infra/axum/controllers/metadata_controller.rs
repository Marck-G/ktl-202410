use axum::{
    extract::{Path, Json},
    response::IntoResponse,
};
use uuid::Uuid;
use axum::http::StatusCode;

use crate::{app::{dtos::{request::api::{add_metadata::AddMetadataDto, remove_metadata::RemoveMetadataDto, update_metadata::UpdateMetadataDto}, response::api::response::ResponseDto}, metadata_service::MetadataService}, infra::postgres::schema::usr_metadata::value};

pub struct MetadataController {
    metadata_service: MetadataService<'static>,
}

impl MetadataController {
    pub fn new(metadata_service: MetadataService<'static>) -> Self {
        Self { metadata_service }
    }

    // **Add metadata to a user**
    pub async fn add_metadata(
        &mut self,
        Json(dto): Json<AddMetadataDto>,
    ) -> impl IntoResponse {
        match self.metadata_service.add_metadata(dto) {
            Ok(metadata) => (
                StatusCode::CREATED,
                Json(ResponseDto::success("Metadata added successfully", metadata))
            ).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ResponseDto::<()>::error(&format!("Error: {}", e)))
            ).into_response(),
        }
    }

    // **Get metadata by user ID**
    pub async fn get_metadata(
        &mut self,
        Path(user_id): Path<Uuid>,
    ) -> impl IntoResponse {
        match self.metadata_service.get_metadata(user_id) {
            Ok(metadata_list) => (
                StatusCode::OK,
                Json(ResponseDto::success("Metadata retrieved successfully", metadata_list))
            ).into_response(),
            Err(e) => (
                StatusCode::NOT_FOUND,
                Json(ResponseDto::<()>::error(&format!("Error: {}", e)))
            ).into_response(),
        }
    }

    // **Update metadata**
    pub async fn update_metadata(
        &mut self,
        Json(dto): Json<UpdateMetadataDto>,
    ) -> impl IntoResponse {
        match self.metadata_service.update_metadata(dto) {
            Ok(_) => (
                StatusCode::OK,
                Json(ResponseDto::<()>::success("Metadata updated successfully", ()))
            ).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ResponseDto::<()>::error(&format!("Error: {}", e)))
            ).into_response(),
        }
    }

    // **Delete metadata**
    pub async fn delete_metadata(
        &mut self,
        Json(dto): Json<RemoveMetadataDto>
    ) -> impl IntoResponse {
        match self.metadata_service.remove_metadata(dto) {
            Ok(_) => (
                StatusCode::OK,
                Json(ResponseDto::<()>::success("Metadata deleted successfully", ()))
            ).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ResponseDto::<()>::error(&format!("Error: {}", e)))
            ).into_response(),
        }
    }
}
