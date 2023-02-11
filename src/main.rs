use std::{io::stdout, thread::sleep, time::Duration};

use micrograd_rs::{Value, MLP};
use tty_interface::{pos, Interface, Position, Result};

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Wall,
    Goal,
}

const SIZE: usize = 10;
struct Map {
    cells: [[Cell; SIZE]; SIZE],
    entity: Entity,
    goal: (usize, usize),
}

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
    Up,
    Down,
    Left,
    Right,
}

trait ControlledEntity {
    fn get_move(&mut self, map: &Map) -> Option<Move>;
}

struct AIEntity(MLP);

impl AIEntity {
    fn new() -> AIEntity {
        AIEntity(MLP::new(4, vec![8, 8, 4]))
    }
}

impl ControlledEntity for AIEntity {
    fn get_move(&mut self, _map: &Map) -> Option<Move> {
        // TODO: derive inputs from the map
        let input = vec![0.0, 0.0, 0.0, 0.0];

        let output = self
            .0
            .forward(input.iter().map(|f| Value::from(*f)).collect());

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
        let entity_move = ai.get_move(&map);

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

        render(&mut interface, &map)?;
        sleep(Duration::from_millis(250));
    }

    Ok(())
}

fn generate_map() -> Map {
    let mut cells = [[Cell::Empty; SIZE]; SIZE];

    for x in 0..SIZE {
        for y in 0..SIZE {
            if x == 0 || y == 0 || x == SIZE - 1 || y == SIZE - 1 {
                cells[y][x] = Cell::Wall;
            }
        }
    }

    cells[8][7] = Cell::Goal;

    Map { cells, entity: Entity::new((1, 1)), goal: (7, 8) }
}

fn render(interface: &mut Interface, map: &Map) -> Result<()> {
    for (y, row) in map.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            interface.set(
                pos!(x as u16, 1 + y as u16),
                match cell {
                    Cell::Wall => "#",
                    _ => " ",
                },
            );

            if x == map.entity.position.0 && y == map.entity.position.1 {
                interface.set(pos!(x as u16, 1 + y as u16), "E");
            }

            if x == map.goal.0 && y == map.goal.1 {
                interface.set(pos!(x as u16, 1 + y as u16), "G");
            }
        }
    }

    interface.apply()
}
