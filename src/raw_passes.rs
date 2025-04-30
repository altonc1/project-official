use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize)]
#[derive(Default)]
pub struct PassingData{
    pub minute: u32,
    pub player_id: f64,
    pub player_name: String,
    pub team_name: String,
    pub x: f64,
    pub y: f64,
    pub end_x: f64,
    pub end_y: f64,
    #[serde(default)]
    pub pass_recipient_id: Option<f64>,
    #[serde(default)]
    pub pass_recipient_name: String,
    #[serde(default)]
    pub outcome_id: Option<f64>,
    #[serde(default)]
    pub outcome_name: String,
}

impl PassingData {
    pub fn passes_reader (path: &str)-> Result<Vec<PassingData>, Box<dyn std::error::Error>>{
        let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;
        let mut passes = Vec::new();
        for result in rdr.deserialize(){
            let record: PassingData =result?;
            passes.push(record);
        }
        Ok(passes)
    }
}
pub struct PassingImportant{
    pub team_name: String,
    pub player_name: String,
    pub x: f64,
    pub y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub pass_recipient_name: String,
}

impl PassingImportant{
    pub fn passing_pair(passer: &str, recipient: &str, data:&Vec<PassingData>) ->  Vec<PassingImportant>{
        let mut processed= Vec::new();
        
        for pass in data{
            if passer.to_string() == pass.player_name && recipient.to_string() == pass.pass_recipient_name{
                processed.push(PassingImportant{
                    team_name: pass.team_name.clone(),
                    player_name: pass.player_name.clone(),
                    x: pass.x,
                    y: pass.y,
                    end_x: pass.end_x,
                    end_y: pass.end_y,
                    pass_recipient_name: pass.pass_recipient_name.clone()
                    })
                }   
            }    
            processed
        }
    }
        

