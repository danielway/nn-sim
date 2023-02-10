use std::{io::stdout, thread::sleep, time::Duration};

use mlp::MLP;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use tty_interface::{pos, Interface, Position, Result};
use value::Value;

mod layer;
mod mlp;
mod neuron;
mod value;

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Wall,
    Goal,
}

const SIZE: usize = 10;
type Map = [[Cell; SIZE]; SIZE];

struct Entity {
    position: (usize, usize),
}

impl Entity {
    fn new(pos: (usize, usize)) -> Entity {
        Entity { position: pos }
    }
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Up, Down, Left, Right
}

trait ControlledEntity {
    fn get_move(&mut self, pos: (usize, usize), map: &Map) -> Option<Move>;
}

struct AIEntity(MLP);

impl AIEntity {
    fn new() -> AIEntity {
        AIEntity(MLP::new(4, vec![8, 8, 4]))
    }
}

impl ControlledEntity for AIEntity {
    fn get_move(&mut self, _pos: (usize, usize), _map: &Map) -> Option<Move> {
        // TODO: derive inputs from the map
        let input = vec![0.0, 0.0, 0.0, 0.0];
        
        let output = self.0.forward(input.iter().map(|f| Value::from(*f)).collect());

        let mut greatest_index = 0;
        let mut greatest_value = 0.0;
        
        for i in 0..output.len() {
            if output[i].data() > greatest_value {
                greatest_index = i;
                greatest_value = output[i].data();
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

const ITER_LIMIT: usize = 10;

fn main() {
    execute().unwrap();
}

fn execute() -> Result<()> {
    let map = generate_map();
    
    let mut entity = Entity::new((1, 1));
    let mut ai = AIEntity::new();

    let mut stdout = stdout();
    let mut interface = Interface::new_relative(&mut stdout)?;

    for _ in 0..ITER_LIMIT {
        let entity_move = ai.get_move(entity.position, &map);

        if let Some(entity_move) = entity_move {
            match entity_move {
                Move::Up => {
                    if entity.position.1 > 1 {
                        entity.position.1 -= 1;
                    }
                }
                Move::Down => {
                    if entity.position.1 < 8 {
                        entity.position.1 += 1;
                    }
                }
                Move::Left => {
                    if entity.position.0 > 1 {
                        entity.position.0 -= 1;
                    }
                }
                Move::Right => {
                    if entity.position.0 < 8 {
                        entity.position.0 += 1;
                    }
                }
            }
        }

        interface.clear_line(0);
        interface.set(pos!(0, 0), &format!("{:?}", entity_move));
        
        render(&mut interface, &map, &entity)?;
        sleep(Duration::from_millis(250));
    }

    Ok(())
}

fn generate_map() -> [[Cell; SIZE]; SIZE] {
    let mut map = [[Cell::Empty; SIZE]; SIZE];

    for x in 0..SIZE {
        for y in 0..SIZE {
            if x == 0 || y == 0 || x == SIZE-1 || y == SIZE-1 {
                map[y][x] = Cell::Wall;
            }
        }
    }

    map[8][7] = Cell::Goal;

    map
}

fn render(interface: &mut Interface, map: &[[Cell; SIZE]; SIZE], entity: &Entity) -> Result<()> {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if x == entity.position.0 && y == entity.position.1 {
                interface.set(pos!(x as u16, 1 + y as u16), "E");
                continue;
            }

            interface.set(
                pos!(x as u16, 1 + y as u16),
                match cell {
                    Cell::Empty => " ",
                    Cell::Wall => "#",
                    Cell::Goal => "G",
                },
            );
        }
    }

    interface.apply()
}
