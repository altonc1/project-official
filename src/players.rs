use serde::Deserialize;
use csv::ReaderBuilder;



//reads players data and stores it in a struct

#[derive(Debug, Deserialize)]
#[derive(Default)]
#[allow(dead_code)]
pub struct PlayerData{
    pub player_id: f64,
    pub player_name: String,
    pub team_name: String,
    pub starter: bool,
    pub jersey_number: u16,
}
impl PlayerData{
    //function: processes player data
    //input: path to data
    //logic:uses readerbuilder and pushes each line of data into vector
    pub fn player_reader (path: &str)-> Result<Vec<PlayerData>, Box<dyn std::error::Error>>{
        let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;
        let mut players = Vec::new();
        for result in rdr.deserialize(){
            let record: PlayerData =result?;
            players.push(record);
        }
        Ok(players)
    }    
}