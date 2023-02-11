use crate::{map::Map, model::Model};

pub struct Entity {
    pub position: (usize, usize),
}

impl Entity {
    pub fn new(pos: (usize, usize)) -> Entity {
        Entity { position: pos }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

pub trait ControlledEntity {
    fn get_move(&mut self, map: &Map) -> Option<Move>;
}

pub struct AIEntity(Model);

impl AIEntity {
    pub fn new(model: Model) -> AIEntity {
        AIEntity(model)
    }
}

impl ControlledEntity for AIEntity {
    fn get_move(&mut self, _map: &Map) -> Option<Move> {
        // TODO: derive inputs from the map
        let input = vec![0.0, 0.0, 0.0, 0.0];

        let output = self.0.forward(input);

        let mut greatest_index = 0;
        let mut greatest_value = 0.0;

        for i in 0..output.len() {
            if output[i] > greatest_value {
                greatest_index = i;
                greatest_value = output[i];
            }
        }

        Some(match greatest_index {
            0 => Move::Up,
            1 => Move::Down,
            2 => Move::Left,
            3 => Move::Right,
            _ => panic!(),
        })
    }
}
