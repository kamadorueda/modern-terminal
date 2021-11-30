pub struct Style {
    background: Option<crate::color::Color>,
    bold: Option<bool>,
    dim: Option<bool>,
    foreground: Option<crate::color::Color>,
}

impl Style {
    pub fn new() -> Style {
        Style {
            background: None,
            bold: None,
            dim: None,
            foreground: None,
        }
    }
}

impl Style {
    pub fn background(&mut self, color: &str) -> &mut Style {
        match crate::color::Color::new(color) {
            Ok(color) => {
                self.background = Some(color);
                self
            }
            _ => self,
        }
    }

    pub fn reset_background(&mut self) -> &mut Style {
        self.background("default")
    }
}

impl Style {
    pub fn bold(&mut self) -> &mut Style {
        self.bold = Some(true);
        self
    }

    pub fn not_bold(&mut self) -> &mut Style {
        self.bold = Some(false);
        self
    }
}

impl Style {
    pub fn dim(&mut self) -> &mut Style {
        self.dim = Some(true);
        self
    }

    pub fn not_dim(&mut self) -> &mut Style {
        self.dim = Some(false);
        self
    }
}

impl Style {
    pub fn foreground(&mut self, color: &str) -> &mut Style {
        match crate::color::Color::new(color) {
            Ok(color) => {
                self.foreground = Some(color);
                self
            }
            _ => self,
        }
    }

    pub fn reset_foreground(&mut self) -> &mut Style {
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
                if let Ok(mut background_sgr) = background.ansi_sgr(false) {
                    sgr.append(&mut background_sgr);
                }
            }
        }
        if let Some(bold) = self.bold {
            if bold {
                sgr.push(1)
            };
        }
        if let Some(dim) = self.dim {
            if dim {
                sgr.push(2)
            };
        }
        if let Some(foreground) = self.foreground {
            if let Some(foreground) = foreground.to_space(space) {
                if let Ok(mut foreground_sgr) = foreground.ansi_sgr(true) {
                    sgr.append(&mut foreground_sgr);
                }
            }
        }

        sgr
    }
}

#[cfg(test)]
mod test_style {
    use super::Style;

    #[test]
    fn ansi_sgr() {
        assert_eq!(Style::new().ansi_sgr(crate::color::Space::Bits24), []);
    }

    #[test]
    fn ansi_sgr_background() {
        assert_eq!(
            Style::new()
                .background("black")
                .ansi_sgr(crate::color::Space::Bits24),
            [40]
        );
    }

    #[test]
    fn ansi_sgr_reset_background() {
        assert_eq!(
            Style::new()
                .reset_background()
                .ansi_sgr(crate::color::Space::Bits24),
            [49]
        );
    }

    #[test]
    fn ansi_sgr_bold() {
        assert_eq!(
            Style::new().bold().ansi_sgr(crate::color::Space::Bits24),
            [1]
        );
    }

    #[test]
    fn ansi_sgr_not_bold() {
        assert_eq!(
            Style::new()
                .bold()
                .not_bold()
                .ansi_sgr(crate::color::Space::Bits24),
            []
        );
    }

    #[test]
    fn ansi_sgr_dim() {
        assert_eq!(
            Style::new().dim().ansi_sgr(crate::color::Space::Bits24),
            [2]
        );
    }

    #[test]
    fn ansi_sgr_not_dim() {
        assert_eq!(
            Style::new()
                .dim()
                .not_dim()
                .ansi_sgr(crate::color::Space::Bits24),
            []
        );
    }

    #[test]
    fn ansi_sgr_foreground() {
        assert_eq!(
            Style::new()
                .foreground("black")
                .ansi_sgr(crate::color::Space::Bits24),
            [30]
        );
    }

    #[test]
    fn ansi_sgr_reset_foreground() {
        assert_eq!(
            Style::new()
                .reset_foreground()
                .ansi_sgr(crate::color::Space::Bits24),
            [39]
        );
    }
}
