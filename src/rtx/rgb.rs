use super::Vector;
use crate::rtx::Interval;
use std::fmt;

pub type RGB = Vector<3>;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    return 0.0;
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = linear_to_gamma(self.components[0]);
        let g = linear_to_gamma(self.components[1]);
        let b = linear_to_gamma(self.components[2]);

        // Translate the [0,1] component values to the byte range [0,255].
        let intensity = Interval::new(0.0, 0.999);
        let r_byte = (intensity.clamp(r) * 256.0) as u8;
        let g_byte = (intensity.clamp(g) * 256.0) as u8;
        let b_byte = (intensity.clamp(b) * 256.0) as u8;

        // Write the formatted string to the formatter.
        write!(f, "{} {} {}", r_byte, g_byte, b_byte)
    }
}
