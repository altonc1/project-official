use serde::Deserialize;
use csv::ReaderBuilder;



//reads passes data and stores it in a struct
//serde default makes it so if the field is empty it just returns an empty string
#[derive(Debug, Deserialize)]
#[derive(Default)]
#[allow(dead_code)]
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
    //function: processes passes data
    //input: path to data
    //logic:uses readerbuilder and pushes each line of data into vector
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

