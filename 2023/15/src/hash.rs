pub(crate) fn hash(s: &str) -> u8 {
    s.as_bytes().iter().fold(0u16, |s, &b| {
        const U8_MASK: u16 = 255;
        let s = (s + b as u16) & U8_MASK;
        let p = (s << 4) + s;
        let n = p & U8_MASK;
        n
    }) as u8
}
