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
        let (magic_number, version) = self.parse_preamble()?;
        let type_section = self.parse_type_section()?;
        let func_section = self.parse_func_section()?;
        let module = super::Module {
            magic_number,
            version,
            type_section,
            func_section,
        };

        Ok(module)
    }

    fn parse_preamble(&mut self) -> std::result::Result<(String, u32), String> {
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

        let version = self.parse_u32()?;

        Ok((magic_number, version))
    }

    pub fn parse_type_section(
        &mut self,
    ) -> std::result::Result<Option<section::TypeSection>, String> {
        if self.curr_pos >= self.nbytes {
            return Ok(None);
        }

        let sec_code = section::SectionCode::from(self.bytes[self.curr_pos]);

        if sec_code != section::SectionCode::Type {
            return Ok(None);
        }

        self.curr_pos += 1;

        let sec_size = self.parse_leb128_unsigned()? as usize;
        let ntypes = self.parse_leb128_unsigned()? as usize;
        let func_types = self.parse_func_types(ntypes)?;

        Ok(Some(section::TypeSection {
            sec_code,
            sec_size,
            func_types,
        }))
    }

    fn parse_func_types(
        &mut self,
        ntypes: usize,
    ) -> std::result::Result<Vec<types::FuncType>, String> {
        let mut func_types: Vec<types::FuncType> = Vec::with_capacity(ntypes);

        for _ in 0..ntypes {
            func_types.push(self.parse_func_type()?);
        }

        Ok(func_types)
    }

    fn parse_func_type(&mut self) -> std::result::Result<types::FuncType, String> {
        if self.curr_pos >= self.nbytes {
            return Err(String::from("expected function byte (0x60)"));
        }

        let _func_byte = self.bytes[self.curr_pos];
        self.curr_pos += 1;

        let nparams = self.parse_leb128_unsigned()? as usize;
        let params = self.parse_value_types(nparams)?;
        let nresults = self.parse_leb128_unsigned()? as usize;
        let results = self.parse_value_types(nresults)?;

        Ok(types::FuncType { params, results })
    }

    fn parse_value_types(
        &mut self,
        ntypes: usize,
    ) -> std::result::Result<Vec<types::ValueType>, String> {
        let mut value_types: Vec<types::ValueType> = Vec::with_capacity(ntypes);

        for _ in 0..ntypes {
            if self.curr_pos >= self.nbytes {
                return Err(String::from("expected value type"));
            }

            value_types.push(types::ValueType::from(self.bytes[self.curr_pos]));
            self.curr_pos += 1;
        }

        Ok(value_types)
    }

    fn parse_u32(&mut self) -> std::result::Result<u32, String> {
        let input = &self.bytes[self.curr_pos..];
        let (int_bytes, _) = input.split_at(std::mem::size_of::<u32>());

        match int_bytes.try_into() {
            Err(err) => Err(format!("{}", err)),
            Ok(bytes) => {
                self.curr_pos += 4;
                Ok(u32::from_le_bytes(bytes))
            }
        }
    }

    fn parse_leb128_unsigned(&mut self) -> std::result::Result<u64, String> {
        let mut slice = &self.bytes[self.curr_pos..];
        let prev_len = slice.len();

        match leb128::read::unsigned(&mut slice) {
            Err(err) => Err(format!("expected unsigned LEB128 encoded integer: {}", err)),
            Ok(x) => {
                self.curr_pos += prev_len - slice.len();
                Ok(x)
            }
        }
    }

    fn parse_func_section(&mut self) -> std::result::Result<Option<section::FuncSection>, String> {
        if self.curr_pos >= self.nbytes {
            return Ok(None);
        }

        let sec_code = section::SectionCode::from(self.bytes[self.curr_pos]);

        if sec_code != section::SectionCode::Function {
            return Ok(None);
        }

        self.curr_pos += 1;

        let sec_size = self.parse_leb128_unsigned()? as usize;
        let nfuncs = self.parse_leb128_unsigned()? as usize;
        let mut type_indices = Vec::with_capacity(nfuncs);

        for _ in 0..nfuncs {
            type_indices.push(self.parse_leb128_unsigned()? as usize);
        }

        Ok(Some(section::FuncSection {
            sec_code,
            sec_size,
            type_indices,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leb128_unsigned() {
        let mut bytes: &[u8] = &[0xcb, 0xbe, 0xf1, 0x23];
        let mut parser = Parser::new("#raw", bytes);
        let i = parser.parse_leb128_unsigned();

        assert_eq!(i, Ok(75259723), "{:#?}", i);
        assert_eq!(parser.curr_pos, 4);

        bytes = &[0x02, 0x00, 0x00];
        parser = Parser::new("#raw", bytes);
        let i = parser.parse_leb128_unsigned();

        assert_eq!(i, Ok(2), "{:#?}", i);
        assert_eq!(parser.curr_pos, 1);
    }
}
