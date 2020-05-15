use crate::game::map::{Map, SpawnType, CellType};
use crate::game::entities::Player;
use std::time::Duration;
use crate::core::vector::Vector;

pub mod entities;
pub mod map;

pub struct GameState {
    pub map: Map,
    pub player: Player,
    pub map_zoom_level: u16,
    _private: (),
}

pub struct ActiveInputs {
    pub exit_game: bool,
    pub turn_left: bool,
    pub turn_right: bool,
    pub move_forward: bool,
    pub move_back: bool,
    pub zoom_in: bool,
    pub zoom_out: bool
}

#[derive(Debug)]
enum Side { Right, Left, Top, Bottom }

impl GameState {
    pub fn new() -> Self {
        let map = Map::new();

        let first_spawn_cell = map.spawns
            .iter()
            .filter(|x| x.entity == SpawnType::Player)
            .nth(0)
            .unwrap();

        let half_cell_length = map.units_per_cell / 2;
        let initial_pos = Vector {
            x: ((map.units_per_cell * first_spawn_cell.col as u32) + half_cell_length) as f32,
            y: ((map.units_per_cell * first_spawn_cell.row as u32) + half_cell_length) as f32,
        };

        let player = Player::new(initial_pos);

        GameState {
            map,
            player,
            map_zoom_level: 1,
            _private: (),
        }
    }

    pub fn tick(&mut self, time_since_last_frame: &Duration, inputs: &ActiveInputs) {
        if inputs.zoom_in {
            self.map_zoom_level += 1;
        }

        if inputs.zoom_out && self.map_zoom_level > 1 {
            self.map_zoom_level -= 1;
        }

        let turn_amount = self.player.turn_speed * time_since_last_frame.as_secs_f32();
        if inputs.turn_left {
            self.player.facing = self.player.facing - turn_amount;
        }

        if inputs.turn_right {
            self.player.facing = self.player.facing + turn_amount;
        }

        let mut velocity = Vector { x: 0_f32, y: 0_f32 };
        if inputs.move_forward {
            velocity = velocity + Vector {
                x: self.player.facing.0.cos() * self.player.move_speed,
                y: self.player.facing.0.sin() * self.player.move_speed,
            };
        }

        if inputs.move_back {
            velocity = velocity + Vector {
                x: self.player.facing.0.cos() * self.player.move_speed * -1_f32,
                y: self.player.facing.0.sin() * self.player.move_speed * -1_f32,
            };
        }

        self.player.position = self.player.position + (velocity * time_since_last_frame.as_secs_f32());

        self.apply_wall_collision(Side::Right);
        self.apply_wall_collision(Side::Bottom);
        self.apply_wall_collision(Side::Left);
        self.apply_wall_collision(Side::Top);
    }

    fn apply_wall_collision(&mut self, side: Side) {
        let (test_x, test_y) = match side {
            Side::Right => (self.player.position.x + self.player.collision_size as f32 / 2.0, self.player.position.y),
            Side::Left => (self.player.position.x - self.player.collision_size as f32 / 2.0, self.player.position.y),
            Side::Top => (self.player.position.x, self.player.position.y - self.player.collision_size as f32 / 2.0),
            Side::Bottom => (self.player.position.x, self.player.position.y + self.player.collision_size as f32 / 2.0),
        };

        let row = test_y as u32 / self.map.units_per_cell;
        let col = test_x as u32 / self.map.units_per_cell;

        let should_move = match self.map.cell_at(row as usize, col as usize) {
            Some(CellType::Empty) => false,
            _ => true,
        };

        match (should_move, side) {
            (false, _) => (),
            (true, Side::Right) => {self.player.position.x = (col * self.map.units_per_cell - self.player.collision_size as u32 / 2) as f32;},
            (true, Side::Left) => {self.player.position.x = ((col + 1) * self.map.units_per_cell + self.player.collision_size as u32 / 2) as f32;},
            (true, Side::Top) => {self.player.position.y = ((row + 1) * self.map.units_per_cell + self.player.collision_size as u32 / 2) as f32;},
            (true, Side::Bottom) => {self.player.position.y = (row * self.map.units_per_cell - self.player.collision_size as u32 / 2) as f32;},
        }

        if should_move {

        }
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
            zoom_in: false,
            zoom_out: false,
        }
    }
}
