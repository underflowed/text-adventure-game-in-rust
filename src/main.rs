// Text-adventure game in rust
mod playerhandler;
mod roomhandler;

use std::collections::HashSet;
use std::io::stdin;
use playerhandler::player_handler::*;
use roomhandler::room_handler::*;

#[derive(PartialEq)]
pub enum Item {
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

fn main() {
    let mut map: Vec<Vec<Option<Box<Room>>>> = vec![
        vec![
            Some(Box::new(EntranceRoom {})),
            ],
        vec![
            Some(Box::new(AxeRoom{})),
            Some(Box::new(BodyRoom {})),
            Some(Box::new(DarkRoom {})),
            ]

    ];

    let mut player = playerhandler::player_handler::Player {
        map_location: playerhandler::player_handler::Location::new(0, 0),
        inventory: vec![Item::Shovel],
        amount_of_things_licked: 0,
        visitedHash: HashSet::new(),
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
