mod raw_passes;
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
    let argentina_starting=starting_xi(&players, "Argentina");
    println!();
    let france_starting=starting_xi(&players, "France");
    let name= "Olivier Giroud";
    let messi_passing=top_passes(name, &passing);
    mapping::player_passing_map(name, &passing);
    mapping::team_passing("France", &passing, &players);
    Ok(())
}
