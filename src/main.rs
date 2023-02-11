use std::{io::stdout, thread::sleep, time::Duration};

use entity::{AIEntity, ControlledEntity, Entity, Move};
use map::{Cell, Map, MapGenerator, StaticMapGenerator};
use model::Model;
use tty_interface::{pos, Interface, Position, Result};

mod entity;
mod map;
mod model;
mod mutation;

const ITER_LIMIT: usize = 10;

fn main() {
    execute().unwrap();
}

fn execute() -> Result<()> {
    let generator = StaticMapGenerator;
    let mut map = generator.generate();

    let mut ai = AIEntity::new(Model::random());

    let mut stdout = stdout();
    let mut interface = Interface::new_relative(&mut stdout)?;

    for _ in 0..ITER_LIMIT {
        let entity_move = ai.get_move(&map);
        process_entity_move(&mut map.entity, entity_move);

        render(&mut interface, &map, entity_move)?;

        sleep(Duration::from_millis(250));
    }

    Ok(())
}

fn process_entity_move(entity: &mut Entity, mv: Option<Move>) {
    if let Some(entity_move) = mv {
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
}

fn render(interface: &mut Interface, map: &Map, mv: Option<Move>) -> Result<()> {
    interface.clear_rest_of_interface(pos!(0, 0));

    if let Some(mv) = mv {
        interface.set(pos!(0, 0), &format!("{:?}", mv));
    }

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
