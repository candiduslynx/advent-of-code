#[derive(Debug)]
pub(crate) struct Schema {
    pub(crate) numbers: Vec<u32>,
}

impl Schema {
	pub(crate) fn from_strings(_strings: Vec<String>) -> Self {
        Schema{numbers: (&[]).to_vec()}
    }
}
