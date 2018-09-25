//Text-adventure game in rust
use std::io::stdin;

fn get_input() -> String{
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Didn't enter string!");
    input
}

fn main() {
    println!(
        "You are stuck in a room with no reasonable way out, there's a shovel next to you
        and the walls look like dirt"
    );
    let mut input = String::new();
    while !(input.contains("shovel")) {
        input = get_input();
        if !(input.contains("shovel")) {
            println!("your character doesn't know what that is");
        }
    }
    room2();
}


fn room2() {
    println!("You dug yourself out, congrats, you see a door in front of you");
    let mut input = String::new();
    while !(input.contains("door")) {
        input = get_input();
        if !(input.contains("door") || input.contains("memes")){
            println!("your character doesn't know what that is");
        }
        if input.contains("memes") {
            println!("the door has no memes to repart");
        }
    }
    room3();
}

fn room3() {
    println!("You creak open the door, and in front of you is an axe!");
    let mut input = String::new();
    let mut inventory = vec!["Shovel"];
    while !(input.contains("grab") && input.contains("axe")) {
        input = get_input();
        if !(input.contains("lick") || input.contains("axe")) {
            println!("your character doesn't know what that is");
        }
        if input.contains("lick"){
            println!("bunny what the fuck");
        }
    }
    println!("You put the axe in your inventory");
    inventory.push("Axe");
}
