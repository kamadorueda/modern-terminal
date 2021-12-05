pub struct ColorPalette {}

impl ColorPalette {
    pub fn new() -> ColorPalette {
        ColorPalette {}
    }
}

impl crate::core::render::Render for ColorPalette {
    fn render(
        &self,
        options: &crate::core::render::Options,
    ) -> crate::core::segment::RenderedSegments {
        let columns = match options.columns {
            Some(columns) => columns,
            None => crate::core::render::DEFAULT_COLUMNS,
        };
        let rows = match options.rows {
            Some(rows) => rows,
            None => crate::core::render::DEFAULT_ROWS,
        };

        let mut rendered_segments = Vec::new();
        rendered_segments.reserve(rows);

        for row in 0..(rows) {
            let mut segment = crate::core::segment::Segment::new();
            segment.portions.reserve(columns);

            for column in 0..(columns) {
                let col_r = (column as f64) / ((columns - 1) as f64);
                let row_r = (row as f64) / ((rows - 1) as f64);

                let l = col_r;
                let h = (1.0 - 0.75 * row_r + 0.25 * col_r) / 0.75 % 1.0;

                let (r, g, b) =
                    crate::core::color::model::hsl_to_rgb(360.0 * h, 1.0, l);

                segment.add_style(crate::core::style::Style::Background(
                    format!("rgb({}, {}, {})", r, g, b),
                ));
                segment.add_text(" ");
            }

            rendered_segments.push(segment.render(
                crate::core::segment::SegmentPadding::None,
                options.storage,
            ));
        }

        rendered_segments
    }
}
