pub fn hsl_to_rgbn(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    let c = s * (1.0 - (2.0 * l - 1.0).abs());
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (r + m, g + m, b + m)
}

pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let (rn, gn, bn) = hsl_to_rgbn(h, s, l);
    rgbn_to_rgb(rn, gn, bn)
}

pub fn rgb_to_rgbn(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
}

pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let (rn, gn, bn) = rgb_to_rgbn(r, g, b);
    rgbn_to_hsl(rn, gn, bn)
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

pub fn rgbn_to_rgb(r: f64, g: f64, b: f64) -> (u8, u8, u8) {
    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

#[cfg(test)]
mod test_rgb_to_rgbn {
    use super::{rgb_to_rgbn, rgbn_to_rgb};

    #[test]
    fn _255_102_0() {
        assert_eq!(rgb_to_rgbn(255, 102, 0), (1.0, 0.4, 0.0));
        assert_eq!(rgbn_to_rgb(1.0, 0.4, 0.0), (255, 102, 0));
    }
}

#[cfg(test)]
mod test_rgb_to_hsl {
    use super::{hsl_to_rgb, rgb_to_hsl};

    #[test]
    fn _10_04_00() {
        assert_eq!(rgb_to_hsl(255, 102, 0), (24.0, 1.0, 0.5));
        assert_eq!(hsl_to_rgb(24.0, 1.0, 0.5), (255, 102, 0));
    }
}
