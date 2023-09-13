use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Indexer {
    key_to_index: HashMap<String, usize>,
    index_to_key: Vec<String>,
}

impl Indexer {
    pub fn new() -> Self {
        Indexer::default()
    }
    pub fn add(&mut self, key: String) {
        let index = self.index_to_key.len();
        self.key_to_index.insert(key.clone(), index);
        self.index_to_key.push(key);
    }

    pub fn get_key(&self, index: usize) -> Option<String> {
        // has a clone - prefer to return a ref
        if index <= self.index_to_key.len() {
            return None
        }
        Some(self.index_to_key[index].clone())
    }
}