use leb128;
use std::fs;
use std::io::*;
use std::path;

pub struct Parser {
    file: path::PathBuf,
    file_contents: Vec<u8>,
    curr_pos: usize,
}

impl Parser {
    pub fn new(file: &str) -> std::result::Result<Self, String> {
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

        Ok(Parser {
            file: path,
            file_contents: contents,
            curr_pos: 0,
        })
    }

    pub fn parse(&self) -> std::result::Result<super::Module, String> {
        if self.file_contents.len() < 8 {
            return Err(format!(
                "expected atleast 8 bytes in the binary {}, found {}",
                self.file.display(),
                self.file_contents.len()
            ));
        }

        let magic_number = match std::str::from_utf8(&self.file_contents[..4]) {
            Err(err) => return Err(format!("expected valid utf8 magic number: {}", err)),
            Ok(magic) => String::from(magic),
        };

        let version = match leb128::read::unsigned(&mut &self.file_contents[4..8]) {
            Err(err) => return Err(format!("expected LEB128 encoded version: {}", err)),
            Ok(ver) => ver as u32,
        };

        let module = super::Module {
            magic_number,
            version,
        };

        Ok(module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal() {
        let path = "tests/minimal.wasm";
        let parser = Parser::new(path).unwrap();
        let module = parser.parse().unwrap();

        assert_eq!(module.magic_number, "\0asm", "{:#?}", module);
        assert_eq!(module.version, 1u32, "{:#?}", module);
    }
}
