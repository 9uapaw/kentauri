pub type Value = f64;

pub struct ValuePool {
    pub values: Vec<Value>
}

impl ValuePool {

    pub fn new() -> Self {
       ValuePool{values: Vec::new()}
    }
}