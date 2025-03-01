use std::fs;
use std::io::*;
use std::path;

mod parser;
mod section;
mod types;

#[derive(Debug)]
pub struct Module {
    pub magic_number: String,
    pub version: u32,
    pub type_section: Option<section::TypeSection>,
}

impl Module {
    pub fn decode_file(file: &str) -> std::result::Result<Module, String> {
        let path = path::PathBuf::from(&file);
        let display = path.display();

        let mut file = match fs::File::open(&path) {
            Err(err) => return Err(format!("couldn't open {}: {}", display, err)),
            Ok(file) => file,
        };

        let mut contents: Vec<u8> = Vec::new();

        if let Err(err) = file.read_to_end(&mut contents) {
            return Err(format!("couldn't read {}: {}", display, err));
        };

        let mut parser = parser::Parser::new("#raw", &contents);

        parser.parse()
    }

    pub fn decode_raw_bytes(binary: &str, bytes: &[u8]) -> std::result::Result<Module, String> {
        let mut parser = parser::Parser::new(binary, bytes);

        parser.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal() {
        let binary = wat::parse_str("(module)").unwrap();
        let mut module = Module::decode_raw_bytes("#raw", &binary).unwrap();

        assert_eq!(module.magic_number, "\0asm", "{:#?}", module);
        assert_eq!(module.version, 1u32, "{:#?}", module);

        module = Module::decode_file("tests/minimal.wasm").unwrap();

        assert_eq!(module.magic_number, "\0asm", "{:#?}", module);
        assert_eq!(module.version, 1u32, "{:#?}", module);
    }

    #[test]
    fn functions() {
        let binary = wat::parse_file("tests/functions.wat").unwrap();
        let mut module = Module::decode_raw_bytes("#raw", &binary).unwrap();
        let type_section = Some(section::TypeSection {
            sec_code: section::SectionCode::Type,
            sec_size: 0x07,
            func_types: vec![types::FuncType {
                params: vec![types::ValueType::I32, types::ValueType::I64],
                results: vec![types::ValueType::I64],
            }],
        });

        assert_eq!(module.magic_number, "\0asm", "{:#?}", module);
        assert_eq!(module.version, 1u32, "{:#?}", module);
        assert_eq!(module.type_section, type_section);
        
        module = Module::decode_file("tests/functions.wasm").unwrap();

        assert_eq!(module.magic_number, "\0asm", "{:#?}", module);
        assert_eq!(module.version, 1u32, "{:#?}", module);
        assert_eq!(module.type_section, type_section);
    }
}
