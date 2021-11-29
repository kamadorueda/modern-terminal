#[derive(Debug, PartialEq)]
pub enum Space {
    Bits2,  // Terminal defaults
    Bits4,  // 4-bit (Standard)
    Bits8,  // 8-bit
    Bits24, // 24-bit (True Color)
}

#[derive(Debug, PartialEq)]
pub struct Color {
    code: Option<u8>,
    foreground: bool,
    rgb: Option<(u8, u8, u8)>,
    space: Space,
}

impl Color {
    pub fn new(color: &str, foreground: bool) -> Result<Color, String> {
        let original = color;
        let color = original.trim().to_lowercase();

        if color == "default" {
            return Ok(Color {
                code: None,
                foreground,
                rgb: None,
                space: Space::Bits2,
            });
        }

        if let Some(code) = Color::name_to_code(&color) {
            return match code {
                0..=7 => Ok(Color {
                    code: Some(code),
                    foreground,
                    rgb: None,
                    space: Space::Bits4,
                }),
                8..=15 => Ok(Color {
                    code: Some(code),
                    foreground,
                    rgb: None,
                    space: Space::Bits4,
                }),
                _ => Ok(Color {
                    code: Some(code),
                    foreground,
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
                    foreground,
                    rgb: Some((red, green, blue)),
                    space: Space::Bits24,
                });
            }
        }

        Err(format!("Invalid color: {}", original))
    }

    fn name_to_code(name: &str) -> Option<u8> {
        match name {
            "black" => Some(0),
            "red" => Some(1),
            "green" => Some(2),
            "yellow" => Some(3),
            "blue" => Some(4),
            "magenta" => Some(5),
            "cyan" => Some(6),
            "white" => Some(7),
            "bright_black" => Some(8),
            "bright_red" => Some(9),
            "bright_green" => Some(10),
            "bright_yellow" => Some(11),
            "bright_blue" => Some(12),
            "bright_magenta" => Some(13),
            "bright_cyan" => Some(14),
            "bright_white" => Some(15),
            "grey0" => Some(16),
            "navy_blue" => Some(17),
            "dark_blue" => Some(18),
            "blue3" => Some(20),
            "blue1" => Some(21),
            "dark_green" => Some(22),
            "deep_sky_blue4" => Some(25),
            "dodger_blue3" => Some(26),
            "dodger_blue2" => Some(27),
            "green4" => Some(28),
            "spring_green4" => Some(29),
            "turquoise4" => Some(30),
            "deep_sky_blue3" => Some(32),
            "dodger_blue1" => Some(33),
            "green3" => Some(40),
            "spring_green3" => Some(41),
            "dark_cyan" => Some(36),
            "light_sea_green" => Some(37),
            "deep_sky_blue2" => Some(38),
            "deep_sky_blue1" => Some(39),
            "spring_green2" => Some(47),
            "cyan3" => Some(43),
            "dark_turquoise" => Some(44),
            "turquoise2" => Some(45),
            "green1" => Some(46),
            "spring_green1" => Some(48),
            "medium_spring_green" => Some(49),
            "cyan2" => Some(50),
            "cyan1" => Some(51),
            "dark_red" => Some(88),
            "deep_pink4" => Some(125),
            "purple4" => Some(55),
            "purple3" => Some(56),
            "blue_violet" => Some(57),
            "orange4" => Some(94),
            "grey37" => Some(59),
            "medium_purple4" => Some(60),
            "slate_blue3" => Some(62),
            "royal_blue1" => Some(63),
            "chartreuse4" => Some(64),
            "dark_sea_green4" => Some(71),
            "pale_turquoise4" => Some(66),
            "steel_blue" => Some(67),
            "steel_blue3" => Some(68),
            "cornflower_blue" => Some(69),
            "chartreuse3" => Some(76),
            "cadet_blue" => Some(73),
            "sky_blue3" => Some(74),
            "steel_blue1" => Some(81),
            "pale_green3" => Some(114),
            "sea_green3" => Some(78),
            "aquamarine3" => Some(79),
            "medium_turquoise" => Some(80),
            "chartreuse2" => Some(112),
            "sea_green2" => Some(83),
            "sea_green1" => Some(85),
            "aquamarine1" => Some(122),
            "dark_slate_gray2" => Some(87),
            "dark_magenta" => Some(91),
            "dark_violet" => Some(128),
            "purple" => Some(129),
            "light_pink4" => Some(95),
            "plum4" => Some(96),
            "medium_purple3" => Some(98),
            "slate_blue1" => Some(99),
            "yellow4" => Some(106),
            "wheat4" => Some(101),
            "grey53" => Some(102),
            "light_slate_grey" => Some(103),
            "medium_purple" => Some(104),
            "light_slate_blue" => Some(105),
            "dark_olive_green3" => Some(149),
            "dark_sea_green" => Some(108),
            "light_sky_blue3" => Some(110),
            "sky_blue2" => Some(111),
            "dark_sea_green3" => Some(150),
            "dark_slate_gray3" => Some(116),
            "sky_blue1" => Some(117),
            "chartreuse1" => Some(118),
            "light_green" => Some(120),
            "pale_green1" => Some(156),
            "dark_slate_gray1" => Some(123),
            "red3" => Some(160),
            "medium_violet_red" => Some(126),
            "magenta3" => Some(164),
            "dark_orange3" => Some(166),
            "indian_red" => Some(167),
            "hot_pink3" => Some(168),
            "medium_orchid3" => Some(133),
            "medium_orchid" => Some(134),
            "medium_purple2" => Some(140),
            "dark_goldenrod" => Some(136),
            "light_salmon3" => Some(173),
            "rosy_brown" => Some(138),
            "grey63" => Some(139),
            "medium_purple1" => Some(141),
            "gold3" => Some(178),
            "dark_khaki" => Some(143),
            "navajo_white3" => Some(144),
            "grey69" => Some(145),
            "light_steel_blue3" => Some(146),
            "light_steel_blue" => Some(147),
            "yellow3" => Some(184),
            "dark_sea_green2" => Some(157),
            "light_cyan3" => Some(152),
            "light_sky_blue1" => Some(153),
            "green_yellow" => Some(154),
            "dark_olive_green2" => Some(155),
            "dark_sea_green1" => Some(193),
            "pale_turquoise1" => Some(159),
            "deep_pink3" => Some(162),
            "magenta2" => Some(200),
            "hot_pink2" => Some(169),
            "orchid" => Some(170),
            "medium_orchid1" => Some(207),
            "orange3" => Some(172),
            "light_pink3" => Some(174),
            "pink3" => Some(175),
            "plum3" => Some(176),
            "violet" => Some(177),
            "light_goldenrod3" => Some(179),
            "tan" => Some(180),
            "misty_rose3" => Some(181),
            "thistle3" => Some(182),
            "plum2" => Some(183),
            "khaki3" => Some(185),
            "light_goldenrod2" => Some(222),
            "light_yellow3" => Some(187),
            "grey84" => Some(188),
            "light_steel_blue1" => Some(189),
            "yellow2" => Some(190),
            "dark_olive_green1" => Some(192),
            "honeydew2" => Some(194),
            "light_cyan1" => Some(195),
            "red1" => Some(196),
            "deep_pink2" => Some(197),
            "deep_pink1" => Some(199),
            "magenta1" => Some(201),
            "orange_red1" => Some(202),
            "indian_red1" => Some(204),
            "hot_pink" => Some(206),
            "dark_orange" => Some(208),
            "salmon1" => Some(209),
            "light_coral" => Some(210),
            "pale_violet_red1" => Some(211),
            "orchid2" => Some(212),
            "orchid1" => Some(213),
            "orange1" => Some(214),
            "sandy_brown" => Some(215),
            "light_salmon1" => Some(216),
            "light_pink1" => Some(217),
            "pink1" => Some(218),
            "plum1" => Some(219),
            "gold1" => Some(220),
            "navajo_white1" => Some(223),
            "misty_rose1" => Some(224),
            "thistle1" => Some(225),
            "yellow1" => Some(226),
            "light_goldenrod1" => Some(227),
            "khaki1" => Some(228),
            "wheat1" => Some(229),
            "cornsilk1" => Some(230),
            "grey100" => Some(231),
            "grey3" => Some(232),
            "grey7" => Some(233),
            "grey11" => Some(234),
            "grey15" => Some(235),
            "grey19" => Some(236),
            "grey23" => Some(237),
            "grey27" => Some(238),
            "grey30" => Some(239),
            "grey35" => Some(240),
            "grey39" => Some(241),
            "grey42" => Some(242),
            "grey46" => Some(243),
            "grey50" => Some(244),
            "grey54" => Some(245),
            "grey58" => Some(246),
            "grey62" => Some(247),
            "grey66" => Some(248),
            "grey70" => Some(249),
            "grey74" => Some(250),
            "grey78" => Some(251),
            "grey82" => Some(252),
            "grey85" => Some(253),
            "grey89" => Some(254),
            "grey93" => Some(255),
            _ => None,
        }
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
            foreground: false,
            rgb: None,
            space: Space::Bits2,
        };
        assert_eq!(Color::new("default", false), Ok(color));

        let color = Color {
            code: None,
            foreground: true,
            rgb: None,
            space: Space::Bits2,
        };
        assert_eq!(Color::new("default", true), Ok(color));
    }

    #[test]
    fn new_black() {
        let color = Color {
            code: Some(0),
            foreground: false,
            rgb: None,
            space: Space::Bits4,
        };
        assert_eq!(Color::new("black", false), Ok(color));

        let color = Color {
            code: Some(0),
            foreground: true,
            rgb: None,
            space: Space::Bits4,
        };
        assert_eq!(Color::new("black", true), Ok(color));
    }

    #[test]
    fn new_bright_black() {
        let color = Color {
            code: Some(8),
            foreground: false,
            rgb: None,
            space: Space::Bits4,
        };
        assert_eq!(Color::new("bright_black", false), Ok(color));

        let color = Color {
            code: Some(8),
            foreground: true,
            rgb: None,
            space: Space::Bits4,
        };
        assert_eq!(Color::new("bright_black", true), Ok(color));
    }

    #[test]
    fn new_grey0() {
        let color = Color {
            code: Some(16),
            foreground: false,
            rgb: None,
            space: Space::Bits8,
        };
        assert_eq!(Color::new("grey0", false), Ok(color));

        let color = Color {
            code: Some(16),
            foreground: true,
            rgb: None,
            space: Space::Bits8,
        };
        assert_eq!(Color::new("grey0", true), Ok(color));
    }

    #[test]
    fn new_rgb() {
        let color = Color {
            code: None,
            foreground: false,
            rgb: Some((255, 128, 0)),
            space: Space::Bits24,
        };
        assert_eq!(Color::new("#ff8000", false), Ok(color));

        let color = Color {
            code: None,
            foreground: true,
            rgb: Some((255, 128, 0)),
            space: Space::Bits24,
        };
        assert_eq!(Color::new("rgb(255, 128, 0)", true), Ok(color));
    }
}
