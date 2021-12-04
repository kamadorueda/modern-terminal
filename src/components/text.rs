pub struct Text {
    pub styles: Vec<crate::core::style::Style>,
    pub text:   String,
}

impl crate::core::render::Render for Text {
    fn render(
        &self,
        options: &crate::core::render::Options,
    ) -> crate::core::segment::Segments {
        let mut segments = Vec::new();

        for line in wrap(&self.text, options.columns, options.rows) {
            let mut segment = crate::core::segment::Segment::new();
            for style in self.styles.iter() {
                segment.add_style(style.clone());
            }
            segment.add_text(&line);
            segments.push(segment);
        }

        segments
    }
}

fn wrap(
    text: &str,
    columns: usize,
    rows: usize,
) -> Vec<String> {
    let mut lines = Vec::new();

    for line in &mut textwrap::wrap(text, columns) {
        let line = String::from(match line {
            std::borrow::Cow::Borrowed(line) => *line,
            std::borrow::Cow::Owned(line) => line,
        });
        if lines.len() < rows {
            lines.push(line);
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::wrap;

    #[test]
    fn test_wrap() {
        assert_eq!(wrap("0123456789 01234 56789 012", 4, 7), vec![
            "0123", "4567", "89", "0123", "4", "5678", "9"
        ],);
    }
}
