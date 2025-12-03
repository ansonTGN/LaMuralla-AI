use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock; // Necesario para manejar el bloqueo
use crate::domain::{
    ports::{KGRepository, AIService},
    models::IngestionRequest,
    errors::AppError
};

pub struct IngestionService {
    repo: Arc<dyn KGRepository>,
    ai: Arc<RwLock<dyn AIService>>, // Actualizado para aceptar RwLock
}

impl IngestionService {
    pub fn new(repo: Arc<dyn KGRepository>, ai: Arc<RwLock<dyn AIService>>) -> Self {
        Self { repo, ai }
    }

    pub async fn ingest(&self, req: IngestionRequest) -> Result<Uuid, AppError> {
        let chunk_id = Uuid::new_v4();

        // Adquirimos el bloqueo de lectura una sola vez o por llamada
        let ai_guard = self.ai.read().await;

        // 1. Vectorizar Texto
        let embedding = ai_guard.generate_embedding(&req.content).await?;

        // 2. Guardar Chunk Vectorial (Repo no necesita el bloqueo de AI)
        // Soltamos el guard si queremos concurrencia máxima, pero aquí está bien mantenerlo
        self.repo.save_chunk(chunk_id, &req.content, embedding).await?;

        // 3. Extracción Simbólica (LLM)
        let extraction = match ai_guard.extract_knowledge(&req.content).await {
            Ok(ext) => ext,
            Err(_) => {
                // Reintento simple
                ai_guard.extract_knowledge(&req.content).await?
            }
        };

        // 4. Persistir Grafo
        self.repo.save_graph(chunk_id, extraction).await?;

        Ok(chunk_id)
    }
}