//Text-adventure game in rust
use std::io::stdin;

fn main() {
    let mut input = String::new();

    while input.contains("shovel") != true {
        println!(
            "You are stuck in a room with no reasonable way out, there's a shovel next to you
        and the walls look like dirt"
        );
        stdin().read_line(&mut input).expect("Didn't enter string!");
    }
    room2();
}

fn room2() {
    let mut input = String::new();
    while input.contains("door") != true {
        println!("You dug yourself out, congrats, you see a door in front of you");
        stdin().read_line(&mut input).expect("Didn't enter string!");
        if input.contains("memes") {
            println!("the door has no memes to repart");
        }
    }
    room3();
}

fn room3() {
    let mut input = String::new();
    let mut inventory = vec!["Shovel"];
    while input.contains("axe") != true {
        println!("You creak open the door, and in front of you is an axe!");
        stdin().read_line(&mut input).expect("Didn't enter string!");
    }
    println!("You put the axe in your inventory");
    inventory.push("Axe");
}
