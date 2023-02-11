use std::{io::stdout, thread::sleep, time::Duration};

use entity::{AIEntity, ControlledEntity, Entity, Move};
use map::{Cell, Map, MapGenerator, StaticMapGenerator};
use model::Model;
use mutation::ConstantMutationStrategy;
use tty_interface::{pos, Interface, Position, Result};

mod entity;
mod map;
mod model;
mod mutation;

const ITER_LIMIT: usize = 10;
const STEP_LIMIT: usize = 100;
const MUTATION_COUNT: usize = 5;

fn main() {
    execute().unwrap();
}

fn execute() -> Result<()> {
    let mut strategy = ConstantMutationStrategy::new(0.9);

    let mut stdout = stdout();
    let mut interface = Interface::new_relative(&mut stdout)?;

    let mut base = Model::random();
    for _ in 0..ITER_LIMIT {
        let models: Vec<Model> = [base.clone()]
            .into_iter()
            .chain((0..MUTATION_COUNT).map(|_| {
                let mut clone = base.clone();
                clone.mutate(&mut strategy);
                clone
            }))
            .collect();

        let mut scores = Vec::new();
        for model in &models {
            let (steps, dist) = simulate(&mut interface, model)?;
            let score = steps.unwrap_or(0) as f64 + dist;
            scores.push(score);
        }

        // TODO: integrate best-scoring back into the base and repeat
        // TODO: multi-thread each model's simulation and render their maps in a grid in realtime
    }

    Ok(())
}

fn simulate(interface: &mut Interface, model: &Model) -> Result<(Option<usize>, f64)> {
    let generator = StaticMapGenerator;
    let mut map = generator.generate();

    let mut ai = AIEntity::new(model);

    let dist_fn = |map: &Map| {
        ((map.goal.0 as f64 - map.entity.position.0 as f64).powf(2.0)
            + (map.goal.1 as f64 - map.entity.position.1 as f64).powf(2.0))
        .sqrt()
    };

    for step in 0..STEP_LIMIT {
        let entity_move = ai.get_move(&map);
        process_entity_move(&mut map.entity, entity_move);

        render(interface, &map, entity_move, step + 1)?;

        if map.entity.position.0 == map.goal.0 && map.entity.position.1 == map.goal.1 {
            return Ok((Some(step), dist_fn(&map)));
        }

        sleep(Duration::from_millis(250));
    }

    Ok((None, dist_fn(&map)))
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

fn render(interface: &mut Interface, map: &Map, mv: Option<Move>, step: usize) -> Result<()> {
    interface.clear_rest_of_interface(pos!(0, 0));

    interface.set(pos!(0, 0), &format!("Step {}, Move {:?}", step, mv));

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
