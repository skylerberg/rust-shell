use std::collections::HashMap;

pub struct Shell {
    pub aliases : HashMap<String, String>,
}

impl Shell {
    pub fn new() -> Self {
        Shell { aliases : HashMap::new() }
    }
}

