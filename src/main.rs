//Text-adventure game in rust
use std::io::stdin;

fn main() {
    let items = ["Axe", "Rope", "Shovel"];
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
    println!("this is working");
}
