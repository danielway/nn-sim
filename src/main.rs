#[derive(Copy, Clone)]
enum Cell {
    Empty, Wall, Goal
}

const SIZE: usize = 10;

fn main() {
    let mut map = [[Cell::Empty; SIZE]; SIZE];
    map[1][2] = Cell::Wall;
    render(&map);
}

fn render(map: &[[Cell; SIZE]; SIZE]) {
    println!("Map:");
    for row in map {
        for cell in row {
            print!("{}", match cell {
                Cell::Empty => " ",
                Cell::Wall => "#",
                Cell::Goal => "!",
            });
        }
        println!();
    }
}
