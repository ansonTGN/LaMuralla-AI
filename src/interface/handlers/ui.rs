use axum::{
    response::{Html, IntoResponse},
    extract::State,
};
use std::sync::Arc;
use tera::Context;
use crate::interface::handlers::admin::AppState;

pub async fn render_dashboard(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let _ai_guard = state.ai_service.read().await;
    // Extraemos info básica para mostrar en el frontend
    // (Nota: RigAIService no expone config pública fácilmente sin cambios, 
    // aquí asumimos valores por defecto o modificamos el trait, 
    // por simplicidad pasamos valores fijos o leemos de ENV para el demo)
    
    let mut ctx = Context::new();
    ctx.insert("config", &serde_json::json!({
        "model_name": "gpt-4o", // Podrías leerlo del estado real si expones getters
        "embedding_dim": 1536
    }));

    match state.tera.render("dashboard.html", &ctx) {
        Ok(html) => Html(html),
        Err(err) => Html(format!("<h1>Error rendering template</h1><p>{}</p>", err)),
    }
}