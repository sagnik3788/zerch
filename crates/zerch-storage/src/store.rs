use anyhow::{Context, Result};
use qdrant_client::qdrant::{
    CreateCollectionBuilder, DeleteCollectionBuilder, Distance, PointStruct,
    SearchParamsBuilder, SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder,
};
use qdrant_client::{Payload, Qdrant};
use serde_json::json;
use uuid::Uuid;

pub type JsonValue = serde_json::Value;

/// The Qdrant collection used to store log embeddings.
pub const COLLECTION_NAME: &str = "zerch_logs";

/// Dimensionality of all-MiniLM-L6-v2 embeddings.
pub const VECTOR_SIZE: u64 = 384;

/// A handle to the Qdrant vector database.
pub struct QdrantStore {
    pub client: Qdrant,
}

impl QdrantStore {
    /// Connect to Qdrant at the given URL (e.g. `"http://localhost:6333"` for gRPC).
    pub async fn new(url: &str) -> Result<Self> {
        let mut config = qdrant_client::config::QdrantConfig::default();
        config.uri = url.to_string();
        config.check_compatibility = false;
        let client = Qdrant::new(config)
            .map_err(|e| anyhow::anyhow!("Failed to connect: {}", e))?;
        Ok(Self { client })
    }

    /// Ensure the collection exists. Creates it if it doesn't.
    pub async fn init_collection(&self) -> Result<()> {
        let exists = self
            .client
            .collection_exists(COLLECTION_NAME)
            .await
            .context("Failed to check collection existence")?;

        if !exists {
            self.client
                .create_collection(
                    CreateCollectionBuilder::new(COLLECTION_NAME)
                        .vectors_config(VectorParamsBuilder::new(VECTOR_SIZE, Distance::Cosine)),
                )
                .await
                .context("Failed to create Qdrant collection")?;
            log::info!("Created Qdrant collection '{}'", COLLECTION_NAME);
        } else {
            log::info!("Qdrant collection '{}' already exists", COLLECTION_NAME);
        }
        Ok(())
    }

    /// Delete the collection and recreate it (effectively wiping all vectors).
    pub async fn clear(&self) -> Result<()> {
        let exists = self
            .client
            .collection_exists(COLLECTION_NAME)
            .await?;

        if exists {
            self.client
                .delete_collection(DeleteCollectionBuilder::new(COLLECTION_NAME))
                .await
                .context("Failed to delete Qdrant collection")?;
            log::info!("Cleared Qdrant collection '{}'", COLLECTION_NAME);
        }

        self.init_collection().await
    }

    /// Count the number of vectors in Qdrant.
    pub async fn count(&self) -> Result<u64> {
        let info = self
            .client
            .collection_info(COLLECTION_NAME)
            .await
            .context("Failed to get collection info")?;
        let points_count = info.result.unwrap_or_default().points_count.unwrap_or(0);
        Ok(points_count)
    }

    /// Upsert a single vector + its original log text into Qdrant.
    pub async fn append_vector(&self, vector: &[f32], text: &str) -> Result<()> {
        let id = Uuid::new_v4().to_string();

        let payload: Payload = json!({ "text": text }).try_into().unwrap();
        let point = PointStruct::new(id, vector.to_vec(), payload);

        self.client
            .upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, vec![point]))
            .await
            .context("Failed to upsert vector into Qdrant")?;

        Ok(())
    }

    /// Batch upsert multiple vectors at once (more efficient for bulk indexing).
    pub async fn append_vectors_batch(&self, items: &[(Vec<f32>, String)]) -> Result<()> {
        if items.is_empty() {
            return Ok(());
        }

        let points: Vec<PointStruct> = items
            .iter()
            .map(|(vector, text)| {
                let payload: Payload = json!({ "text": text }).try_into().unwrap();
                PointStruct::new(Uuid::new_v4().to_string(), vector.clone(), payload)
            })
            .collect();

        self.client
            .upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, points))
            .await
            .context("Failed to batch upsert vectors into Qdrant")?;

        Ok(())
    }

    /// Batch upsert templates with rich metadata (template + raw_log + params).
    pub async fn append_templates_batch(&self, items: &[(Vec<f32>, JsonValue)]) -> Result<()> {
        if items.is_empty() {
            return Ok(());
        }

        let points: Vec<PointStruct> = items
            .iter()
            .map(|(vector, metadata)| {
                let payload: Payload = metadata.clone().try_into().unwrap();
                PointStruct::new(Uuid::new_v4().to_string(), vector.clone(), payload)
            })
            .collect();

        self.client
            .upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, points))
            .await
            .context("Failed to batch upsert templates into Qdrant")?;

        Ok(())
    }

    /// Search for the top-`limit` most similar vectors to `query_vector`.
    /// Returns a list of `(score, text)` pairs sorted by descending similarity.
    pub async fn search(&self, query_vector: Vec<f32>, limit: u64) -> Result<Vec<(f32, String)>> {
        let response = self
            .client
            .search_points(
                SearchPointsBuilder::new(COLLECTION_NAME, query_vector, limit)
                    .with_payload(true)
                    .params(SearchParamsBuilder::default().exact(false)),
            )
            .await
            .context("Failed to search Qdrant")?;

        let results = response
            .result
            .into_iter()
            .filter_map(|scored| {
                let score = scored.score;
                let text = scored
                    .payload
                    .get("text")
                    .and_then(|v| v.as_str())
                    .map_or(String::new(), |s| s.to_string());
                if text.is_empty() {
                    None
                } else {
                    Some((score, text))
                }
            })
            .collect();

        Ok(results)
    }

    /// Search with full metadata (template + raw_log + params).
    pub async fn search_with_metadata(
        &self,
        query_vector: Vec<f32>,
        limit: u64,
    ) -> Result<Vec<(f32, String, JsonValue)>> {
        let response = self
            .client
            .search_points(
                SearchPointsBuilder::new(COLLECTION_NAME, query_vector, limit)
                    .with_payload(true)
                    .params(SearchParamsBuilder::default().exact(false)),
            )
            .await
            .context("Failed to search Qdrant")?;

        let mut results = Vec::new();
        for scored in response.result {
            let score = scored.score;
            let raw_log = scored
                .payload
                .get("raw_log")
                .and_then(|v| v.as_str())
                .map_or(String::new(), |s| s.to_string());
            // Fallback to "text" field if raw_log is empty (for CLI-indexed data)
            let raw_log = if raw_log.is_empty() {
                scored
                    .payload
                    .get("text")
                    .and_then(|v| v.as_str())
                    .map_or(String::new(), |s| s.to_string())
            } else {
                raw_log
            };
            if raw_log.is_empty() {
                continue;
            }
            
            let template = match scored.payload.get("template") {
                Some(v) => v.as_str().map_or("", |s| s).to_string(),
                None => String::new(),
            };
            let cluster_id = match scored.payload.get("cluster_id") {
                Some(v) => v.as_str().map_or("", |s| s).to_string(),
                None => String::new(),
            };
            
            let params = match scored.payload.get("params") {
                Some(p) => serde_json::Value::from(p.clone()),
                None => json!(null),
            };
            
            let metadata = json!({
                "raw_log": raw_log,
                "template": template,
                "params": params,
                "cluster_id": cluster_id,
            });
            results.push((score, raw_log, metadata));
        }

        Ok(results)
    }
}
