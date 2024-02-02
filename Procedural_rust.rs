extern crate rand;

use rand::Rng;

// Define constants for dungeon size
const WIDTH: usize = 20;
const HEIGHT: usize = 10;

// Define a struct for representing the dungeon
struct Dungeon {
    tiles: [[char; WIDTH]; HEIGHT],
}

impl Dungeon {
    fn new() -> Dungeon {
        let mut tiles = [['#'; WIDTH]; HEIGHT]; // Initialize with walls

        // Generate dungeon layout
        for y in 1..HEIGHT-1 {
            for x in 1..WIDTH-1 {
                // Randomly decide whether to place a floor tile
                if rand::thread_rng().gen_range(0, 4) == 0 {
                    tiles[y][x] = '.';
                }
            }
        }

        // Place player at center of dungeon
        tiles[HEIGHT / 2][WIDTH / 2] = '@';

        Dungeon { tiles }
    }

    // Method to print the dungeon to the console
    fn print(&self) {
        for row in &self.tiles {
            for &tile in row {
                print!("{}", tile);
            }
            println!();
        }
    }
}

fn main() {
    let dungeon = Dungeon::new();
    dungeon.print();
}
