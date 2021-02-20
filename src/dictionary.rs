use crate::phrase::Phrase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dictionary {
    inner: HashMap<String, Phrase>,
}

impl Dictionary {
    pub fn as_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn new() -> Dictionary {
        Dictionary {
            inner: HashMap::new(),
        }
    }

    pub fn generate<S: Into<String>>(source: S) -> Dictionary {
        let data = read_to_string(source.into()).unwrap();
        serde_json::from_str(&data).unwrap()
    }

    pub fn get_val(&self, key: &str) -> Option<&Phrase> {
        self.inner.get(key)
    }

    pub fn get_val_mut(&mut self, key: &str) -> Option<&mut Phrase> {
        self.inner.get_mut(key)
    }

    pub fn get_topic(&self, topic: String) -> Vec<&Phrase> {
        self.inner.values().filter(|p| p.topic == topic).collect()
    }

    pub fn insert(&mut self, key: String, value: Phrase) -> Option<Phrase> {
        self.inner.insert(key, value)
    }

    pub fn keys(&self) -> Vec<&String> {
        self.inner.keys().collect()
    }

    pub fn remove(&mut self, key: &str) -> Option<Phrase> {
        self.inner.remove(key)
    }
}
