use crate::entity::Entity;

#[derive(Copy, Clone)]
pub enum Cell {
    Empty,
    Wall,
    Goal,
}

const SIZE: usize = 10;
pub struct Map {
    pub cells: [[Cell; SIZE]; SIZE],
    pub entity: Entity,
    pub goal: (usize, usize),
}

pub trait MapGenerator {
    fn generate(&self) -> Map;
}

pub struct StaticMapGenerator;

impl MapGenerator for StaticMapGenerator {
    fn generate(&self) -> Map {
        let mut cells = [[Cell::Empty; SIZE]; SIZE];

        for x in 0..SIZE {
            for y in 0..SIZE {
                if x == 0 || y == 0 || x == SIZE - 1 || y == SIZE - 1 {
                    cells[y][x] = Cell::Wall;
                }
            }
        }

        cells[8][7] = Cell::Goal;

        Map {
            cells,
            entity: Entity::new((1, 1)),
            goal: (7, 8),
        }
    }
}
