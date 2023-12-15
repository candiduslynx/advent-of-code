use std::hash::Hasher;

pub(crate) fn hash(s: &str) -> usize {
    s.as_bytes().iter().fold(0usize, |s, &b| {
        const U8_MASK: usize = 255;
        let s = (s + b as usize) & U8_MASK;
        let p = (s << 4) + s;
        let n = p & U8_MASK;
        n
    })
}

pub(crate) struct ShiftSumHasher {
    state: u64,
}

impl ShiftSumHasher {
    pub(crate) fn new() -> Self {
        ShiftSumHasher { state: 0u64 }
    }

    pub(crate) fn calc(bytes: &[u8]) -> u64 {
        let mut s = ShiftSumHasher::new();
        s.write(bytes);
        s.finish()
    }
}

impl std::hash::Hasher for ShiftSumHasher {
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
