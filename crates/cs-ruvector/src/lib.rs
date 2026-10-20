use cs_embedding::embed;
use cs_incidents::Incident;
use ruvector_core::types::{DbOptions, HnswConfig};
use ruvector_core::{DistanceMetric, SearchQuery, VectorDB, VectorEntry};
use std::collections::HashMap;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum RecallError {
    #[error("ruvector: {0}")]
    Database(String),
}
pub struct RuVectorRecall {
    db: VectorDB,
}
impl RuVectorRecall {
    pub fn open(path: &str) -> Result<Self, RecallError> {
        VectorDB::new(DbOptions {
            dimensions: 4,
            distance_metric: DistanceMetric::Cosine,
            storage_path: path.into(),
            hnsw_config: Some(HnswConfig::default()),
            quantization: None,
        })
        .map(|db| Self { db })
        .map_err(|e| RecallError::Database(e.to_string()))
    }
    pub fn remember(&self, i: &Incident) -> Result<String, RecallError> {
        let mut m = HashMap::new();
        m.insert("kind".into(), serde_json::json!(i.action_kind));
        self.db
            .insert(VectorEntry {
                id: Some(i.id.clone()),
                vector: embed(i).to_vec(),
                metadata: Some(m),
            })
            .map_err(|e| RecallError::Database(e.to_string()))
    }
    pub fn recall(&self, i: &Incident, k: usize) -> Result<Vec<String>, RecallError> {
        self.db
            .search(SearchQuery {
                vector: embed(i).to_vec(),
                k,
                filter: None,
                ef_search: Some(32),
            })
            .map(|r| r.into_iter().map(|x| x.id).collect())
            .map_err(|e| RecallError::Database(e.to_string()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_insert_search() {
        let d = tempfile::tempdir().unwrap();
        let file = d.path().join("vectors.db");
        let db = RuVectorRecall::open(file.to_str().unwrap()).unwrap();
        let i = Incident::new("i1", "deploy", 0.9, 4, "credential fanout").unwrap();
        db.remember(&i).unwrap();
        assert_eq!(db.recall(&i, 1).unwrap(), vec!["i1"]);
    }
}
