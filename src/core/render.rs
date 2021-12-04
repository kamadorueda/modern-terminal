#[derive(Debug)]
pub struct Segment {
    pub text:  String,
    pub style: crate::core::style::Style,
}

pub type Segments = Vec<Segment>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    pub columns: usize,
    pub is_tty:  bool,
    pub rows:    usize,
}

pub trait Render {
    fn render(
        &self,
        options: &Options,
    ) -> Segments;
}
