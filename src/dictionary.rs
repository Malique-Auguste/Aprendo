use crate::phrase::Phrase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::SystemTime;
use rand::prelude::*;
use crate::helper::shuffle;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dictionary {
    //hashmap is used instead of vec because it is constant-time access
    inner: HashMap<String, Phrase>,
}

impl Dictionary {
    pub fn as_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn new(path: Option<&str>) -> Result<Dictionary, String> {
        match path {
            Some(p) => {
                let data = read_to_string(p).unwrap();
                Ok(serde_json::from_str(&data).unwrap())
            },
            None => Ok(Dictionary {
                inner: HashMap::new(),
            })
        }
    }

    pub fn get_test_phrases(&self, mut size: usize) -> Vec<&Phrase> {
        let all_phrases = self.inner.values();
        let all_phrases_len = all_phrases.len();
        if size >= all_phrases.len() {
            size = 0;
        }
        else {
            size = all_phrases_len - size;
        }

        let mut all_phrases: Vec<(&Phrase, usize)> = all_phrases.zip(vec![0; all_phrases_len]).collect();

        for group in all_phrases.iter_mut() {
            //28800 is the number of seconds in 8 hours
            let duration = group.0.last_tested.elapsed().unwrap().as_secs() as usize / 28800;
            group.1 = group.0.difficulty.as_num()  * duration;

            if group.1 == 0 {
                group.1 = 1;
            } 
        }

        let mut rng = rand::thread_rng();
        shuffle(&mut all_phrases);

        for i in 0..size {
            let min = rng.gen_range(0..all_phrases.iter().fold(0, |acc, p| acc + p.1));
            let mut acc = 0;

            let pos = all_phrases.iter().position(|x| {
                acc += x.1;
                acc >= min
            }).unwrap();

            all_phrases.remove(pos);
        }

        all_phrases.iter().map(|x| x.0).collect()
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

    pub fn values(&self) -> Vec<&Phrase> {
        self.inner.values().collect()
    }

    pub fn remove(&mut self, key: &str) -> Option<Phrase> {
        self.inner.remove(key)
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }
}
