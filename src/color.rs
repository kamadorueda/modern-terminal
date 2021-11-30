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
    pub fn new(color: &str) -> Result<Color, &str> {
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

        Err(original)
    }

    pub fn to_space(&self, space: Space) -> Option<Color> {
        match (self.space, space) {
            (_, Space::Bits2) => Some(Color {
                code: None,
                rgb: None,
                space: Space::Bits2,
            }),

            (Space::Bits24, Space::Bits8) => match self.rgb {
                Some((r, g, b)) => {
                    let (rn, gn, bn) = crate::color_systems::rgb_to_rgbn(r, g, b);
                    let (_, s, l) = crate::color_systems::rgbn_to_hsl(rn, gn, bn);

                    Some(Color {
                        code: Some(if s < 0.08 {
                            match (25.0 * l).round() as u8 {
                                0 => 16,
                                25 => 231,
                                step => 231 + step,
                            }
                        } else {
                            (16.0
                                + (5.0 * rn).round() * 36.0
                                + (5.0 * gn).round() * 6.0
                                + (5.0 * bn).round()) as u8
                        }),
                        rgb: None,
                        space: Space::Bits8,
                    })
                }
                None => None,
            },
            (Space::Bits24, Space::Bits4) => None,
            (Space::Bits24, _) => Some(*self),

            (Space::Bits8, Space::Bits4) => None,
            (Space::Bits8, _) => Some(*self),

            (Space::Bits4, _) => Some(*self),

            (Space::Bits2, _) => Some(*self),
        }
    }

    pub fn ansi_sgr(&self, foreground: bool) -> Result<Vec<u8>, &Color> {
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
                16..=255 => Err(self),
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

            _ => Err(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    use super::Space;

    #[test]
    fn default() {
        let color = Color::new("default").unwrap();
        let expected = Color {
            code: None,
            rgb: None,
            space: Space::Bits2,
        };
        assert_eq!(color, expected);

        assert_eq!(color.ansi_sgr(false), Ok(vec![49]));
        assert_eq!(color.ansi_sgr(true), Ok(vec![39]));
    }

    #[test]
    fn black() {
        let color = Color::new("black").unwrap();
        let expected = Color {
            code: Some(0),
            rgb: None,
            space: Space::Bits4,
        };
        assert_eq!(color, expected);

        assert_eq!(color.ansi_sgr(false), Ok(vec![40]));
        assert_eq!(color.ansi_sgr(true), Ok(vec![30]));
    }

    #[test]
    fn bright_black() {
        let color = Color::new("bright_black").unwrap();
        let expected = Color {
            code: Some(8),
            rgb: None,
            space: Space::Bits4,
        };
        assert_eq!(color, expected);

        assert_eq!(color.ansi_sgr(false), Ok(vec![100]));
        assert_eq!(color.ansi_sgr(true), Ok(vec![90]));
    }

    #[test]
    fn grey0() {
        let color = Color::new("grey0").unwrap();
        let expected = Color {
            code: Some(16),
            rgb: None,
            space: Space::Bits8,
        };
        assert_eq!(color, expected);

        assert_eq!(color.ansi_sgr(false), Ok(vec![48, 5, 16]));
        assert_eq!(color.ansi_sgr(true), Ok(vec![38, 5, 16]));
    }

    #[test]
    fn ff8000() {
        let color = Color::new("#ff8000").unwrap();
        let expected = Color {
            code: None,
            rgb: Some((255, 128, 0)),
            space: Space::Bits24,
        };
        assert_eq!(color, expected);

        assert_eq!(color.ansi_sgr(false), Ok(vec![48, 2, 255, 128, 0]));
        assert_eq!(color.ansi_sgr(true), Ok(vec![38, 2, 255, 128, 0]));

        let color_8 = Color {
            code: Some(214),
            rgb: None,
            space: Space::Bits8,
        };
        assert_eq!(color.to_space(Space::Bits8), Some(color_8));
    }
}
