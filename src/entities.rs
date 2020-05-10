use crate::core::{Vector, Radians};

pub struct Player {
    pub position: Vector,
    pub facing: Radians,
    pub collision_size: u16,
    pub velocity: Vector,
    _private: (), // prevent construction
}

impl Player {
    pub fn new(initial_pos: Vector) -> Self {
        Player {
            position: initial_pos,
            facing: Radians(0_f32),
            collision_size: 10,
            velocity: Vector {x: 0f32, y: 0f32},
            _private: (),
        }
    }
}