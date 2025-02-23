use leb128;

pub struct Parser {
    binary: String,
    bytes: Vec<u8>,
    curr_pos: usize,
}

impl Parser {
    pub fn new(binary: &str, bytes: &[u8]) -> Parser {
        Parser {
            binary: String::from(binary),
            bytes: bytes.to_vec(),
            curr_pos: 0,
        }
    }

    pub fn parse(&mut self) -> std::result::Result<super::Module, String> {
        if self.bytes.len() < 8 {
            return Err(format!(
                "expected atleast 8 bytes in the binary {}, found {}",
                self.binary,
                self.bytes.len()
            ));
        }

        let magic_number = match std::str::from_utf8(&self.bytes[..4]) {
            Err(err) => return Err(format!("expected valid UTF8 encoded magic number: {}", err)),
            Ok(magic) => String::from(magic),
        };

        let version = match leb128::read::unsigned(&mut &self.bytes[4..8]) {
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
    
}
