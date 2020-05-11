use crate::core::radians::Radians;
use std::f32::consts::PI;

pub struct Degrees(pub f32);

impl Degrees {
    pub fn to_radians(&self) -> Radians {
        Radians(self.0 * (PI / 180_f32))
    }
}