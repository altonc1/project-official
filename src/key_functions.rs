//module with all general key functions for this project
pub mod functions{
    use std::collections::HashMap;
    use crate::PlayerData;
    use crate::PassingData;

    //function: takes in player as input and prints in all his passing partners in order of most to least. returns hashmap wih passer, reciever and value of passing count -> used for mapping
    //inputs: player name, passing data, and a bool on whether or not to print(important because mapping uses this function and if i dont have this the terminal will be flooded with outputs from this even though i only want to map)
    //logic: iterates through data and only pushes data into hashmap if name is correct and pass recipient name isnt empty. I then have a if loop with if print== true in the main interface, this is 
            // always set as true while in the mapping function it is always set at false. Hashmap has to be iterated into a vector so that it can be ordered and printed.
    pub fn top_passes(name: &str, data: &Vec<PassingData>, print: bool) -> HashMap<(String, String), usize>{
        let mut passes: HashMap<(String,String), usize>= HashMap::new();
        for pass in data{
            if name== pass.player_name && !pass.pass_recipient_name.is_empty(){
                let pair= (pass.player_name.clone(), pass.pass_recipient_name.clone());
                *passes.entry(pair).or_insert(0) += 1;
            }
        }
        if print== true{
        println!("{:<40}{:<40}{}", "Passer", "Recipient", "Count");
        let mut passes_vec: Vec<_> = passes.clone().into_iter().collect();
        passes_vec.sort_by(|a, b| b.1.cmp(&a.1));
        for (names,count)in &passes_vec{
            println!("{:<40}{:<40}{}", names.0, names.1, count);
        }
    }
        passes
    }


    //function: prints and returns starting elevent players
    //inputs: team name and player data
    //logic: makes sure team name matches and player is a starter, if it is it pushes it into hashmap and prints out the name and jersey number
    pub fn starting_xi (players: &Vec<PlayerData>, team: &str)-> HashMap<String, u16>{
        let mut starting= HashMap::new();
        println!("{} Starting XI", team);
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

    //function: vital for mapping team passing map, basically averages out a player's passing start location, this determines where their position on the map is
    //inputs: passer name, passing data
    //logic: set a mutable x and y total value and pass count. Every time passer name matches and pass is successful the values are added. At the end i do total/pass count to find averagees
    pub fn average_pass_start_location (passer: &str, data: &Vec<PassingData>)-> (f64, f64){
        let mut total_x= 0.0;
        let mut total_y= 0.0;
        let mut pass_count=0.0;
        for pass in data{
            if passer == pass.player_name{
                if pass.outcome_name.is_empty(){
                    total_x += pass.x;
                    total_y += pass.y;
                    pass_count+=1.0; 
                    }
            }
        } 
        let locations = (total_x/pass_count,total_y/pass_count);
        locations
    }

    //function: calculates a player's pass success %
    //inputs: passer name and passing data
    //logic: has a mutable counter for success and total updates total when pass is unsuccessful and both when it is, the last if loop is to make sure that a valid name is entered, if its invalid player name is prints out error
    pub fn pass_success (passer: &str, data: &Vec<PassingData>) -> u32{
        let mut total= 0;
        let mut successful= 0;
        for pass in data{
            if pass.player_name==passer{
                if pass.outcome_name.is_empty(){
                    total += 1;
                    successful +=1;
                }
                else {
                    total+=1;
                }
            }
        }
        if total==0{
            println!("Invalid Player Name");
            println!();
            return 0
        }
        else{
            let percentage= successful*100/total;
            println!("{}'s pass percentage: {}%", passer, percentage);
            println!();
            return percentage
        }
    }

    //function: prints all the players in the team that made at least one pass and prints it in order of greatest # of passes to least, returns a hashmap of player and passes count
    //inputs: team name and the passing data
    //logic: iterates through every line but only pushes data into hashmap if the name is correct and outcome name is empty, uses iterator to iterate into vector so it can be ordered and printed
    pub fn most_passes (team:&str, data: &Vec<PassingData>) -> HashMap<String, u32>{
        let mut passes: HashMap<String, u32>= HashMap::new(); 
        for pass in data{
            if pass.team_name== team && pass.outcome_name.is_empty(){
                *passes.entry(pass.player_name.clone()).or_insert(0)+= 1;
            }
        }

        println!("Top Passers on {}", team);
        println!("{:<40}{}", "Passer", "Count");
        let mut passes_vec: Vec<_> = passes.clone().into_iter().collect();
        passes_vec.sort_by(|a, b| b.1.cmp(&a.1));
        for (name,count)in &passes_vec{
            println!("{:<40}{}", name, count);
        }
        passes
    }
}







#[cfg(test)]
mod tests {
    //test functions, the following test functions goes over all the key functions. I chose to make a new data variable for each set so that eaach example is clearer to compare

    use crate::PassingData;
    use crate::starting_xi;
    use crate::PlayerData;
    use crate::key_functions::functions::average_pass_start_location;
    use crate::pass_success;
    use crate::top_passes;
    use crate::most_passes;
    #[test]
    fn test_pass_success(){
        let data = vec![
            PassingData {
                player_name: "Messi".to_string(),
                outcome_name: "".to_string(),
                ..Default::default()
            },
            PassingData {
                player_name: "Messi".to_string(),
                outcome_name: "Out".to_string(),
                ..Default::default()
            }];
        let result = pass_success("Messi", &data);
        assert_eq!(result, 50);
    }
    #[test]
    fn test_average_pass_start_location_basic() {
        let data = vec![
            PassingData {
                player_name: "Messi".into(),
                x: 10.0,
                y: 10.0,
                outcome_name: "".into(),
                ..Default::default()
            },
            PassingData {
                player_name: "Messi".into(),
                x: 20.0,
                y: 20.0,
                outcome_name: "".into(),
                ..Default::default()
                
            }];
        let (x, y) = average_pass_start_location("Messi", &data);
        assert_eq!(x, 15.0);
        assert_eq!(y, 15.0);

    }
    #[test]
    fn test_starting_xi(){
        let players = vec![
            PlayerData {
                player_name: "Messi".into(),
                team_name: "Argentina".into(),
                jersey_number: 10,
                starter: true,
                ..Default::default()
            }];
        let result = starting_xi(&players, "Argentina");
        assert_eq!(result.get("Messi"), Some(&10));
        }

    #[test]
    fn test_top_passes_basic() {
        let data = vec![
            PassingData {
                player_name: "Messi".into(),
                pass_recipient_name: "Di Maria".into(),
                ..Default::default()
            },
            PassingData {
                player_name: "Messi".into(),
                pass_recipient_name: "Di Maria".into(),
                ..Default::default()
            }];
        let result = top_passes("Messi", &data, false);
        assert_eq!(result.get(&("Messi".into(), "Di Maria".into())), Some(&2));
    }
    #[test]
    fn test_most_passes() {
        let data = vec![
            PassingData {
                player_name: "Messi".to_string(),
                team_name: "Argentina".to_string(),
                outcome_name: "".to_string(), // successful
                ..Default::default()
            },
            PassingData {
                player_name: "Messi".to_string(),
                team_name: "Argentina".to_string(),
                outcome_name: "".to_string(), // successful
                ..Default::default()
            },
            PassingData {
                player_name: "Di Maria".to_string(),
                team_name: "Argentina".to_string(),
                outcome_name: "Out".to_string(), // unsuccessful
                ..Default::default()
            }];
        let result = most_passes("Argentina", &data);
        assert_eq!(result.get("Messi"), Some(&2));
        assert_eq!(result.get("Di Maria"), None);
    }
}
