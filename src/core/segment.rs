#[derive(Clone, Debug)]
pub enum SegmentPortion {
    Style(crate::core::style::Style),
    Text(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SegmentPadding {
    Center(usize),
    Left(usize),
    None,
    Right(usize),
}

#[derive(Clone, Debug)]
pub struct Segment {
    pub portions: Vec<SegmentPortion>,
}

pub type RenderedSegments = Vec<String>;

impl Segment {
    pub fn new() -> Segment {
        Segment { portions: vec![] }
    }

    pub fn add_segment(
        &mut self,
        segment: &Segment,
    ) {
        self.portions.extend(segment.portions.clone());
    }

    pub fn add_text(
        &mut self,
        text: &str,
    ) {
        self.portions.push(SegmentPortion::Text(String::from(text)));
    }

    pub fn add_style(
        &mut self,
        style: crate::core::style::Style,
    ) {
        self.portions.push(SegmentPortion::Style(style));
    }

    pub fn render(
        &self,
        padding: SegmentPadding,
        storage: Option<crate::core::color::storage::Storage>,
    ) -> String {
        let mut rendered = String::new();

        let rendered_length: usize = self
            .portions
            .iter()
            .map(|portion| match portion {
                SegmentPortion::Text(text) => text.len(),
                SegmentPortion::Style(_) => 0,
            })
            .sum();

        match padding {
            SegmentPadding::Center(desired_length) => {
                if desired_length > rendered_length {
                    rendered.push_str(&whitespace(
                        (desired_length - rendered_length) / 2,
                    ));
                };
            },
            SegmentPadding::Right(desired_length) => {
                if desired_length > rendered_length {
                    rendered.push_str(&whitespace(
                        desired_length - rendered_length,
                    ));
                }
            },
            _ => {},
        };

        for portion in self.portions.iter() {
            match portion {
                SegmentPortion::Text(text) => {
                    rendered.push_str(&text);
                },
                SegmentPortion::Style(style) => match storage {
                    Some(storage) => {
                        rendered.push_str(&style.ansi_escape_code(storage));
                    },
                    None => {},
                },
            }
        }

        match storage {
            Some(storage) => rendered.push_str(
                &crate::core::style::Style::None.ansi_escape_code(storage),
            ),
            None => (),
        };

        match padding {
            SegmentPadding::Left(desired_length) => {
                if desired_length > rendered_length {
                    rendered.push_str(&whitespace(
                        desired_length - rendered_length,
                    ));
                }
            },
            SegmentPadding::Center(desired_length) => {
                if desired_length > rendered_length {
                    rendered.push_str(&whitespace(
                        desired_length
                            - rendered_length
                            - (desired_length - rendered_length) / 2,
                    ));
                }
            },
            _ => {},
        }

        rendered
    }
}

fn whitespace(length: usize) -> String {
    std::iter::repeat(" ").take(length).collect::<String>()
}

#[cfg(test)]
mod test_segment {
    use super::{Segment, SegmentPadding};

    #[test]
    fn new() {
        let mut segment = Segment::new();
        segment.add_style(crate::core::style::Style::Bold);
        segment.add_text("a b c d");

        assert_eq!(segment.render(SegmentPadding::None, None), "a b c d");
        assert_eq!(segment.render(SegmentPadding::Center(8), None), "a b c d ");
        assert_eq!(segment.render(SegmentPadding::Left(8), None), "a b c d ");
        assert_eq!(segment.render(SegmentPadding::Right(8), None), " a b c d");
        assert_eq!(
            segment.render(
                SegmentPadding::Center(9),
                Some(crate::core::color::storage::Storage::Bits24)
            ),
            " \u{1b}[1ma b c d\u{1b}[0m "
        );
    }
}
