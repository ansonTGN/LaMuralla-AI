use axum::{Json, extract::State};
use std::sync::Arc;
use crate::domain::{models::GraphDataResponse, errors::AppError};
use super::admin::AppState;

#[utoipa::path(
    get,
    path = "/api/graph",
    responses(
        (status = 200, description = "Retrieve full graph for visualization", body = GraphDataResponse),
        (status = 500, description = "Database error")
    ),
    tag = "visualization" // CORRECCIÓN: 'tag' en singular
)]
pub async fn get_graph(
    State(state): State<Arc<AppState>>,
) -> Result<Json<GraphDataResponse>, AppError> {
    
    // Llamada al repositorio
    let graph_data = state.repo.get_full_graph().await?;
    
    Ok(Json(graph_data))
}