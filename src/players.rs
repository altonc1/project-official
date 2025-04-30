use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[derive(Default)]
pub struct PlayerData{
    pub player_id: f64,
    pub player_name: String,
    pub team_name: String,
    pub starter: bool,
    pub jersey_number: u16,
}
impl PlayerData{
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