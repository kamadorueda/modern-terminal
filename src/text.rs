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
    pub fn append(&mut self, text: &str) -> &mut Text {
        self.text.push_str(text);
        self
    }

    pub fn render(&self) -> String {
        format!("\u{1b}[0m{}\u{1b}[0m", self.text)
    }
}

impl Text {
    pub fn foreground(&mut self, color: &str) -> &mut Text {
        self.style.foreground(color);
        self.append(&self.style.ansi_escape_code(self.space))
    }
}

#[cfg(test)]
mod tests {
    use super::Text;

    #[test]
    fn test() {
        assert_eq!(
            Text::new(crate::color::Space::Bits24)
                .foreground("yellow")
                .append("Hello")
                .foreground("default")
                .append(", ")
                .foreground("bright_yellow")
                .append("World!")
                .render(),
            "\u{1b}[0m\u{1b}[33mHello\u{1b}[39m, \u{1b}[93mWorld!\u{1b}[0m"
        );
    }
}
