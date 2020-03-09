use crate::value::obj_str::ObjStr;
use ustr::Ustr;

#[derive(Debug, Clone)]
pub enum Obj {
    String(ObjStr),
}
