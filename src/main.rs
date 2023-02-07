mod nn;

use crate::nn::{ValueAccessor, ValueTree};

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
    let mut vt = ValueTree::new();

    let x1 = vt.create_value(2.0, "x1", None).id();
    let x2 = vt.create_value(0.0, "x2", None).id();

    let w1 = vt.create_value(-3.0, "w1", None).id();
    let w2 = vt.create_value(1.0, "w2", None).id();

    let b = vt.create_value(6.8813735870195432, "b", None).id();

    let x1w1 = vt.mul_values(x1, w1, "x1w2").id();
    let x2w2 = vt.mul_values(x2, w2, "x2w2").id();

    let x1w1x2w2 = vt.add_values(x1w1, x2w2, "x1w1x2w2").id();

    let n = vt.add_values(x1w1x2w2, b, "n").id();
    let o = vt.tanh_value(n, "o").id();

    vt.backward(o);
    println!("{:?}", vt.get(x1));

    let map = generate();
    let entity = Entity { pos: (0, 0) };

    render(&map, &entity);
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

fn render(map: &[[Cell; SIZE]; SIZE], entity: &Entity) {
    println!("Map:");
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if x == entity.pos.0 && y == entity.pos.1 {
                print!("E");
                continue;
            }

            print!(
                "{}",
                match cell {
                    Cell::Empty => " ",
                    Cell::Wall => "#",
                    Cell::Goal => "G",
                }
            );
        }
        println!();
    }
}
