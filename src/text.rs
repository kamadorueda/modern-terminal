pub struct Text {
    space: crate::color::Space,
    style: crate::style::Style,
    text: String,
}

impl Text {
    pub fn new(space: crate::color::Space) -> Text {
        Text {
            space,
            style: crate::style::Style::new(),
            text: String::new(),
        }
    }
}

impl Text {
    pub fn a(&mut self, text: &str) -> &mut Text {
        if !text.is_empty() {
            self.apply_style();
            self.text.push_str(text);
        }

        self
    }

    fn apply_style(&mut self) {
        self.text.push_str(&self.style.ansi_escape_code(self.space));
    }

    pub fn to_string(&self) -> String {
        if self.text.is_empty() {
            String::new()
        } else {
            format!("\u{1b}[0m{}\u{1b}[0m", self.text)
        }
    }
}

impl Text {
    pub fn bg(&mut self, color: &str) -> &mut Text {
        self.style.background(color);
        self
    }

    pub fn no_bg(&mut self) -> &mut Text {
        self.style.reset_background();
        self
    }
}

impl Text {
    pub fn b(&mut self) -> &mut Text {
        self.style.bold();
        self
    }

    pub fn not_b(&mut self) -> &mut Text {
        self.apply_style();
        self.style.not_bold();
        self
    }
}

impl Text {
    pub fn d(&mut self) -> &mut Text {
        self.style.dim();
        self
    }

    pub fn not_d(&mut self) -> &mut Text {
        self.apply_style();
        self.style.not_dim();
        self
    }
}

impl Text {
    pub fn fg(&mut self, color: &str) -> &mut Text {
        self.style.foreground(color);
        self
    }

    pub fn no_fg(&mut self) -> &mut Text {
        self.style.reset_foreground();
        self
    }
}

#[cfg(test)]
mod test_text {
    use super::Text;

    #[test]
    fn default() {
        assert_eq!(Text::new(crate::color::Space::Bits24).to_string(), "");
    }

    #[test]
    fn bg() {
        assert_eq!(
            Text::new(crate::color::Space::Bits24)
                .bg("yellow")
                .a("a")
                .bg("red")
                .a("b")
                .no_bg()
                .a("c")
                .to_string(),
            "\u{1b}[0m\u{1b}[43ma\u{1b}[41mb\u{1b}[49mc\u{1b}[0m"
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            Text::new(crate::color::Space::Bits24)
                .b()
                .a("a")
                .not_b()
                .a("b")
                .b()
                .a("c")
                .to_string(),
            "\u{1b}[0m\u{1b}[1ma\u{1b}[1m\u{1b}[mb\u{1b}[1mc\u{1b}[0m"
        );
    }

    #[test]
    fn d() {
        assert_eq!(
            Text::new(crate::color::Space::Bits24)
                .d()
                .a("a")
                .not_d()
                .a("b")
                .d()
                .a("c")
                .to_string(),
            "\u{1b}[0m\u{1b}[2ma\u{1b}[2m\u{1b}[mb\u{1b}[2mc\u{1b}[0m"
        );
    }

    #[test]
    fn fg() {
        assert_eq!(
            Text::new(crate::color::Space::Bits24)
                .fg("yellow")
                .a("a")
                .fg("red")
                .a("b")
                .no_fg()
                .a("c")
                .to_string(),
            "\u{1b}[0m\u{1b}[33ma\u{1b}[31mb\u{1b}[39mc\u{1b}[0m"
        );
    }
}
