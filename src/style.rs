pub struct Style {
    background: Option<crate::color::Color>,
    bold: Option<bool>,
    foreground: Option<crate::color::Color>,
}

impl Style {
    pub fn new() -> Style {
        Style {
            background: None,
            bold: None,
            foreground: None,
        }
    }
}

impl Style {
    pub fn background(&self, color: &str) -> Style {
        match crate::color::Color::new(color) {
            Ok(color) => Style {
                background: Some(color),
                ..*self
            },
            _ => Style { ..*self },
        }
    }

    pub fn bold(&self) -> Style {
        Style {
            bold: Some(true),
            ..*self
        }
    }

    pub fn foreground(&self, color: &str) -> Style {
        match crate::color::Color::new(color) {
            Ok(color) => Style {
                foreground: Some(color),
                ..*self
            },
            _ => Style { ..*self },
        }
    }

    pub fn not_background(&self) -> Style {
        self.background("default")
    }

    pub fn not_bold(&self) -> Style {
        Style {
            bold: Some(false),
            ..*self
        }
    }

    pub fn not_foreground(&self) -> Style {
        self.foreground("default")
    }
}

impl Style {
    pub fn ansi_escape_code(&self, space: crate::color::Space) -> String {
        let sgr = self.ansi_sgr(space);
        let sgr: Vec<String> = sgr.iter().map(u8::to_string).collect();

        format!("\u{1b}[{}m", sgr.join(";"))
    }

    pub fn ansi_sgr(&self, space: crate::color::Space) -> Vec<u8> {
        let mut sgr: Vec<u8> = Vec::new();

        if let Some(background) = self.background {
            if let Some(background) = background.to_space(space) {
                if let Ok(mut background_sgr) = background.sgr(false) {
                    sgr.append(&mut background_sgr);
                }
            }
        }
        if let Some(bold) = self.bold {
            sgr.push(if bold { 1 } else { 21 });
        }
        if let Some(foreground) = self.foreground {
            if let Some(foreground) = foreground.to_space(space) {
                if let Ok(mut foreground_sgr) = foreground.sgr(true) {
                    sgr.append(&mut foreground_sgr);
                }
            }
        }

        sgr
    }
}

#[cfg(test)]
mod tests {
    use super::Style;

    #[test]
    fn ansi_sgr() {
        let style = Style::new();
        assert_eq!(style.ansi_sgr(crate::color::Space::Bits24), []);
    }

    #[test]
    fn ansi_sgr_background() {
        let style = Style::new().background("black");
        assert_eq!(style.ansi_sgr(crate::color::Space::Bits24), [40]);
    }

    #[test]
    fn ansi_sgr_bold() {
        let style = Style::new().bold();
        assert_eq!(style.ansi_sgr(crate::color::Space::Bits24), [1]);
    }

    #[test]
    fn ansi_sgr_foreground() {
        let style = Style::new().foreground("black");
        assert_eq!(style.ansi_sgr(crate::color::Space::Bits24), [30]);
    }

    #[test]
    fn ansi_sgr_not_background() {
        let style = Style::new().not_background();
        assert_eq!(style.ansi_sgr(crate::color::Space::Bits24), [49]);
    }

    #[test]
    fn ansi_sgr_not_bold() {
        let style = Style::new().not_bold();
        assert_eq!(style.ansi_sgr(crate::color::Space::Bits24), [21]);
    }

    #[test]
    fn ansi_sgr_not_foreground() {
        let style = Style::new().not_foreground();
        assert_eq!(style.ansi_sgr(crate::color::Space::Bits24), [39]);
    }
}
