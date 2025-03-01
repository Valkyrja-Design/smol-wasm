#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValueType {
    I32, // 0x7f
    I64, // 0x7e
}

impl From<u8> for ValueType {
    fn from(value: u8) -> Self {
        match value {
            0x7f => Self::I32,
            0x7e => Self::I64,
            _ => panic!(
                "invalid value type {}, expected one of {{0x7f, 0x7e}}",
                value
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FuncType {
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}
