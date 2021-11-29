pub struct ColorTriplet {
    red: u8,
    green: u8,
    blue: u8,
}

impl ColorTriplet {
    pub fn hex(&self) -> String {
        format!("#{:0>2X}{:0>2X}{:0>2X}", self.red, self.green, self.blue)
    }
    pub fn normalized(&self) -> (f64, f64, f64) {
        (
            self.red as f64 / 255.0,
            self.green as f64 / 255.0,
            self.blue as f64 / 255.0,
        )
    }
    pub fn rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex() {
        let color = ColorTriplet {
            red: 255,
            green: 128,
            blue: 0,
        };
        assert_eq!(color.hex(), "#FF8000");
    }
    #[test]
    fn normalized() {
        let color = ColorTriplet {
            red: 255,
            green: 102,
            blue: 0,
        };
        assert_eq!(color.normalized(), (1.0, 0.4, 0.0))
    }
    #[test]
    fn rgb() {
        let color = ColorTriplet {
            red: 255,
            green: 128,
            blue: 0,
        };
        assert_eq!(color.rgb(), "rgb(255, 128, 0)");
    }
}
