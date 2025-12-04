use axum::{
    extract::{State, Multipart},
    response::IntoResponse,
    body::{Body, Bytes},
};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use crate::application::ingestion::IngestionService;
// IMPORTANTE: Usamos el nuevo módulo
use crate::infrastructure::transmutation::DocumentTransmuter;
use super::admin::AppState;

#[utoipa::path(
    post,
    path = "/api/ingest",
    request_body(
        content_type = "multipart/form-data", 
        description = "Sube archivos (PDF, DOCX, XLSX, CSV, HTML, TXT)",
    ),
    responses(
        (status = 200, description = "Stream de progreso"),
        (status = 500, description = "Error interno")
    )
)]
pub async fn ingest_document(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {

    let (tx, rx) = mpsc::channel::<String>(10);
    let tx_inner = tx.clone();

    tokio::spawn(async move {
        let mut content = String::new();
        let mut filename = String::from("unknown");

        while let Ok(Some(field)) = multipart.next_field().await {
            let name = field.name().unwrap_or("").to_string();

            if name == "file" {
                filename = field.file_name().unwrap_or("archivo_desconocido").to_string();
                let _ = tx_inner.send(format!("📂 Recibido archivo: {}", filename)).await;
                
                // Leemos los bytes a memoria (cuidado con archivos >100MB, idealmente streaming)
                match field.bytes().await {
                    Ok(bytes) => {
                         let _ = tx_inner.send("✨ Transmutando formato a texto plano...".to_string()).await;
                         
                         // --- AQUÍ ESTÁ EL CAMBIO CLAVE ---
                         match DocumentTransmuter::transmute(&filename, &bytes) {
                            Ok(text) => {
                                content = text;
                                let _ = tx_inner.send(format!("✅ Transmutación exitosa ({} caracteres).", content.len())).await;
                            },
                            Err(e) => {
                                let _ = tx_inner.send(format!("❌ Error de Transmutación: {}", e)).await;
                                return; // Detener si falla la conversión
                            }
                         }
                         // --------------------------------
                    },
                    Err(e) => {
                        let _ = tx_inner.send(format!("❌ Error subida: {}", e)).await;
                        return;
                    }
                }
            } else if name == "content" {
                 if let Ok(text) = field.text().await {
                    if !text.is_empty() {
                        content = text;
                        let _ = tx_inner.send("📝 Usando texto directo...".to_string()).await;
                    }
                 }
            }
        }

        if content.trim().len() < 5 {
            let _ = tx_inner.send("❌ Error: Contenido vacío o insuficiente.".to_string()).await;
            return;
        }

        // Iniciar Servicio de Ingesta (Chunking -> Embedding -> Graph)
        let service = IngestionService::new(state.repo.clone(), state.ai_service.clone());

        match service.ingest_with_progress(content, tx_inner.clone()).await {
            Ok(_) => { let _ = tx_inner.send("DONE".to_string()).await; },
            Err(e) => { let _ = tx_inner.send(format!("❌ Error Crítico en Ingesta: {}", e)).await; }
        }
    });

    let stream = ReceiverStream::new(rx).map(|msg| {
        Ok::<_, std::io::Error>(Bytes::from(format!("{}\n", msg))) 
    });

    Body::from_stream(stream)
}