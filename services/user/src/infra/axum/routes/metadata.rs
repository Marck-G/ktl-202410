
// **Routes for the MetadataController**
pub fn metadata_routes(metadata_controller: MetadataController) -> Router {
    Router::new()
        .route("/users/:id/metadata", post(metadata_controller.add_metadata)) // Add metadata
        .route("/users/:id/metadata", get(metadata_controller.get_metadata)) // Get metadata
        .route("/metadata/:id", put(metadata_controller.update_metadata)) // Update metadata
        .route("/metadata/:id", delete(metadata_controller.delete_metadata)) // Delete metadata
}
