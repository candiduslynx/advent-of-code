pub(crate) fn hash(s: &str) -> usize {
    s.as_bytes().iter().fold(0usize, |s, &b| {
        const U8_MASK: usize = 255;
        let s = (s + b as usize) & U8_MASK;
        let p = (s << 4) + s;
        let n = p & U8_MASK;
        n
    })
}
