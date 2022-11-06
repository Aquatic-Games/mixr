pub struct BinaryReader {
    pub data: Vec<u8>,
    pub position: usize
}

impl BinaryReader {
    pub fn new(path: &str) -> Result<BinaryReader, std::io::Error> {
        let data = std::fs::read(path);
        let data = match data {
            Ok(dat) => dat,
            Err(error) => return Err(error),
        };

        Ok(BinaryReader {
            data: data,
            position: 0
        })
    }

    pub fn read_u8(&mut self) -> u8 {
        let data = self.data[self.position];
        self.position += 1;
        data
    }

    pub fn read_i16(&mut self) -> i16 {
        let b1 = self.read_u8() as i16;
        let b2 = self.read_u8() as i16;

        b1 | (b2 << 8)
    }

    pub fn read_i32(&mut self) -> i32 {
        let b1 = self.read_u8() as i32;
        let b2 = self.read_u8() as i32;
        let b3 = self.read_u8() as i32;
        let b4 = self.read_u8() as i32;

        b1 | (b2 << 8) | (b3 << 16) | (b4 << 24)
    }

    pub fn read_string(&mut self, num_chars: i32) -> String {
        let mut text = String::new();

        for _ in 0..num_chars {
            text.push(self.data[self.position] as char);
            self.position += 1;
        }

        text
    }

    pub fn read_bytes(&mut self, num_bytes: usize) -> Vec<u8> {
        let data = self.data[self.position..(self.position + num_bytes)].to_vec();
        self.position += num_bytes;
        data
    }
}