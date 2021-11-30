pub fn rgb_to_rgbn(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
}

pub fn rgbn_to_hsl(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let x_max = f64::max(r, f64::max(g, b));
    let x_min = f64::min(r, f64::min(g, b));
    let l = (x_max + x_min) / 2.0;
    let s = if l.abs() < f64::EPSILON || (l - 1.0).abs() < f64::EPSILON {
        0.0
    } else {
        (x_max - l) / f64::min(l, 1.0 - l)
    };
    let h = if (x_max - x_min).abs() < f64::EPSILON {
        dbg!(0.0)
    } else if (x_max - r).abs() < f64::EPSILON {
        dbg!(60.0 * (0.0 + (g - b) / (x_max - x_min)))
    } else if (x_max - g).abs() < f64::EPSILON {
        60.0 * (2.0 + (b - r) / (x_max - x_min))
    } else if (x_max - b).abs() < f64::EPSILON {
        60.0 * (4.0 + (r - g) / (x_max - x_min))
    } else {
        0.0
    };

    (h, s, l)
}

#[cfg(test)]
mod tests {
    use super::rgb_to_rgbn;
    use super::rgbn_to_hsl;

    #[test]
    fn test_rgb_to_rgbn() {
        assert_eq!(rgb_to_rgbn(255, 102, 0), (1.0, 0.4, 0.0))
    }

    #[test]
    fn test_rgbn_to_hsl() {
        assert_eq!(rgbn_to_hsl(1.0, 0.4, 0.0), (24.0, 1.0, 0.5))
    }
}
