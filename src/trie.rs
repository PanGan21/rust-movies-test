// Define a Trie data structure for genres
pub struct GenreTrie {
    children: std::collections::HashMap<String, GenreTrie>,
}

impl GenreTrie {
    pub fn new() -> Self {
        GenreTrie {
            children: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, genre: &str) {
        let mut node = self;
        for c in genre.chars() {
            node = node
                .children
                .entry(c.to_string())
                .or_insert(GenreTrie::new());
        }
    }

    pub fn contains(&self, genre: &str) -> bool {
        let mut node = self;
        for c in genre.chars() {
            if let Some(next_node) = node.children.get(&c.to_string()) {
                node = next_node;
            } else {
                return false;
            }
        }
        true
    }
}
