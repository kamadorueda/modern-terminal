pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    pub fn hex(&self) -> String {
        format!("#{:0>2X}{:0>2X}{:0>2X}", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_hex() {
        let rgb = RGB {
            red: 255,
            green: 128,
            blue: 0,
        };
        assert_eq!(rgb.hex(), "#FF8000");
    }
}
