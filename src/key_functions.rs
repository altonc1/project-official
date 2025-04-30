pub mod functions{
    use std::collections::HashMap;
    use crate::raw_passes::PassingImportant;
    use crate::PlayerData;
    use crate::PassingData;
    pub fn top_passes(name: &str, data: &Vec<PassingData>) -> HashMap<(String, String), usize>{
        let mut passes: HashMap<(String,String), usize>= HashMap::new();
        for pass in data{
            if name== pass.player_name && !pass.pass_recipient_name.is_empty(){
                let pair= (pass.player_name.clone(), pass.pass_recipient_name.clone());
                *passes.entry(pair).or_insert(0) += 1;
            }
        }
        println!("{:<40}{:<40}{}", "Passer", "Recipient", "Count");
        let mut passes_vec: Vec<_> = passes.clone().into_iter().collect();
        passes_vec.sort_by(|a, b| b.1.cmp(&a.1));
        for (names,count)in &passes_vec{
            println!("{:<40}{:<40}{}", names.0, names.1, count);
        }
        passes
    }
    pub fn starting_xi (players: &Vec<PlayerData>, team: &str)-> HashMap<String, u16>{
        let mut starting= HashMap::new();
        println!("{}", team);
        println!("{:<40}{}", "Name", "Jersey Number");
        for player in players{
            if player.team_name.to_lowercase() == team.to_lowercase(){
                if player.starter == true{
                    println!("{:<40}{}", &player.player_name, &player.jersey_number);
                    starting.insert(player.player_name.clone(),player.jersey_number);
                }
            }
        }
        starting
    }

    pub fn average_pass_location (passer: &str, recipient: &str, data: &Vec<PassingData>)-> (f64, f64, f64, f64){
        let processed= PassingImportant::passing_pair(passer, recipient, data);
        let mut total_x= 0.0;
        let mut total_y= 0.0;
        let mut total_x_end=0.0;
        let mut total_y_end=0.0;
        let mut pass_count=0.0;
        for pass in processed{
                total_x += pass.x;
                total_y += pass.y;
                total_x_end += pass.end_x;
                total_y_end += pass.end_y;
                pass_count+=1.0;
            }
        let values = (total_x/pass_count,total_y/pass_count,total_x_end/pass_count,total_y_end/pass_count);
        values
        }

    pub fn average_pass_start_location (passer: &str, data: &Vec<PassingData>)-> (f64, f64){
        let mut total_x= 0.0;
        let mut total_y= 0.0;
        let mut pass_count=0.0;
        for pass in data{
            if passer == pass.player_name{
                if pass.outcome_name != ""{
                    total_x += pass.x;
                    total_y += pass.y;
                    pass_count+=1.0; 
                    }
            }
        } 
        let locations = (total_x/pass_count,total_y/pass_count);
        locations
    }
}
