use serde::Deserialize;
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

