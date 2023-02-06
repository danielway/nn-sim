mod nn;
use nn::Value;

#[derive(Copy, Clone)]
enum Cell {
    Empty, Wall, Goal
}

struct Entity {
    pos: (usize, usize),
}

const SIZE: usize = 10;

fn main() {
    let a = Value::new(2.0, "A", None);
    let b = Value::new(5.0, "B", None);
    let c = a.add(b);
    println!("{}", c.data);
    return;
    
    let mut map = generate();
    
    let mut entity = Entity{pos: (0, 0)};
    
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
            
            print!("{}", match cell {
                Cell::Empty => " ",
                Cell::Wall => "#",
                Cell::Goal => "G",
            });
        }
        println!();
    }
}
