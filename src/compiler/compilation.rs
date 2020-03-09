use crate::bytecode::chunk::Chunk;
use std::collections::HashSet;

pub struct Compilation {
    pub chunk: Option<Chunk>,
    pub intern_str: HashSet<String>,
}

impl Compilation {
    pub fn new() -> Self {
        Compilation {
            chunk: None,
            intern_str: HashSet::new(),
        }
    }
}
