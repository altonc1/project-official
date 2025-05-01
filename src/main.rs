use std::io;
mod raw_passes;
use key_functions::functions::pass_success;
use players::PlayerData;
use raw_passes::PassingData;
mod players;
mod mapping;
mod key_functions;
use crate::key_functions::functions::top_passes;
use crate::key_functions::functions::starting_xi;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let passing= PassingData::passes_reader("./data/passes.csv")?;
    let players= PlayerData::player_reader("./data/players.csv")?;

    loop {
        println!("Welcome to France vs Argentina 2022 WC Final Stats");
        println!("Choose an Option (Type Option Number):");
        println!("1. Team Starting Lineups");
        println!("2. Player's Top Passing Partners");
        println!("3. Player Passing Map");
        println!("4. Team Passing Map");
        println!("5. Player Passing Percentage");
        println!("6. Exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();
        match option.trim() {
            "1" =>{
                println!("Enter team: ");
                let mut team= String::new();
                io::stdin().read_line(&mut team).unwrap();
                let team= team.trim();
                starting_xi(&players, &team);
                println!();
            }
            "2" =>{
                println!("Enter name: ");
                let mut name= String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name=name.trim();
                top_passes(name.trim(), &passing, true);
                println!();
            }
            "3" =>{
                println!("Enter name: ");
                let mut name= String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name=name.trim();
                let _=mapping::player_passing_map(&name, &passing);
                println!();
            }
            "4" =>{
                println!("Enter team: ");
                let mut team= String::new();
                io::stdin().read_line(&mut team).unwrap();
                let team= team.trim();
                let _=mapping::team_passing(&team, &passing, &players);
            }
            "5" =>{
                println!("Enter player: ");
                let mut name= String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name= name.trim();
                pass_success(name, &passing);
            }
            "6" => break,
            _ => println!("Invalid Option, Please Choose Again"),
        }
    }
    println!("See you next time");
    Ok(())
}
