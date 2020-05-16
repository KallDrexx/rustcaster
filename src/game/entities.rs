use crate::core::radians::Radians;
use crate::core::vector::Vector;

pub struct Player {
    pub position: Vector,
    pub facing: Radians,
    pub collision_size: u16,
    pub velocity: Vector,
    pub turn_speed: f32,
    pub move_speed: f32,
}

impl Player {
    pub fn new(initial_pos: Vector) -> Self {
        Player {
            position: initial_pos,
            facing: Radians(0_f32),
            collision_size: 2,
            velocity: Vector {x: 0f32, y: 0f32},
            turn_speed: 5_f32,
            move_speed: 10_f32,
        }
    }
}