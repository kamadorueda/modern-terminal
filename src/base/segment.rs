pub struct Segment {
    space: crate::base::color::storage::Storage,
    style: crate::base::style::Style,
    text: String,
}

impl Segment {
    pub fn new(space: crate::base::color::storage::Storage) -> Segment {
        Segment {
            space,
            style: crate::base::style::Style::new(),
            text: String::new(),
        }
    }
}

impl Segment {
    pub fn a(&mut self, text: &str) -> &mut Segment {
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

impl Segment {
    pub fn bg(&mut self, color: &str) -> &mut Segment {
        self.style.background(color);
        self
    }

    pub fn no_bg(&mut self) -> &mut Segment {
        self.style.reset_background();
        self
    }
}

impl Segment {
    pub fn b(&mut self) -> &mut Segment {
        self.style.bold();
        self
    }

    pub fn not_b(&mut self) -> &mut Segment {
        self.apply_style();
        self.style.not_bold();
        self
    }
}

impl Segment {
    pub fn d(&mut self) -> &mut Segment {
        self.style.dim();
        self
    }

    pub fn not_d(&mut self) -> &mut Segment {
        self.apply_style();
        self.style.not_dim();
        self
    }
}

impl Segment {
    pub fn fg(&mut self, color: &str) -> &mut Segment {
        self.style.foreground(color);
        self
    }

    pub fn no_fg(&mut self) -> &mut Segment {
        self.style.reset_foreground();
        self
    }
}

#[cfg(test)]
mod test_segment {
    use super::Segment;

    #[test]
    fn default() {
        assert_eq!(
            Segment::new(crate::base::color::storage::Storage::Bits24).to_string(),
            ""
        );
    }

    #[test]
    fn bg() {
        assert_eq!(
            Segment::new(crate::base::color::storage::Storage::Bits24)
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
            Segment::new(crate::base::color::storage::Storage::Bits24)
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
            Segment::new(crate::base::color::storage::Storage::Bits24)
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
            Segment::new(crate::base::color::storage::Storage::Bits24)
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
