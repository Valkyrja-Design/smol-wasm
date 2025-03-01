use super::types;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum SectionCode {
    Custom = 0x00,
    Type = 0x01,
    Import = 0x02,
    Function = 0x03,
    Table = 0x04,
    Memory = 0x05,
    Global = 0x06,
    Export = 0x07,
    Start = 0x08,
    Element = 0x09,
    Code = 0x0a,
    Data = 0x0b,
    DataCount = 0x0c,
}

impl From<u8> for SectionCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Custom,
            0x01 => Self::Type,
            0x02 => Self::Import,
            0x03 => Self::Function,
            0x04 => Self::Table,
            0x05 => Self::Memory,
            0x06 => Self::Global,
            0x07 => Self::Export,
            0x08 => Self::Start,
            0x09 => Self::Element,
            0x0a => Self::Code,
            0x0b => Self::Data,
            0x0c => Self::DataCount,
            _ => panic!("invalid section code {}", value)
        }
    }    
}

#[derive(Debug, PartialEq)]
pub struct TypeSection {
    pub sec_code: SectionCode,
    pub sec_size: usize,
    pub func_types: Vec<types::FuncType>,
}