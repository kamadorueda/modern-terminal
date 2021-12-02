pub struct ColorPalette {}

impl ColorPalette {
    pub fn new() -> ColorPalette { ColorPalette {} }
}

impl crate::base::render::Render for ColorPalette {
    fn render(
        &self,
        options: &crate::base::render::Options,
    ) -> crate::base::render::Segments {
        let mut segments = Vec::new();
        segments.reserve((1 + options.rows) * options.columns);

        for row in 0..(options.rows) {
            for column in 0..(options.columns) {
                let col_r = (column as f64) / ((options.columns - 1) as f64);
                let row_r = (row as f64) / ((options.rows - 1) as f64);

                let l = col_r;
                let h = (1.0 - 0.75 * row_r + 0.25 * col_r) / 0.75 % 1.0;

                let (r, g, b) =
                    crate::base::color::model::hsl_to_rgb(360.0 * h, 1.0, l);

                segments.push((
                    String::from(" "),
                    crate::base::style::Style::new()
                        .background(&format!("rgb({}, {}, {})", r, g, b)),
                ));
            }

            segments
                .push((String::from("\n"), crate::base::style::Style::new()));
        }

        segments
    }
}
