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
use crate::infrastructure::parsing::parse_text_from_bytes; // E0432 CORREGIDO
use super::admin::AppState;

#[utoipa::path(
    post, // <-- Faltaba esto
    path = "/api/ingest",
    request_body(
        content_type = "multipart/form-data", 
        description = "Sube un archivo (PDF/DOCX/TXT) en el campo 'file' o texto plano en 'content'",
    ),
    responses(
        (status = 200, description = "Stream de texto con el progreso del proceso"),
        (status = 500, description = "Error interno del servidor")
    ),
    tag = "ingestion" // Añadimos el tag para utoipa
)]
pub async fn ingest_document(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {

    // Creamos un canal para streaming de logs
    let (tx, rx) = mpsc::channel::<String>(10);
    let tx_inner = tx.clone();

    // Lanzamos el proceso en background
    tokio::spawn(async move {
        // 1. Leer archivo del Multipart
        let mut content = String::new();
        // Variable renombrada a 'file_label' y usada para logging, eliminando la advertencia.
        let mut file_label = String::from("Text Input"); 

        while let Ok(Some(field)) = multipart.next_field().await {
            if let Some(name) = field.name() {
                if name == "file" {
                    // 1. Obtener nombre y notificar
                    file_label = field.file_name().unwrap_or("file").to_string();
                    let _ = tx_inner.send(format!("📂 Leyendo archivo: {}...", file_label)).await;
                    
                    // 2. Obtener bytes del archivo
                    let bytes_result = field.bytes().await;

                    match bytes_result {
                        Ok(bytes) => {
                             let _ = tx_inner.send("📄 Parseando contenido...".to_string()).await;
                             match parse_text_from_bytes(&file_label, &bytes) {
                                Ok(text) => content = text,
                                Err(e) => {
                                    let _ = tx_inner.send(format!("❌ Error parseando: {}", e)).await;
                                    return;
                                }
                             }
                        },
                        Err(e) => {
                            // Si falla la subida (ej. límite de tamaño excedido, parseo multipart inválido)
                            let _ = tx_inner.send(format!("❌ Error subida: Error parsing `multipart/form-data` request: {}", e)).await;
                            return;
                        }
                    }
                } else if name == "content" {
                     if let Ok(text) = field.text().await {
                        if !text.is_empty() {
                            content = text;
                            file_label = "Texto Plano".to_string(); // Actualizamos la etiqueta para el log
                            let _ = tx_inner.send("📝 Recibido texto directo...".to_string()).await;
                        }
                     }
                }
            }
        }
        
        if content.trim().len() < 5 {
            let _ = tx_inner.send("❌ Error: Contenido vacío o muy corto.".to_string()).await;
            return;
        }

        // 2. Iniciar Servicio
        let service = IngestionService::new(state.repo.clone(), state.ai_service.clone());

        match service.ingest_with_progress(content, tx_inner.clone()).await {
            Ok(_) => {
                let _ = tx_inner.send("DONE".to_string()).await;
            },
            Err(e) => {
                let _ = tx_inner.send(format!("❌ Error Crítico: {}", e)).await;
            }
        }
    });

    // Convertimos el Receiver en un Stream compatible con Axum Body
    let stream = ReceiverStream::new(rx).map(|msg| {
        Ok::<_, std::io::Error>(Bytes::from(format!("{}\n", msg))) 
    });

    Body::from_stream(stream)
}