pub enum TextAlignment {
    Center,
    Left,
    Right,
}

pub struct Text {
    pub align:    TextAlignment,
    pub portions: Vec<crate::core::segment::SegmentPortion>,
}

impl crate::core::render::Render for Text {
    fn render(
        &self,
        options: &crate::core::render::Options,
    ) -> crate::core::segment::RenderedSegments {
        let columns = match options.columns {
            Some(columns) => columns,
            None => crate::core::render::DEFAULT_COLUMNS,
        };

        let mut rendered_segments = Vec::new();
        let mut spans: Vec<Vec<crate::core::segment::SegmentPortion>> =
            vec![vec![]];

        for portion in self.portions.iter() {
            spans.last_mut().unwrap().push(portion.clone());
            match portion {
                crate::core::segment::SegmentPortion::Text(_) => {
                    spans.push(Vec::new());
                }
                _ => {}
            }
        }

        let mut segments: Vec<crate::core::segment::Segment> =
            vec![crate::core::segment::Segment::new()];

        for span in spans.iter() {
            for portion in span.iter() {
                match portion {
                    crate::core::segment::SegmentPortion::Style(style) => {
                        segments.last_mut().unwrap().add_style(style.clone());
                    }
                    crate::core::segment::SegmentPortion::Text(text) => {
                        let mut text = &text[..];
                        while text.len() > columns {
                            let clone = segments.last_mut().unwrap().clone();

                            segments
                                .last_mut()
                                .unwrap()
                                .add_text(&text[0..columns]);
                            segments.push(clone);
                            text = &text[columns..];
                        }
                        segments.last_mut().unwrap().add_text(&text);
                    }
                }
            }
        }

        rendered_segments.reserve(segments.len());
        for segment in segments.iter() {
            rendered_segments.push(segment.render(
                match self.align {
                    TextAlignment::Center => {
                        crate::core::segment::SegmentPadding::Center(columns)
                    }
                    TextAlignment::Left => {
                        crate::core::segment::SegmentPadding::Left(columns)
                    }
                    TextAlignment::Right => {
                        crate::core::segment::SegmentPadding::Right(columns)
                    }
                },
                options.storage,
            ));
        }

        rendered_segments
    }
}
