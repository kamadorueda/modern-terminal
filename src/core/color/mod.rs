pub mod codes;
pub mod model;
pub mod storage;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    code:    Option<u8>,
    rgb:     Option<(u8, u8, u8)>,
    storage: crate::core::color::storage::Storage,
}

impl Color {
    pub fn new(color: &str) -> Result<Color, &str> {
        let original = color;
        let color = original.trim().to_lowercase();

        if color == "default" {
            return Ok(Color {
                code:    None,
                rgb:     None,
                storage: crate::core::color::storage::Storage::Bits1,
            });
        }

        if let Some(code) = crate::core::color::codes::from_name(&color) {
            return match code {
                0..=7 => Ok(Color {
                    code:    Some(code),
                    rgb:     None,
                    storage: crate::core::color::storage::Storage::Bits4,
                }),
                8..=15 => Ok(Color {
                    code:    Some(code),
                    rgb:     None,
                    storage: crate::core::color::storage::Storage::Bits4,
                }),
                _ => Ok(Color {
                    code:    Some(code),
                    rgb:     None,
                    storage: crate::core::color::storage::Storage::Bits8,
                }),
            };
        }

        for (radix, regex) in &[
            // #ff00ff
            (16, "^#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})$"),
            // rgb(255, 0, 255)
            (10, r"^rgb\(\s*([0-9]+)\s*,\s*([0-9]+)\s*,\s*([0-9]+)\s*\)$"),
        ] {
            for capture in
                regex::Regex::new(regex).unwrap().captures_iter(&color)
            {
                let red = u8::from_str_radix(&capture[1], *radix).unwrap();
                let green = u8::from_str_radix(&capture[2], *radix).unwrap();
                let blue = u8::from_str_radix(&capture[3], *radix).unwrap();

                return Ok(Color {
                    code:    None,
                    rgb:     Some((red, green, blue)),
                    storage: crate::core::color::storage::Storage::Bits24,
                });
            }
        }

        Err(original)
    }

    pub fn to_storage(
        &self,
        storage: crate::core::color::storage::Storage,
    ) -> Option<Color> {
        match (self.storage, storage) {
            (_, crate::core::color::storage::Storage::Bits1) => Some(Color {
                code:    None,
                rgb:     None,
                storage: crate::core::color::storage::Storage::Bits1,
            }),

            (
                crate::core::color::storage::Storage::Bits24,
                crate::core::color::storage::Storage::Bits8,
            ) => match self.rgb {
                Some((r, g, b)) => {
                    let (rn, gn, bn) =
                        crate::core::color::model::rgb_to_rgbn(r, g, b);
                    let (_, s, l) =
                        crate::core::color::model::rgbn_to_hsl(rn, gn, bn);

                    Some(Color {
                        code:    Some(if s < 0.08 {
                            match (25.0 * l).round() as u8 {
                                0 => 16,
                                25 => 231,
                                step => 231 + step,
                            }
                        }
                        else {
                            (16.0
                                + (5.0 * rn).round() * 36.0
                                + (5.0 * gn).round() * 6.0
                                + (5.0 * bn).round())
                                as u8
                        }),
                        rgb:     None,
                        storage: crate::core::color::storage::Storage::Bits8,
                    })
                },
                None => None,
            },
            (
                crate::core::color::storage::Storage::Bits24,
                crate::core::color::storage::Storage::Bits4,
            ) => None,
            (crate::core::color::storage::Storage::Bits24, _) => Some(*self),

            (
                crate::core::color::storage::Storage::Bits8,
                crate::core::color::storage::Storage::Bits4,
            ) => None,
            (crate::core::color::storage::Storage::Bits8, _) => Some(*self),

            (crate::core::color::storage::Storage::Bits4, _) => Some(*self),

            (crate::core::color::storage::Storage::Bits1, _) => Some(*self),
        }
    }

    pub fn ansi_sgr(
        &self,
        foreground: bool,
    ) -> Result<Vec<u8>, &Color> {
        match self {
            Color {
                storage: crate::core::color::storage::Storage::Bits1,
                ..
            } => Ok(vec![if foreground { 39 } else { 49 }]),

            Color {
                code: Some(code),
                storage: crate::core::color::storage::Storage::Bits4,
                ..
            } => match code {
                0..=7 => Ok(vec![code + if foreground { 30 } else { 40 }]),
                8..=15 => Ok(vec![code + if foreground { 82 } else { 92 }]),
                16..=255 => Err(self),
            },

            Color {
                code: Some(code),
                storage: crate::core::color::storage::Storage::Bits8,
                ..
            } => Ok(vec![if foreground { 38 } else { 48 }, 5, *code]),

            Color {
                rgb: Some((r, g, b)),
                storage: crate::core::color::storage::Storage::Bits24,
                ..
            } => Ok(vec![if foreground { 38 } else { 48 }, 2, *r, *g, *b]),

            _ => Err(self),
        }
    }
}

#[cfg(test)]
mod test_color {
    use super::Color;

    #[test]
    fn default() {
        let color = Color::new("default").unwrap();
        let expected = Color {
            code:    None,
            rgb:     None,
            storage: crate::core::color::storage::Storage::Bits1,
        };
        assert_eq!(color, expected);

        assert_eq!(color.ansi_sgr(false), Ok(vec![49]));
        assert_eq!(color.ansi_sgr(true), Ok(vec![39]));
    }

    #[test]
    fn black() {
        let color = Color::new("black").unwrap();
        let expected = Color {
            code:    Some(0),
            rgb:     None,
            storage: crate::core::color::storage::Storage::Bits4,
        };
        assert_eq!(color, expected);

        assert_eq!(color.ansi_sgr(false), Ok(vec![40]));
        assert_eq!(color.ansi_sgr(true), Ok(vec![30]));
    }

    #[test]
    fn bright_black() {
        let color = Color::new("bright_black").unwrap();
        let expected = Color {
            code:    Some(8),
            rgb:     None,
            storage: crate::core::color::storage::Storage::Bits4,
        };
        assert_eq!(color, expected);

        assert_eq!(color.ansi_sgr(false), Ok(vec![100]));
        assert_eq!(color.ansi_sgr(true), Ok(vec![90]));
    }

    #[test]
    fn grey0() {
        let color = Color::new("grey0").unwrap();
        let expected = Color {
            code:    Some(16),
            rgb:     None,
            storage: crate::core::color::storage::Storage::Bits8,
        };
        assert_eq!(color, expected);

        assert_eq!(color.ansi_sgr(false), Ok(vec![48, 5, 16]));
        assert_eq!(color.ansi_sgr(true), Ok(vec![38, 5, 16]));
    }

    #[test]
    fn ff8000() {
        let color = Color::new("#ff8000").unwrap();
        let expected = Color {
            code:    None,
            rgb:     Some((255, 128, 0)),
            storage: crate::core::color::storage::Storage::Bits24,
        };
        assert_eq!(color, expected);

        assert_eq!(color.ansi_sgr(false), Ok(vec![48, 2, 255, 128, 0]));
        assert_eq!(color.ansi_sgr(true), Ok(vec![38, 2, 255, 128, 0]));

        let color_8 = Color {
            code:    Some(214),
            rgb:     None,
            storage: crate::core::color::storage::Storage::Bits8,
        };
        assert_eq!(
            color.to_storage(crate::core::color::storage::Storage::Bits8),
            Some(color_8)
        );
    }
}
