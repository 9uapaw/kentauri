use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
pub enum OpCode {
    OP_RETURN,
    OP_CONST,
    OP_NEGATE,
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV
}
