use crate::value::Value;

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
    let x1 = Value::from(2.0).with_label("x1");
    let x2 = Value::from(0.0).with_label("x2");

    let w1 = Value::from(-3.0).with_label("w1");
    let w2 = Value::from(1.0).with_label("w2");

    let b = Value::from(6.8813735870195432).with_label("b");

    let x1w1 = (x1 * w1).with_label("x1w1");
    let x2w2 = (x2 * w2).with_label("x2w2");

    let x1w1x2w2 = (x1w1 + x2w2).with_label("x1w1x2w2");

    let n = (x1w1x2w2 + b).with_label("n");
    let o = n.tanh().with_label("o");

    o.backward();
    println!("{:?}", o);

    // let mlp = MLP::new(&mut vt, 3, vec![4, 4, 1]);

    // let xs = vec![
    //     vec![2.0, 3.0, -1.0],
    //     vec![3.0, -1.0, 0.5],
    //     vec![0.5, 1.0, 1.0],
    //     vec![1.0, 1.0, -1.0],
    // ];

    // let ys = vec![1.0, -1.0, -1.0, 1.0];

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
