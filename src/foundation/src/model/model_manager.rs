use dashmap::DashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::api::inference::InferenceRequest;
use crate::model::circular_buffer::CircularBuffer;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ModelId(pub String);

impl ModelId {
    pub fn from_path(models_path: PathBuf) -> Option<Self> {
        models_path
            .file_name()
            .and_then(|os_model_str| os_model_str.to_str())
            .map(|model| ModelId(model.to_string()))
    }
}

pub struct ModelManager {
    models: DashMap<ModelId, Mutex<CircularBuffer<InferenceRequest>>>,
    models_buffer_capacity: usize,
}

impl ModelManager {
    pub fn new(models_buffer_capacity: usize) -> Self {
        Self {
            models: DashMap::new(),
            models_buffer_capacity,
        }
    }

    pub fn load_models_from_dir<P: AsRef<Path>>(&self, models_dir: P) -> std::io::Result<()> {
        let model_entries = fs::read_dir(models_dir)?;

        for model_entry in model_entries {
            let model_entry = model_entry?;
            if model_entry.file_type()?.is_dir() {
                if let Some(model_id) = ModelId::from_path(model_entry.path()) {
                    self.models.entry(model_id).or_insert_with(|| {
                        Mutex::new(CircularBuffer::new(self.models_buffer_capacity))
                    });
                }
            }
        }

        Ok(())
    }

    pub fn add_request(&self, model_id: ModelId, req: InferenceRequest) {
        let buffer = self
            .models
            .entry(model_id)
            .or_insert_with(|| Mutex::new(CircularBuffer::new(self.models_buffer_capacity)));

        let mut buffer = buffer.lock().unwrap();
        buffer.push(req);
    }
}
