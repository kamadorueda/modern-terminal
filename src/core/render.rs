pub const DEFAULT_COLUMNS: usize = 80;
pub const DEFAULT_ROWS: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    pub columns: Option<usize>,
    pub is_tty:  bool,
    pub padding: crate::core::segment::SegmentPadding,
    pub rows:    Option<usize>,
}

pub trait Render {
    fn render(
        &self,
        options: &Options,
    ) -> crate::core::segment::Segments;
}
