// Text-adventure game in rust
use std::io::stdin;

fn get_input() -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Could not read input.");
    input.to_lowercase()
}

// Decleration of any structs the game will use
struct Player {
    map_location: Location,
    inventory: Vec<Item>,
    amount_of_things_licked: i32,
}

struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Location {
        Location { x, y }
    }

    fn add(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

enum Item {
    Shovel,
    Axe,
}

impl Item {
    fn to_string(&self) -> &str {
        match &self {
            Item::Shovel => "Shovel",
            Item::Axe => "Axe",
        }
    }
}

trait Room {
    fn description(&self) -> String;
    fn handle(&self, player: &mut Player) -> Event;
}

enum Event {
    MoveUp,
    MoveLeft,
    MoveDown,
    MoveRight,
}

struct EntranceRoom {}
struct AxeRoom {}
struct BodyRoom {}

impl Room for EntranceRoom {
    fn description(&self) -> String {
        String::from("You wake up in what looks like a room. Dirt lines the walls, the only source of light is a torch, and there's a shovel in your hand.")
    }

    fn handle(&self, player: &mut Player) -> Event {
        println!("{}", self.description());

        loop {
            let input = get_input();

            if input.contains("shovel") {
                println!("You dig yourself out into another room.");
                return Event::MoveRight;
            } else {
                println!("I didn't understand that.");
            }
        }
    }
}

impl Room for AxeRoom {
    fn description(&self) -> String {
        String::from("You look around this room. You see a door on the left wall, as well as an axe in front of you.")
    }

    fn handle(&self, player: &mut Player) -> Event {
        println!("{}", self.description());
        loop {
            let input = get_input();
            if input.contains("axe") {
                println!("You grab the axe!");
                player.inventory.push(Item::Axe);
            } else if input.contains("open") && input.contains("door") {
                println!("You open the door and walk in.");
                return Event::MoveRight;
            } else {
                println!("I didn't understand that.");
            }
        }
    }
}

impl Room for BodyRoom {
    fn description(&self) -> String {
        String::from("The room is filled with bodies, not fresh, however they look like they've been here a while. A door exists.")
    }

    fn handle(&self, player: &mut Player) -> Event {
        println!("{}", self.description());
        loop {
            let input = get_input();
            println!("I didn't understand that.");
        }
    }
}

fn main() {
    let mut map: Vec<Vec<Option<Box<Room>>>> = vec![vec![
        Some(Box::new(EntranceRoom {})),
        Some(Box::new(AxeRoom {})),
        Some(Box::new(BodyRoom {})),
    ]];

    let mut player = Player {
        map_location: Location::new(0, 0),
        inventory: vec![Item::Shovel],
        amount_of_things_licked: 0,
    };

    // Counting amount of things licked, will lead to secret ending if lick counter reaches 50.

    /*
    if input.contains("lick") {
        println!("bunny stop");
        player.amount_of_things_licked += 1
    }
    if input.contains("inventory") {
        println!("You currently have");
        for item in player.inventory.iter() {
            println!("{}", item.to_string());
        }
    }
    */

    loop {
        let (x, y) = (player.map_location.x, player.map_location.y);
        let ref mut room = map[y as usize][x as usize];
        let room = room.as_mut().unwrap();
        let event = room.handle(&mut player);
        match event {
            Event::MoveUp => player.map_location.add(0, -1),
            Event::MoveLeft => player.map_location.add(-1, 0),
            Event::MoveDown => player.map_location.add(0, 1),
            Event::MoveRight => player.map_location.add(1, 0),
        }
    }
}
