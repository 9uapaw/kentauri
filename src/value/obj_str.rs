use ustr::Ustr;

#[derive(Debug, Clone)]
pub struct ObjStr {
    pub string: Ustr,
}

impl ObjStr {
    pub fn new(value: &str) -> Self {
        ObjStr {
            string: Ustr::from(value),
        }
    }
}
