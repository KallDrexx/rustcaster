use crate::game::map::Map;
use crate::game::entities::Player;
use crate::core::Vector;
use std::time::Duration;

pub mod entities;
pub mod map;

pub struct GameState {
    pub map: Map,
    pub player: Player,
    _private: (),
}

pub struct ActiveInputs {
    pub exit_game: bool,
    pub turn_left: bool,
    pub turn_right: bool,
    pub move_forward: bool,
    pub move_back: bool,
}

impl GameState {
    pub fn new() -> Self {
        let map = Map::new();

        // Start the player in the middle of the 2nd diagonal cell
        let pos_value = map.units_per_cell as f32 * 1.5_f32;
        let initial_pos = Vector {x: pos_value, y: pos_value};
        let player = Player::new(initial_pos);

        GameState {
            map,
            player,
            _private: (),
        }
    }

    pub fn tick(&mut self, _time_since_last_frame: &Duration, _inputs: ActiveInputs) {

    }
}

impl ActiveInputs {
    pub fn new() -> Self {
        ActiveInputs {
            exit_game: false,
            turn_right: false,
            turn_left: false,
            move_back: false,
            move_forward: false,
        }
    }
}
