pub mod player_handler {
    use std::collections::HashSet;
    use std::io::stdin;
    use Item;

    pub fn get_input() -> String {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Could not read input.");
        println!("\n");
        input.to_lowercase()
    }

    pub fn print_inv(inventory: &mut Vec<Item>) {
        for i in inventory {
            println!("///////////////////");
            println!("* {}\n", i.to_string());
        }
    }

    pub struct Player {
        pub map_location: Location,
        pub inventory: Vec<Item>,
        pub amount_of_things_licked: i32,
        pub visitedHash: HashSet<i32>,
    }

    pub struct Location {
        pub x: i32,
        pub y: i32,
    }

    impl Location {
        pub fn new(x: i32, y: i32) -> Location {
            Location { x, y }
        }

        pub fn add(&mut self, x: i32, y: i32) {
            self.x += x;
            self.y += y;
        }
    }
}
