pub struct Style {
    bold: Option<bool>,
}

impl Default for Style {
    fn default() -> Style {
        Style { bold: None }
    }
}

impl Style {
    pub fn to_ansi_sgr_parameters(&self) -> Vec<u8> {
        let mut sgr: Vec<u8> = Vec::new();

        if let Some(bold) = self.bold {
            sgr.push(if bold { 1 } else { 21 });
        }

        sgr
    }

    pub fn to_ansi_escape_code(&self) -> String {
        let sgr = self.to_ansi_sgr_parameters();
        let sgr: Vec<String> = sgr.iter().map(u8::to_string).collect();

        format!("\u{1b}[{}m", sgr.join(";"))
    }
}

#[cfg(test)]
mod tests {
    use super::Style;

    #[test]
    fn to_ansi_default() {
        assert_eq!(Style::default().to_ansi_sgr_parameters(), []);
    }

    #[test]
    fn to_ansi_bold_false() {
        let style = Style {
            bold: Some(false),
            ..Style::default()
        };
        assert_eq!(style.to_ansi_sgr_parameters(), [21]);
    }

    #[test]
    fn to_ansi_bold_true() {
        let style = Style {
            bold: Some(true),
            ..Style::default()
        };
        assert_eq!(style.to_ansi_sgr_parameters(), [1]);
    }
}
