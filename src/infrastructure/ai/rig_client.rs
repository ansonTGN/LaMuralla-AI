use async_trait::async_trait;
use rig::{
    providers::openai,
    completion::Prompt,
    embeddings::EmbeddingsBuilder,
};
use secrecy::ExposeSecret;
use serde_json::from_str;
use crate::domain::{models::{AIConfig, KnowledgeExtraction}, ports::AIService, errors::AppError};

pub struct RigAIService {
    config: AIConfig,
}

impl RigAIService {
    pub fn new(config: AIConfig) -> Self {
        Self { config }
    }

    fn clean_json_response(&self, raw: &str) -> String {
        raw.trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .to_string()
    }
}

#[async_trait]
impl AIService for RigAIService {
    fn update_config(&mut self, config: AIConfig) -> Result<(), AppError> {
        self.config = config;
        Ok(())
    }

    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, AppError> {
        let client = openai::Client::from_url(
            self.config.api_key.expose_secret(), 
            self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1")
        );

        let model = client.embedding_model(&self.config.embedding_model);
        
        let embeddings = EmbeddingsBuilder::new(model)
            .document("temp_id", text, vec![]) 
            .build()
            .await
            .map_err(|e| AppError::AIError(format!("Embedding failed: {}", e)))?;

        let first_doc = embeddings.first()
            .ok_or_else(|| AppError::AIError("No embedding returned from provider".to_string()))?;
            
        let first_embedding_struct = first_doc.embeddings.first()
            .ok_or_else(|| AppError::AIError("Document generated no embeddings".to_string()))?;

        // CORRECCIÓN: Convertir Vec<f64> a Vec<f32>
        let embedding_f32: Vec<f32> = first_embedding_struct.vec.iter().map(|&x| x as f32).collect();
        
        Ok(embedding_f32)
    }

    async fn extract_knowledge(&self, text: &str) -> Result<KnowledgeExtraction, AppError> {
        let client = openai::Client::from_url(
            self.config.api_key.expose_secret(), 
            self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1")
        );

        let agent = client.agent(&self.config.model_name)
            .preamble("You are an expert Ontology Engineer. Extract entities and relationships from the text. \
                       Return strictly JSON format matching this structure: \
                       { \"entities\": [{\"name\": \"...\", \"category\": \"...\"}], \"relations\": [{\"source\": \"...\", \"target\": \"...\", \"relation_type\": \"...\"}] }")
            .build();

        let response = agent.prompt(text).await
            .map_err(|e| AppError::AIError(format!("Extraction failed: {}", e)))?;

        let cleaned_json = self.clean_json_response(&response);

        let extraction: KnowledgeExtraction = from_str(&cleaned_json)
            .map_err(|e| AppError::ParseError(format!("Failed to parse JSON from LLM: {} - Raw: {}", e, cleaned_json)))?;

        Ok(extraction)
    }
}