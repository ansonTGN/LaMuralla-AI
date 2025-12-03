mod domain;
mod application;
mod infrastructure;
mod interface;

use axum::{routing::{post, get}, Router};
use std::sync::Arc;
use tokio::sync::RwLock;
use neo4rs::Graph;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer;
use secrecy::SecretString;
use tera::Tera;

use crate::domain::models::*;
use crate::infrastructure::ai::rig_client::RigAIService;
use crate::infrastructure::persistence::neo4j_repo::Neo4jRepo;
// Importamos 'chat'
use crate::interface::handlers::{admin::{self, AppState}, ingest, graph, ui, chat};
use crate::application::dtos::*;

// Documentación OpenAPI (Swagger)
#[derive(OpenApi)]
#[openapi(
    paths(
        interface::handlers::admin::update_config,
        interface::handlers::ingest::ingest_document,
        interface::handlers::graph::get_graph,
        interface::handlers::chat::chat_handler // <-- NUEVO PATH
    ),
    components(
        schemas(
            AIConfig, AIProvider, 
            IngestionRequest, IngestionResponse, 
            AdminConfigPayload,
            VisNode, VisEdge, GraphDataResponse,
            ChatRequest, ChatResponse // <-- NUEVOS SCHEMAS
        )
    ),
    tags(
        (name = "admin", description = "Administration endpoints"),
        (name = "ingestion", description = "Data ingestion endpoints"),
        (name = "visualization", description = "Graph visual exploration"),
        (name = "chat", description = "Semantic GraphRAG Chat") // <-- NUEVO TAG
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("🚀 Starting La Muralla Backend...");

    let initial_config = AIConfig {
        provider: AIProvider::OpenAI,
        model_name: "gpt-4o".to_string(),
        embedding_model: "text-embedding-3-small".to_string(),
        api_key: SecretString::new(std::env::var("OPENAI_API_KEY").unwrap_or_default()),
        embedding_dim: 1536,
        base_url: None,
    };

    let uri = std::env::var("NEO4J_URI").expect("NEO4J_URI required");
    let user = std::env::var("NEO4J_USER").expect("NEO4J_USER required");
    let pass = std::env::var("NEO4J_PASS").expect("NEO4J_PASS required");
    
    tracing::info!("🔌 Connecting to Neo4j at {}", uri);
    let graph = Arc::new(Graph::new(&uri, &user, &pass).await?);
    
    let repo = Arc::new(Neo4jRepo::new(graph.clone()));
    let ai_service = Arc::new(RwLock::new(RigAIService::new(initial_config)));

    tracing::info!("🎨 Loading HTML templates...");
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("❌ Error parsing templates: {}", e);
            ::std::process::exit(1);
        }
    };

    let app_state = Arc::new(AppState {
        repo,
        ai_service,
        tera, 
    });

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        
        // --- API REST ---
        .route("/api/admin/config", post(admin::update_config))
        .route("/api/ingest", post(ingest::ingest_document))
        .route("/api/graph", get(graph::get_graph))
        .route("/api/chat", post(chat::chat_handler)) // <-- NUEVA RUTA
        
        // --- FRONTEND ---
        .route("/", get(ui::render_dashboard))

        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("✅ Server running on http://{}", addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}