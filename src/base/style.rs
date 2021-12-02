#[derive(Clone, Copy)]
pub struct Style {
    background: Option<crate::base::color::Color>,
    bold:       Option<bool>,
    dim:        Option<bool>,
    foreground: Option<crate::base::color::Color>,
}

impl Style {
    pub fn new() -> Style {
        Style {
            background: None,
            bold:       None,
            dim:        None,
            foreground: None,
        }
    }
}

impl Style {
    pub fn background(&self, color: &str) -> Style {
        match crate::base::color::Color::new(color) {
            | Ok(color) => Style { background: Some(color), ..*self },
            | _ => *self,
        }
    }

    pub fn reset_background(&self) -> Style { self.background("default") }
}

impl Style {
    pub fn bold(&self) -> Style { Style { bold: Some(true), ..*self } }

    pub fn not_bold(&self) -> Style { Style { bold: Some(false), ..*self } }
}

impl Style {
    pub fn dim(&self) -> Style { Style { dim: Some(true), ..*self } }

    pub fn not_dim(&self) -> Style { Style { dim: Some(false), ..*self } }
}

impl Style {
    pub fn foreground(&self, color: &str) -> Style {
        match crate::base::color::Color::new(color) {
            | Ok(color) => Style { foreground: Some(color), ..*self },
            | _ => *self,
        }
    }

    pub fn reset_foreground(&self) -> Style { self.foreground("default") }
}

impl Style {
    pub fn ansi_escape_code(
        &self,
        storage: crate::base::color::storage::Storage,
    ) -> String {
        let sgr = self.ansi_sgr(storage);
        let sgr: Vec<String> = sgr.iter().map(u8::to_string).collect();

        format!("\u{1b}[{}m", sgr.join(";"))
    }

    pub fn ansi_sgr(
        &self,
        storage: crate::base::color::storage::Storage,
    ) -> Vec<u8> {
        let mut sgr: Vec<u8> = Vec::new();

        if let Some(background) = self.background {
            if let Some(background) = background.to_storage(storage) {
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
            if let Some(foreground) = foreground.to_storage(storage) {
                if let Ok(mut foreground_sgr) = foreground.ansi_sgr(true) {
                    sgr.append(&mut foreground_sgr);
                }
            }
        }

        sgr
    }

    pub fn render(
        &self,
        text: &str,
        storage: crate::base::color::storage::Storage,
    ) -> String {
        format!("{}{}", self.ansi_escape_code(storage), text)
    }
}

#[cfg(test)]
mod test_style {
    use super::Style;

    #[test]
    fn ansi_sgr() {
        assert_eq!(
            Style::new().ansi_sgr(crate::base::color::storage::Storage::Bits24),
            []
        );
    }

    #[test]
    fn ansi_sgr_background() {
        assert_eq!(
            Style::new()
                .background("black")
                .ansi_sgr(crate::base::color::storage::Storage::Bits24),
            [40]
        );
    }

    #[test]
    fn ansi_sgr_reset_background() {
        assert_eq!(
            Style::new()
                .reset_background()
                .ansi_sgr(crate::base::color::storage::Storage::Bits24),
            [49]
        );
    }

    #[test]
    fn ansi_sgr_bold() {
        assert_eq!(
            Style::new()
                .bold()
                .ansi_sgr(crate::base::color::storage::Storage::Bits24),
            [1]
        );
    }

    #[test]
    fn ansi_sgr_not_bold() {
        assert_eq!(
            Style::new()
                .bold()
                .not_bold()
                .ansi_sgr(crate::base::color::storage::Storage::Bits24),
            []
        );
    }

    #[test]
    fn ansi_sgr_dim() {
        assert_eq!(
            Style::new()
                .dim()
                .ansi_sgr(crate::base::color::storage::Storage::Bits24),
            [2]
        );
    }

    #[test]
    fn ansi_sgr_not_dim() {
        assert_eq!(
            Style::new()
                .dim()
                .not_dim()
                .ansi_sgr(crate::base::color::storage::Storage::Bits24),
            []
        );
    }

    #[test]
    fn ansi_sgr_foreground() {
        assert_eq!(
            Style::new()
                .foreground("black")
                .ansi_sgr(crate::base::color::storage::Storage::Bits24),
            [30]
        );
    }

    #[test]
    fn ansi_sgr_reset_foreground() {
        assert_eq!(
            Style::new()
                .reset_foreground()
                .ansi_sgr(crate::base::color::storage::Storage::Bits24),
            [39]
        );
    }
}
