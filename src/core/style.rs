#[derive(Clone, Debug)]
pub enum Style {
    Background(String),
    Blink,
    BlinkFast,
    Bold,
    Conceal,
    Dim,
    Encircle,
    Frame,
    Foreground(String),
    Italic,
    None,
    Overline,
    Reverse,
    Strike,
    Underline,
    UnderlineMore,
}

impl Style {
    pub fn ansi_escape_code(
        &self,
        storage: crate::core::color::storage::Storage,
    ) -> String {
        let sgr = self.ansi_sgr(storage);
        let sgr: Vec<String> = sgr.iter().map(u8::to_string).collect();
        if sgr.is_empty() {
            String::new()
        } else {
            format!("\u{1b}[{}m", sgr.join(";"))
        }
    }

    pub fn ansi_sgr(
        &self,
        storage: crate::core::color::storage::Storage,
    ) -> Vec<u8> {
        match self {
            Style::Background(color) => {
                if let Ok(color) = crate::core::color::Color::new(color) {
                    if let Some(color) = color.to_storage(storage) {
                        if let Ok(sgr) = color.ansi_sgr(false) {
                            return sgr;
                        }
                    }
                }
                vec![]
            }
            Style::Blink => {
                vec![5]
            }
            Style::BlinkFast => {
                vec![6]
            }
            Style::Bold => {
                vec![1]
            }
            Style::Conceal => {
                vec![8]
            }
            Style::Dim => {
                vec![2]
            }
            Style::Encircle => {
                vec![52]
            }
            Style::Foreground(color) => {
                if let Ok(color) = crate::core::color::Color::new(color) {
                    if let Some(color) = color.to_storage(storage) {
                        if let Ok(sgr) = color.ansi_sgr(true) {
                            return sgr;
                        }
                    }
                }
                vec![]
            }
            Style::Frame => {
                vec![51]
            }
            Style::Italic => {
                vec![3]
            }
            Style::None => {
                vec![0]
            }
            Style::Overline => {
                vec![53]
            }
            Style::Reverse => {
                vec![7]
            }
            Style::Strike => {
                vec![9]
            }
            Style::Underline => {
                vec![4]
            }
            Style::UnderlineMore => {
                vec![21]
            }
        }
    }
}

#[cfg(test)]
mod test_style {
    use super::Style;

    #[test]
    fn ansi_sgr_none() {
        assert_eq!(
            Style::None.ansi_sgr(crate::core::color::storage::Storage::Bits24),
            [0]
        );
    }

    #[test]
    fn ansi_sgr_background() {
        assert_eq!(
            Style::Background("black".to_string())
                .ansi_sgr(crate::core::color::storage::Storage::Bits24),
            [40]
        );
    }

    #[test]
    fn ansi_sgr_reset_background() {
        assert_eq!(
            Style::Background("default".to_string())
                .ansi_sgr(crate::core::color::storage::Storage::Bits24),
            [49]
        );
    }

    #[test]
    fn ansi_sgr_bold() {
        assert_eq!(
            Style::Bold.ansi_sgr(crate::core::color::storage::Storage::Bits24),
            [1]
        );
    }

    #[test]
    fn ansi_sgr_dim() {
        assert_eq!(
            Style::Dim.ansi_sgr(crate::core::color::storage::Storage::Bits24),
            [2]
        );
    }

    #[test]
    fn ansi_sgr_foreground() {
        assert_eq!(
            Style::Foreground("black".to_string())
                .ansi_sgr(crate::core::color::storage::Storage::Bits24),
            [30]
        );
    }

    #[test]
    fn ansi_sgr_reset_foreground() {
        assert_eq!(
            Style::Foreground("default".to_string())
                .ansi_sgr(crate::core::color::storage::Storage::Bits24),
            [39]
        );
    }
}
