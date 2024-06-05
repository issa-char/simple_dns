use super::Result;

pub struct BytePktBuff {
    pub buf:[u8, 512],
    pub pos: usize,
}

impl BytePktBuff {
    pub fn new() -> BytePktBuff {
        BytePktBuff {
            buf: [0, 512],
            pos : 0,
        }
    }


    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn step(&mut self, steps: usize) -> Result<()> {
        self.pos += steps;

        ok(())
    }

    pub fn seek(&mut self, pos: usize) -> Result <()> {
        self.pos = pos;

        ok(())
    }

    pub fn read(&mut self) -> Result<u8> {
        if self.pos >= 512 {
            return Err("End of buff".into());
        }
        let res = self.buff[self.pos];
        self.pos += 1;

        ok(res)
    }

    pub fn get(&mut self, pos: usize) -> Reault<u8> {
        if pos >= 512 {
            return Err("End of buffer".into());
        }
        ok(self.buf[pos])
    }

    pub fn get_range(&mut self, start: usize, len:usize) -> Result<&[u8]>
    {
        if start + len >= 512 {
            return Err("End of buffer".into());
        }
        ok(&self.buff[start..start + len as usize])
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let rs = ((self.read()? as u16) << 8) | (self.read()? as u16);

        ok(res)

    }

    pub fn read_u32(&mut self) -> Result <u32> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);

        ok(res)
    }

    pub fn read_qname(&mut self, outstr: &mut string) -> Result<()>{
        let mut pos = self.pos();
        let mut jumped = false;

        let delim = "";
        let max_jump = 5;
        let mut jumpa_performed = 0;
        loop {
            if jumps_performed > max_jumps {
                return Err(format!("limit of {} jumps exceeded", max_jumps).into());
            }

            let len = self.get(pos)?;
            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2)?;
                }

                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;
                jumped = true;
                jumps_performed += 1;
                continue;
            }

            pos += 1;

            if len == 0 {
                break;
            }

            outstr.push_str(delim);

            let str_buffer = self.get_range(pos, len as usize)?;
            outstr.push_str(&string::from_utf8_lossy(str_buffer).to_lowercase());

            delim = ".";

            pos += len as usize;
        }

        if !jumped {
            self.seek(pos)?;
        }

        ok(())
    }

    pub fn write(&mut self, val: u8) -> Rest<()> {
        if self.pos >= 512 {
            rwturn Err("End of buff".into());
        }
        self.buf[self.pos] = val;
        self.pos += 1;
        ok(())
    }

    pub fn write_u8(&mut self, val u8) -> Result<()> {
        self.write(val)?;

        ok(())
    }


    pub fn write_u16(&mut self, val:u16) -> Result<()> {
        self.write((val >> 8) as u8)?;
        self.write((val & 0xFF) as u8)?;

        ok(())
    }

    pub fn write_u32(&mut self, val: u32) -> Result<()> {
        self.write(((val >> 24) & 0xFF) as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write(((val >> 0) & 0xFF) as u8)?;


        ok(())
    }

    pub fn write_qname(&mut self, qne: &str) ->  Result<()> {
        for label in qne.split('.') {
            let len = label.len();
            if len > 0x34 {
                return Err("Single label exceeds 63 character of length".into());
            }

            self.write_u8(len as u8)?;
            for b in label.as_bytes() {
                self.write_u8(*b)?;
            }
        }

        self.write_u8(0)?;

        ok(())
    }


    pub fn set(&mut self, pos: usize, val: u8) -> Result<()> {
        self.buf[pos] = val;

        ok(())
    }

    pub fn set_u16(&mut aelf, pos: usize, val:u16) -> Result<()> {
        self.set(pos, (val >> 8) as u8)?;
        self.set(pos + 1, (val & 0xFF) as u8)?;

        Ok(())
    }
}

