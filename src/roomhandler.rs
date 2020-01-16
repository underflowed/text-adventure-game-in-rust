pub mod room_handler{
    use std::io::stdin;
    use playerhandler::player_handler::*;
    use Item;



    pub trait Room {
        fn description(&self) -> String;
        fn handle(&self, player: &mut Player) -> Event;
    }
    
    pub enum Event {
        MoveUp,
        MoveLeft,
        MoveDown,
        MoveRight,
    }
    
    pub struct EntranceRoom {}
    pub struct AxeRoom {}
    pub struct BodyRoom {}
    pub struct DarkRoom {}
    
    impl Room for EntranceRoom {
        fn description(&self) -> String {
            String::from("You wake up in what looks like a room. A certain wall to the right of you seems brittle and ready to crumble. \
              The only source of light is a torch, and there's a shovel in your hand.")
        }
        fn handle(&self, player: &mut Player) -> Event {
            if player.visitedHash.contains(&1) {
                println!("You welcome the sight of the broken wall in front of you, I'm sure that took a lot of work. \
                 Hopping over rubble on the ground created from your vicious shovel based attack on the wall, you re-enter the room.");
                if !(player.inventory.contains(&Item::Torch)) {
                    println!("The torch is still burning away on one of the walls in the room.");
                }
            } else {
                println!("{}", self.description());
            }
    
            loop {
                let input = get_input();
                if !(player.visitedHash.contains(&1)) {
                    if input.contains("hit") {
                        println!("You hit the wall with the shovel and it crumbles revealing a room. You walk inside.");
                        player.visitedHash.insert(1);
                        return Event::MoveDown;
                    }
                }
                if input.contains("grab") && input.contains("torch") {
                    println!("You grab the torch!");
                    player.inventory.push(Item::Torch);
                } else if input.contains("check") && input.contains("inventory") {
                    print_inv(&mut player.inventory)
                } else if input.contains("go") && input.contains("forward")
                    || input.contains("go") && input.contains("through") && input.contains("wall")
                {
                    println!("You hope once more over the rubble into the other room.");
                    return Event::MoveDown;
                } else {
                    println!("I didn't understand that.");
                }
            }
        }
    }
    
    impl Room for AxeRoom {
        fn description(&self) -> String {
            String::from("You look around this room. You see a door on the left wall, as well as an axe laying on the ground in front of you. \
            Light pours in from the cieling.")
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
                } else if input.contains("go") && input.contains("back") {
                    println!("You walk back to the room you woke up in.");
                    return Event::MoveUp;
                } else if input.contains("check") && input.contains("inventory") {
                    print_inv(&mut player.inventory)
                } else if input.contains("look") && input.contains("up") {
                    println!("The light shines down in rays on your face, the roof is riddled with holes from what seems like years of decay and wear.");
                } else {
                    println!("I didn't understand that.");
                }
            }
        }
    }
    
    impl Room for BodyRoom {
        fn description(&self) -> String {
            String::from("Upon walking in you get the whiff of some god awful aroma, upon inspection you realize that the room is filled with bodies, \
            not fresh, however they look like they've been here a while. \
             A door stands at the end of the room. The light from the room prior is your only light source.")
        }
    
        fn handle(&self, player: &mut Player) -> Event {
            println!("{}", self.description());
    
            loop {
                let input = get_input();
    
                if input.contains("open") && input.contains("door") {
                    println!("The door takes some force to open but you manage to get it open. You walk through it.");
                    return Event::MoveRight;
                } else if input.contains("lick") && input.contains("body") {
                    println!("What the fuck don't lick the dead people");
                    player.amount_of_things_licked += 1;
                } else if input.contains("go") && input.contains("back") {
                    println!("You walk back to the room you were in");
                    return Event::MoveLeft;
                } else if input.contains("check") && input.contains("inventory") {
                    print_inv(&mut player.inventory)
                } else {
                    println!("I didn't understand that.");
                }
            }
        }
    }
    
    impl Room for DarkRoom {
        fn description(&self) -> String {
            String::from("The room lights up as you hold your torch out, your happy to notice this one doesnt have human remains, \
             The room is filled with cobwebs and the walls are made of wood, You see a door to your left and your right.")
        }
    
        fn handle(&self, player: &mut Player) -> Event {
            if !(player.inventory.contains(&Item::Torch)) {
                println!("It's way too dark! You can barely see inside. You head back to the room with the bodies.");
                return Event::MoveLeft;
            } else {
                println!("{}", self.description());
                loop {
                    let input = get_input();
                    if input.contains("go") && input.contains("back") {
                        println!("You put the torch away and walk back to the room you came from");
                        return Event::MoveLeft;
                    } else if input.contains("check") && input.contains("inventory") {
                        print_inv(&mut player.inventory)
                    }
                }
            }
        }
    }
}