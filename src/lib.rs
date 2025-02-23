pub mod parser;

#[derive(Debug)]
pub struct Module {
    pub magic_number: String,
    pub version: u32,
}