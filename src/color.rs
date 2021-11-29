#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Space {
    Bits2,  // Terminal defaults
    Bits4,  // 4-bit (Standard)
    Bits8,  // 8-bit
    Bits24, // 24-bit (True Color)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    code: Option<u8>,
    rgb: Option<(u8, u8, u8)>,
    space: Space,
}

impl Color {
    pub fn sgr(&self, foreground: bool) -> Result<Vec<u8>, String> {
        match self {
            Color {
                space: Space::Bits2,
                ..
            } => Ok(vec![if foreground { 39 } else { 49 }]),

            Color {
                code: Some(code),
                space: Space::Bits4,
                ..
            } => match code {
                0..=7 => Ok(vec![code + if foreground { 30 } else { 40 }]),
                8..=15 => Ok(vec![code + if foreground { 82 } else { 92 }]),
                16..=255 => Err(format!("4 bits space cannot hold code: {}", code)),
            },

            Color {
                code: Some(code),
                space: Space::Bits8,
                ..
            } => Ok(vec![if foreground { 38 } else { 48 }, 5, *code]),

            Color {
                rgb: Some((r, g, b)),
                space: Space::Bits24,
                ..
            } => Ok(vec![if foreground { 38 } else { 48 }, 2, *r, *g, *b]),

            _ => Err(String::from("Invalid color")),
        }
    }

    pub fn new(color: &str) -> Result<Color, String> {
        let original = color;
        let color = original.trim().to_lowercase();

        if color == "default" {
            return Ok(Color {
                code: None,
                rgb: None,
                space: Space::Bits2,
            });
        }

        if let Some(code) = crate::color_codes::from_name(&color) {
            return match code {
                0..=7 => Ok(Color {
                    code: Some(code),
                    rgb: None,
                    space: Space::Bits4,
                }),
                8..=15 => Ok(Color {
                    code: Some(code),
                    rgb: None,
                    space: Space::Bits4,
                }),
                _ => Ok(Color {
                    code: Some(code),
                    rgb: None,
                    space: Space::Bits8,
                }),
            };
        }

        for (radix, regex) in &[
            // #ff00ff
            (16, "^#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})$"),
            // rgb(255, 0, 255)
            (10, r"^rgb\(\s*([0-9]+)\s*,\s*([0-9]+)\s*,\s*([0-9]+)\s*\)$"),
        ] {
            for capture in regex::Regex::new(regex).unwrap().captures_iter(&color) {
                let red = u8::from_str_radix(&capture[1], *radix).unwrap();
                let green = u8::from_str_radix(&capture[2], *radix).unwrap();
                let blue = u8::from_str_radix(&capture[3], *radix).unwrap();

                return Ok(Color {
                    code: None,
                    rgb: Some((red, green, blue)),
                    space: Space::Bits24,
                });
            }
        }

        Err(format!("Invalid color: {}", original))
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    use super::Space;

    #[test]
    fn new_default() {
        let color = Color {
            code: None,
            rgb: None,
            space: Space::Bits2,
        };
        assert_eq!(Color::new("default"), Ok(color));
        assert_eq!(color.sgr(false), Ok(vec![49]));
        assert_eq!(color.sgr(true), Ok(vec![39]));
    }

    #[test]
    fn new_black() {
        let color = Color {
            code: Some(0),
            rgb: None,
            space: Space::Bits4,
        };
        assert_eq!(Color::new("black"), Ok(color));
        assert_eq!(color.sgr(false), Ok(vec![40]));
        assert_eq!(color.sgr(true), Ok(vec![30]));
    }

    #[test]
    fn new_bright_black() {
        let color = Color {
            code: Some(8),
            rgb: None,
            space: Space::Bits4,
        };
        assert_eq!(Color::new("bright_black"), Ok(color));
        assert_eq!(color.sgr(false), Ok(vec![100]));
        assert_eq!(color.sgr(true), Ok(vec![90]));
    }

    #[test]
    fn new_grey0() {
        let color = Color {
            code: Some(16),
            rgb: None,
            space: Space::Bits8,
        };
        assert_eq!(Color::new("grey0"), Ok(color));
        assert_eq!(color.sgr(false), Ok(vec![48, 5, 16]));
        assert_eq!(color.sgr(true), Ok(vec![38, 5, 16]));
    }

    #[test]
    fn new_rgb() {
        let color = Color {
            code: None,
            rgb: Some((255, 128, 0)),
            space: Space::Bits24,
        };
        assert_eq!(Color::new("#ff8000"), Ok(color));
        assert_eq!(color.sgr(false), Ok(vec![48, 2, 255, 128, 0]));
        assert_eq!(color.sgr(true), Ok(vec![38, 2, 255, 128, 0]));
    }
}
