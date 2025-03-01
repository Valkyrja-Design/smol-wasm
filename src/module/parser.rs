use super::{section, types};
use leb128;

pub struct Parser {
    binary: String,
    bytes: Vec<u8>,
    curr_pos: usize,
    nbytes: usize,
}

impl Parser {
    pub fn new(binary: &str, bytes: &[u8]) -> Parser {
        Parser {
            binary: String::from(binary),
            bytes: bytes.to_vec(),
            curr_pos: 0,
            nbytes: bytes.len(),
        }
    }

    pub fn parse(&mut self) -> std::result::Result<super::Module, String> {
        if self.nbytes < 8 {
            return Err(format!(
                "expected atleast 8 bytes in the binary {}, found {}",
                self.binary, self.nbytes
            ));
        }

        let magic_number = match std::str::from_utf8(&self.bytes[..4]) {
            Err(err) => return Err(format!("expected valid UTF8 encoded magic number: {}", err)),
            Ok(magic) => String::from(magic),
        };

        self.curr_pos = 4;

        let version = self.parse_leb128_unsigned(4)? as u32;
        let type_section = self.parse_type_section()?;
        let module = super::Module {
            magic_number,
            version,
            type_section,
        };

        Ok(module)
    }

    pub fn parse_type_section(
        &mut self,
    ) -> std::result::Result<Option<section::TypeSection>, String> {
        if self.curr_pos < self.nbytes {
            let sec_code = section::SectionCode::from(self.bytes[self.curr_pos]);

            if sec_code != section::SectionCode::Type {
                return Ok(None);
            }

            self.curr_pos += 1;

            let sec_size = self.parse_leb128_unsigned(4)? as usize;
            let ntypes = self.parse_leb128_unsigned(4)? as usize;
            let func_types = self.parse_func_types(ntypes)?;

            return Ok(Some(section::TypeSection {
                sec_code,
                sec_size,
                func_types,
            }));
        }

        Ok(None)
    }

    fn parse_leb128_unsigned(&mut self, size: usize) -> std::result::Result<u64, String> {
        if self.curr_pos + size > self.nbytes {
            return Err(format!("expected {} bytes of unsigned LEB128 encoded integer", size));
        }

        match leb128::read::unsigned(&mut &self.bytes[self.curr_pos..self.curr_pos + 4]) {
            Err(err) => Err(format!(
                "expected {} bytes of unsigned LEB128 encoded integer: {}",
                size, err
            )),
            Ok(x) => {
                self.curr_pos += size;
                Ok(x)
            }
        }
    }

    // fn parse_leb128_signed(&mut self, size: usize) -> std::result::Result<i64, String> {
    //     if self.curr_pos + size > self.nbytes {
    //         return Err(format!("expected {} bytes of signed LEB128 encoded integer", size));
    //     }

    //     match leb128::read::signed(&mut &self.bytes[self.curr_pos..self.curr_pos + 4]) {
    //         Err(err) => Err(format!(
    //             "expected {} bytes of signed LEB128 encoded integer: {}",
    //             size, err
    //         )),
    //         Ok(x) => {
    //             self.curr_pos += size;
    //             Ok(x)
    //         }
    //     }
    // }

    fn parse_func_types(
        &mut self,
        ntypes: usize,
    ) -> std::result::Result<Vec<types::FuncType>, String> {
        if self.curr_pos >= self.nbytes {
            return Err(String::from("expected function byte (0x60)"));
        }

        let _func_byte = self.bytes[self.curr_pos];
        self.curr_pos += 1;

        let func_types: Vec<types::FuncType> = Vec::with_capacity(ntypes);

        Ok(func_types)
    }

    fn parse_value_types(
        &self,
        ntypes: usize,
    ) -> std::result::Result<Vec<types::ValueType>, String> {
        let value_types: Vec<types::ValueType> = Vec::with_capacity(ntypes);

        Ok(value_types)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leb128_unsigned() {
        let bytes: &[u8] = &[0xcb, 0xbe, 0xf1, 0x23];
        let mut parser = Parser::new("#raw", bytes);
        let i = parser.parse_leb128_unsigned(4);

        assert_eq!(i, Ok(75259723), "{:#?}", i);
    }
}
