use std::hash::Hasher;

pub(crate) struct LensHasher {
    state: u64,
}

impl LensHasher {
    pub(crate) fn new() -> Self {
        LensHasher { state: 0u64 }
    }

    pub(crate) fn calc(data: &str) -> u64 {
        let mut s = LensHasher::new();
        s.write(data.as_bytes());
        s.finish()
    }
}

impl Hasher for LensHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            const U8_MASK: u64 = 255;
            self.state = (self.state + u64::from(byte)) & U8_MASK;
            self.state = ((self.state << 4) + self.state) & U8_MASK;
        }
    }
}
