pub struct Options {
    pub columns: u8,
    pub lines: u8,
}

pub trait Render {
    fn render(&self, options: Options) -> Vec<(String, crate::base::style::Style)>;
}
