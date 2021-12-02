pub type Segment = (String, crate::base::style::Style);
pub type Segments = Vec<Segment>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    pub columns: usize,
    pub is_tty:  bool,
    pub rows:    usize,
}

pub trait Render {
    fn render(&self, options: &Options) -> Segments;
}
