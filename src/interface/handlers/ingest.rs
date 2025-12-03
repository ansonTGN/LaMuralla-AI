use axum::{Json, extract::State, http::StatusCode};
use std::sync::Arc;
use crate::domain::models::IngestionRequest;
use crate::application::{ingestion::IngestionService, dtos::IngestionResponse};
use crate::domain::errors::AppError;
use super::admin::AppState;

#[utoipa::path(
    post,
    path = "/api/ingest",
    request_body = IngestionRequest,
    responses(
        (status = 201, description = "Ingested successfully", body = IngestionResponse),
        (status = 400, description = "Validation Error"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn ingest_document(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<IngestionRequest>,
) -> Result<(StatusCode, Json<IngestionResponse>), AppError> {
    
    // 1. Instanciar el servicio (Inyección de dependencias manual)
    // Pasamos los Arcs del estado global al servicio efímero
    let service = IngestionService::new(
        state.repo.clone(),
        state.ai_service.clone()
    );

    // 2. Ejecutar caso de uso
    let id = service.ingest(payload).await?;
    
    Ok((StatusCode::CREATED, Json(IngestionResponse { 
        id: id.to_string(), 
        status: "Ingested successfully".into() 
    })))
}