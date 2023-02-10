use std::io::stdout;

use tty_interface::{pos, Interface, Position, Result};

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

struct Entity {
    pos: (usize, usize),
}

const SIZE: usize = 10;

fn main() {
    execute().unwrap();
}

fn execute() -> Result<()> {
    let map = generate();
    let entity = Entity { pos: (0, 0) };

    let mut stdout = stdout();
    let mut interface = Interface::new_relative(&mut stdout)?;

    render(&mut interface, &map, &entity)?;

    Ok(())
}

fn generate() -> [[Cell; SIZE]; SIZE] {
    let mut map = [[Cell::Empty; SIZE]; SIZE];

    for i in 2..9 {
        map[3][i] = Cell::Wall;
    }

    for i in 0..6 {
        map[7][i] = Cell::Wall;
    }

    map[9][2] = Cell::Goal;

    map
}

fn render(interface: &mut Interface, map: &[[Cell; SIZE]; SIZE], entity: &Entity) -> Result<()> {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if x == entity.pos.0 && y == entity.pos.1 {
                interface.set(pos!(x as u16, y as u16), "E");
                continue;
            }

            interface.set(
                pos!(x as u16, y as u16),
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
