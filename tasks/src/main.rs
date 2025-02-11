
// Create galaga here!


pub fn main() {
    // print!("My first main!");
    run_game();
}

const COLUMNS: u8 = 20;
const ROWS: u8 = 10;

pub struct Coords{x: u8, y: u8}

pub fn run_game() {
    let player_position = Coords{x: 10, y: 8};

    print!("+");
    for _ in 0..COLUMNS {
        print!("-")
    }
    println!("+");
    
    for row in 0..ROWS {
        print!("|");
        for col in 0..COLUMNS {
            let current_position = Coords{x: row, y: col};
            print!(" ");
        }
        println!("|");
    }

    print!("+");
    for _ in 0..COLUMNS {
        print!("-")
    }
    println!("+");
}

