use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Metrics {
    pub data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&mut self, key: impl Into<String>) {
        *self.data.lock().unwrap().entry(key.into()).or_insert(0) += 1;
    }

    pub fn dec(&mut self, key: impl Into<String>) {
        *self.data.lock().unwrap().entry(key.into()).or_insert(0) -= 1;
    }

    pub fn snapshot(&self) -> HashMap<String, i64> {
        self.data.lock().unwrap().clone()
    }
}
