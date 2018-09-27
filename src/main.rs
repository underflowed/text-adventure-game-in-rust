//Text-adventure game in rust
use std::io::stdin;

// Decleration of any structs the game will use
struct Player {
    Inventory: Vec<String>,
    Amount_of_things_licked: i32,
}
struct Room {
    Description: String,
}

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Didn't enter string!");
    input
}

fn main() {
    //Player Properties
    let mut player = Player {
        Inventory: vec!["Shovel".to_string()],
        Amount_of_things_licked: 0,
    };
    //Room object declarations
    let room1 = Room{
        Description : String::from("You wake up in what looks like a room, dirt lines the walls, the only source of light is a torch,
        and there's a shovel in your hand.")
    };
    let room2 = Room {
        Description: String::from(
            "In the room you see a door to the left wall, As well as an axe in front of you",
        ),
    };
    let room3 = Room {
        Description: String::from(
            "The room is filled with bodies, not fresh, however they look like they've
        been here a while. A door",
        ),
    };

    let mut input = String::new();
    //Counting amount of things licked, will lead to secret ending if lick counter reaches 50.
    if input.contains("lick") {
        println!("bunny stop");
        player.Amount_of_things_licked += 1
    }
    if input.contains("inventory"){
        println!("You currently have");
        for i in 0..player.Inventory.len() {
            println!("{}", player.Inventory[i]);
        }
    }

    //Currently treating each room as a seperate loop please fix if you can this just feels wrong LOL
    loop {
        println!("{}", room1.Description);
        if input.is_empty() == true {
            input = get_input();
        }
        if input.contains("shovel") {
            println!("You dig yourself out into another room");
            input = String::new();
            break;
        }
    }
    loop {
        println!("{}", room2.Description);
        if input.is_empty() == true {
            input = get_input();
        }

        if input.contains("axe") {
            println!("You grab the axe!");
            player.Inventory.push(String::from("Axe"));
            input = String::new();
        }
        if input.contains("open") && input.contains("door") {
            println!("You open the door and walk in.");
            input = String::new();
            break;
        }
    }
    println!("broke out of room 2 loop succesfully");
}
