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
    Torch,
}

impl Item {
    fn to_string(&self) -> &str {
        match &self {
            Item::Shovel => "Shovel",
            Item::Axe => "Axe",
            Item::Torch => "Torch",
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
struct DarkRoom {}

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
            } else if input.contains("grab") && input.contains("torch") {
                println!("You grab the torch!");
                player.inventory.push(Item::Torch);
            } else {
                println!("I didn't understand that.");
            }
        }
    }
}

impl Room for AxeRoom {
    fn description(&self) -> String {
        String::from("You look around this room. You see a door on the right wall, as well as an axe laying on the ground in front of you. Light pours in from the cieling.")
    }

    fn handle(&self, player: &mut Player) -> Event {
        println!("{}", self.description());

        loop {
            let input = get_input();

            if input.contains("grab") && input.contains("axe") {
                println!("You grab the axe!");
                player.inventory.push(Item::Axe);
            } else if input.contains("open") && input.contains("door") {
                println!("You open the door and walk in.");
                return Event::MoveRight;
            } else if input.contains("lick") && input.contains("axe") {
                println!("Why would you do that.");
                player.amount_of_things_licked += 1;
            } else {
                println!("I didn't understand that.");
            }
        }
    }
}

impl Room for BodyRoom {
    fn description(&self) -> String {
        String::from("The room is filled with bodies, not fresh, however they look like they've been here a while. A door stands at the end of the room. The light from the room prior is your only light source.")
    }

    fn handle(&self, player: &mut Player) -> Event {
        println!("{}", self.description());

        loop {
            let input = get_input();

            if input.contains("open") && input.contains("door"){
                println!("The door takes some force to open but you manage to get it open. You walk through it.");
                return Event::MoveRight;
            } else if input.contains("lick") && input.contains("body") {
                println!("What the fuck don't lick the dead people");
                player.amount_of_things_licked += 1;
            } else {
                println!("I didn't understand that.");
            }
        }
    }
}

impl Room for DarkRoom {
    fn description(&self) -> String {
        String::from("The room lights up as you hold your torch out, theres a musty smell about, The room is filled with cobwebs and the walls are made of wood, You see a door to your left and your right.")
    }

    fn handle(&self, player: &mut Player) -> Event {
        println!("It's way too dark! You can barely see inside. You head back to the room with the bodies.");
        return Event::MoveLeft;
    }
}

fn main() {
    let mut map: Vec<Vec<Option<Box<Room>>>> = vec![vec![
        Some(Box::new(EntranceRoom {})),
        Some(Box::new(AxeRoom {})),
        Some(Box::new(BodyRoom {})),
        Some(Box::new(DarkRoom {})),
    ]];

    let mut player = Player {
        map_location: Location::new(0, 0),
        inventory: vec![Item::Shovel],
        amount_of_things_licked: 0,
    };

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
