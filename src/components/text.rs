pub struct Text {
    style: crate::base::style::Style,
    text:  String,
}

impl Text {
    pub fn new(
        text: String,
        style: crate::base::style::Style,
    ) -> Text {
        Text { style, text }
    }
}

impl crate::base::render::Render for Text {
    fn render(
        &self,
        options: &crate::base::render::Options,
    ) -> crate::base::render::Segments {
        let mut segments = Vec::new();

        for line in wrap(&self.text, options.columns, options.rows) {
            segments.push((line, self.style));
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
        let line = format!("{:columns$}", line, columns = columns);
        if lines.len() < rows {
            lines.push(line);
            lines.push(String::from("\n"));
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
            "0123", "4567", "89  ", "0123", "4   ", "5678", "9   "
        ],);
    }
}
