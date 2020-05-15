use std::ops::{Add, Sub, Mul, Div};
use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub struct Radians(pub f32);

impl Radians {
    pub fn clamp(self) -> Self {
        const COMPLETE_CIRCLE: f32 = 2_f32 * PI;

        if self.0 >= COMPLETE_CIRCLE {
            Radians(self.0 - (COMPLETE_CIRCLE)).clamp()
        } else if self.0 < 0_f32 {
            Radians(self.0 + (COMPLETE_CIRCLE)).clamp()
        } else {
            Radians(self.0)
        }
    }
}

impl Add for Radians {
    type Output = Radians;

    fn add(self, other: Radians) -> Radians {
        Radians(self.0 + other.0).clamp()
    }
}

impl Add<f32> for Radians {
    type Output = Radians;

    fn add(self, other: f32) -> Radians {
        Radians(self.0 + other).clamp()
    }
}

impl Sub for Radians {
    type Output = Radians;

    fn sub(self, other: Radians) -> Radians {
        Radians(self.0 - other.0).clamp()
    }
}

impl Sub<f32> for Radians {
    type Output = Radians;

    fn sub(self, other: f32) -> Radians {
        Radians(self.0 - other).clamp()
    }
}

impl Mul<f32> for Radians {
    type Output = Radians;

    fn mul(self, other: f32) -> Radians {
        Radians(self.0 * other).clamp()
    }
}

impl Div<f32> for Radians {
    type Output = Radians;

    fn div(self, other: f32) -> Radians {
        Radians(self.0 / other).clamp()
    }
}