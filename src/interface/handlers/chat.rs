use axum::{Json, extract::State};
use std::sync::Arc;
use rig::{completion::Prompt, providers::openai};
use crate::domain::{models::{ChatRequest, ChatResponse}, errors::AppError};
use super::admin::AppState;

#[utoipa::path(
    post,
    path = "/api/chat",
    request_body = ChatRequest,
    responses(
        (status = 200, description = "Respuesta RAG Semántico", body = ChatResponse),
        (status = 500, description = "Error interno")
    ),
    tag = "chat"
)]
pub async fn chat_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    
    // 1. Obtener Embedding de la pregunta del usuario
    // Usamos el servicio de IA configurado (que tiene el lock de lectura)
    let ai_guard = state.ai_service.read().await;
    let embedding = ai_guard.generate_embedding(&payload.message).await?;
    
    // 2. Recuperación Híbrida (Vector + Grafo)
    // Buscamos los 3 chunks más relevantes y sus entidades asociadas en Neo4j
    let hybrid_contexts = state.repo.find_hybrid_context(embedding, 3).await?;
    
    // 3. Construcción del Contexto Enriquecido
    let mut context_text = String::new();
    let mut references_meta = Vec::new();

    for ctx in &hybrid_contexts {
        let entity_list = ctx.connected_entities.join(", ");
        
        // Construimos el bloque de texto para el LLM
        context_text.push_str(&format!(
            "FRAGMENTO [ID: {}]\nCONTENIDO: {}\nCONCEPTOS DEL GRAFO RELACIONADOS: [{}]\n---\n", 
            ctx.chunk_id, ctx.content, entity_list
        ));
        
        // Guardamos metadata para mostrar en el frontend (acortamos el ID visualmente)
        let short_id = if ctx.chunk_id.len() > 8 { &ctx.chunk_id[..8] } else { &ctx.chunk_id };
        references_meta.push(format!("Fragmento {} (Conceptos: {})", short_id, entity_list));
    }

    // 4. Prompt Engineering Profesional
    // CORRECCIÓN: Usamos r#""# para respetar saltos de línea sin escapar, eliminando el warning.
    let system_prompt = format!(r#"Eres 'La Muralla', un asistente de inteligencia artificial avanzado para análisis de documentos.
Tu objetivo es responder a la pregunta del usuario basándote EXCLUSIVAMENTE en la información proporcionada en el siguiente CONTEXTO.

REGLAS DE FORMATO ESTRICTAS:
1. Si la respuesta contiene conceptos técnicos o entidades que aparecen explícitamente en el contexto (sección 'CONCEPTOS DEL GRAFO'), enciérralos entre dobles corchetes, por ejemplo: [[NombreConcepto]]. Esto creará enlaces interactivos para el usuario.
2. Al final de cada afirmación importante, cita la fuente usando el formato: (Ref: ID_FRAGMENTO).
3. Si la información no está en el contexto, indícalo claramente.
4. Responde de manera profesional y concisa en español.

CONTEXTO:
{}"#, 
        context_text
    );

    // 5. Generar Respuesta con LLM
    // Instanciamos un cliente OpenAI simple usando la key del entorno.
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
    
    let client = openai::Client::from_url(&api_key, "https://api.openai.com/v1");
    let agent = client.agent("gpt-4o") // Modelo potente para seguir instrucciones complejas
        .preamble(&system_prompt)
        .build();

    let answer = agent.prompt(&payload.message).await
        .map_err(|e| AppError::AIError(format!("Error generando respuesta: {}", e)))?;

    Ok(Json(ChatResponse {
        response: answer,
        context_used: references_meta,
    }))
}