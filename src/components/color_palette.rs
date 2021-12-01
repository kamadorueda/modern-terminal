pub struct ColorPalette {}

impl ColorPalette {
    pub fn new() -> ColorPalette {
        ColorPalette {}
    }
}

impl crate::base::render::Render for ColorPalette {
    fn render(&self, options: &crate::base::render::Options) -> crate::base::render::Segments {
        let mut segments = Vec::new();
        segments.reserve((1 + options.rows) * options.columns);

        for row in 0..(options.rows) {
            let l = 0.2 + 0.60 * (row as f64) / (options.rows as f64);
            for column in 0..(options.columns) {
                let h = 360.0 * (column as f64) / (options.columns as f64);
                let (r, g, b) = crate::base::color::model::hsl_to_rgb(h, 1.0, l);

                segments.push((
                    String::from(" "),
                    crate::base::style::Style::new()
                        .background(&format!("rgb({}, {}, {})", r, g, b)),
                ));
            }

            segments.push((String::from("\n"), crate::base::style::Style::new()));
        }

        segments
    }
}
