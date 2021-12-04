#[derive(Debug)]
pub enum Part {
    Text(String),
    Style(crate::core::style::Style),
}

#[derive(Debug)]
pub struct Segment {
    pub parts: Vec<Part>,
}

pub type Segments = Vec<Segment>;

impl Segment {
    pub fn new() -> Segment {
        Segment { parts: vec![] }
    }

    pub fn add_text(
        &mut self,
        text: &str,
    ) {
        self.parts.push(Part::Text(String::from(text)));
    }

    pub fn add_style(
        &mut self,
        style: crate::core::style::Style,
    ) {
        self.parts.push(Part::Style(style));
    }

    pub fn render(
        &self,
        storage: Option<crate::core::color::storage::Storage>,
    ) -> String {
        self.parts
            .iter()
            .map(|part| match storage {
                Some(storage) => match part {
                    Part::Text(text) => text.clone(),
                    Part::Style(style) => style.ansi_escape_code(storage),
                },
                None => match part {
                    Part::Text(text) => text.clone(),
                    Part::Style(_) => String::new(),
                },
            })
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod test_segment {
    use super::Segment;

    #[test]
    fn new() {
        let mut segment = Segment::new();
        segment.add_style(crate::core::style::Style::Bold);
        segment.add_text("Hello, World!");

        assert_eq!(segment.render(None), "Hello, World!");
        assert_eq!(
            segment.render(Some(crate::core::color::storage::Storage::Bits24)),
            "\u{1b}[1mHello, World!"
        );
    }
}
