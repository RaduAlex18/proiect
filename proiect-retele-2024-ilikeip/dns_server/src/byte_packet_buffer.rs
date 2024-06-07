
pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}
impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }
    pub fn step(&mut self, steps: usize) -> Result<&mut Self, String> {
        match self.pos.checked_add(steps) {
            Some(x) => {
                self.pos = x;
                Ok(self)
            }
            None => Err("Packet buffer overflow".to_string())
        }
    }
    pub fn seek(&mut self, pos: usize) -> Result<&mut Self, String> {
        if pos >= self.buf.len() {
            Err("Buffer position out of bounds".to_string())
        } else {
            self.pos = pos;
            Ok(self)
        }
    }
    pub fn read_byte(&mut self) -> Result<u8, String> {
        let value = *self.buf.get(self.pos).unwrap();
        self.step(1)?;
        Ok(value)
    }
    pub fn get(&mut self, pos: usize) -> Result<u8, String> {
        match self.buf.get(pos) {
            Some(x) => Ok(*x),
            None => Err("Byte request out of bounds".to_string())
        }
    }
    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], String> {
        match self.buf.get(start..(start + len)) {
            Some(slice) => Ok(slice),
            None => Err("Out of bounds range byte request".to_string())
        }
    }
    pub fn read_word(&mut self) -> Result<u16, String> {
        let value = ((self.read_byte()? as u16) << 8) | (self.read_byte()? as u16);
        Ok(value)
    }
    pub fn read_dword(&mut self) -> Result<u32, String> {
        let value =
            ((self.read_byte()? as u32) << 24)
            | ((self.read_byte()? as u32) << 16)
            | ((self.read_byte()? as u32) << 8)
            | (self.read_byte()? as u32);

        Ok(value)
    }
    pub fn read_qname(&mut self, outstr: &mut String) -> Result<&mut Self, String> {
        let mut pos = self.pos;

        let mut jumped = false;
        let max_jumps = 5;
        let mut jumps_performed = 0;

        let mut delim = "";
        loop {
            if jumps_performed > max_jumps {
                return Err(format!("Limit of {} jumps exceeded", max_jumps));
            }

            let len = self.get(pos)?;
            if len & 0xC0 == 0xC0 {
                if jumped == false {
                    self.seek(pos + 2)?;
                }

                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len ^ 0xC0) as u16) << 8) | b2;
                pos = offset as usize;

                jumped = true;
                jumps_performed += 1;

                continue;
            }
            else {
                pos += 1;
                if len == 0 {
                    break;
                }

                outstr.push_str(delim);
                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(String::from_utf8_lossy(str_buffer).to_lowercase().as_str());

                delim = ".";
                pos += len as usize;
            }
        }
        if !jumped {
            self.seek(pos)?;
        }
        Ok(self)
    }

    pub fn write(&mut self, val: u8) -> Result<&mut Self, String> {
        if self.pos >= self.buf.len() {
            return Err("End of buffer".to_string());
        }
        self.buf[self.pos] = val;
        self.pos += 1;

        Ok(self)
    }

    pub fn write_byte(&mut self, val: u8) -> Result<&mut Self, String> {
        self.write(val)?;
        Ok(self)
    }

    pub fn write_word(&mut self, val: u16) -> Result<&mut Self, String> {
        self.write((val >> 8) as u8)?;
        self.write((val & 0xFF) as u8)?;

        Ok(self)
    }

    pub fn write_dword(&mut self, val: u32) -> Result<&mut Self, String> {
        self.write(((val >> 24) & 0xFF) as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write(((val >> 0) & 0xFF) as u8)?;

        Ok(self)
    }

    pub fn write_qname(&mut self, qname: &str) -> Result<&mut Self, String> {
        for label in qname.split('.') {
            let len = label.len();
            if len > 0x3f {
                return Err("Single label exceeds 63 characters of length".to_string());
            }

            self.write_byte(len as u8)?;
            for b in label.as_bytes() {
                self.write_byte(*b)?;
            }
        }
        self.write_byte(0)?;

        Ok(self)
    }

    pub fn set(&mut self, pos: usize, val: u8) -> Result<&mut Self, String> {
        self.buf[pos] = val;

        Ok(self)
    }

    pub fn set_word(&mut self, pos: usize, val: u16) -> Result<&mut Self, String> {
        self.set(pos, (val >> 8) as u8)?;
        self.set(pos + 1, (val & 0xFF) as u8)?;

        Ok(self)
    }
}
