use std::collections::HashMap;
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MemStore {
    store: HashMap<String, usize>,
}

impl MemStore {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MemStore {
            store: HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn insert_value(&mut self, key: &str, value: usize) {
        self.store.insert(key.to_string(), value);
    }
    #[allow(unused_variables, dead_code)]
    pub fn initialize(&mut self) {
        let syntax_values = vec![
            "# ".to_string(),
            "## ".to_string(),
            "### ".to_string(),
            "#### ".to_string(),
            "##### ".to_string(),
            "###### ".to_string(),
        ];
        for (pos, val) in syntax_values.iter().enumerate() {
            self.store.insert(val.to_string(), pos);
        }
    }
    #[allow(unused)]
    pub fn find_by_key(&self, key: &str) -> bool {
        self.store.contains_key(key)
    }
}
