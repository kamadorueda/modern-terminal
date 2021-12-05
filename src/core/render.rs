pub const DEFAULT_COLUMNS: usize = 80;
pub const DEFAULT_ROWS: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    pub columns: Option<usize>,
    pub is_tty:  bool,
    pub rows:    Option<usize>,
    pub storage: Option<crate::core::color::storage::Storage>,
}

pub trait Render {
    fn render(
        &self,
        options: &Options,
    ) -> crate::core::segment::RenderedSegments;
}
