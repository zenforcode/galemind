use dashmap::DashMap;
use std::sync::Mutex;

use crate::model::circular_buffer::CircularBuffer;
use crate::model::model_request::ModelRequest;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ModelId(pub String);

pub struct ModelManager {
    models: DashMap<ModelId, Mutex<CircularBuffer<ModelRequest>>>,
    models_buffer_capacity: usize,
}

impl ModelManager {
    pub fn new(models_buffer_capacity: usize) -> Self {
        Self {
            models: DashMap::new(),
            models_buffer_capacity,
        }
    }

    pub fn add_request(&self, model_id: ModelId, req: ModelRequest) {
        let buffer = self
            .models
            .entry(model_id)
            .or_insert_with(|| Mutex::new(CircularBuffer::new(self.models_buffer_capacity)));

        let mut buffer = buffer.lock().unwrap();
        buffer.push(req);
    }
}
