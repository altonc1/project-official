use plotters::coord::Shift;
use plotters::prelude::*;
use crate::key_functions::functions::average_pass_start_location;
use crate::players::PlayerData;
use crate::PassingData;
use std::f64::consts::PI;
use crate::starting_xi;
use crate::top_passes;
use plotters::style::text_anchor::Pos;
use plotters::style::text_anchor::HPos;
use plotters::style::text_anchor::VPos;





//function: draws out the field
//inputs: filename
//logic: uses plotter to draw every line
pub fn draw_field<'a>(filename: &'a str)-> Result<DrawingArea<BitMapBackend<'a>, Shift>, Box<dyn std::error::Error>>{
    let root = BitMapBackend::new(filename,(800,600)).into_drawing_area();
    root.fill(&WHITE)?;
    // center circle
    root.draw(&PathElement::new(vec![(400,0),(400,600)],&BLACK))?;
    root.draw(&Circle::new((400,300),75,ShapeStyle::from(&BLACK).stroke_width(1)))?;
    // outer lines
    root.draw(&PathElement::new(vec![(0,0), (0,600)], ShapeStyle::from(&BLACK).stroke_width(2)))?;
    root.draw(&PathElement::new(vec![(800,0), (800,600)], ShapeStyle::from(&BLACK).stroke_width(4)))?;
    root.draw(&PathElement::new(vec![(0,0), (800,0)], ShapeStyle::from(&BLACK).stroke_width(2)))?;
    root.draw(&PathElement::new(vec![(0,600), (800,600)], ShapeStyle::from(&BLACK).stroke_width(4)))?;
    //left box
    root.draw(&PathElement::new(vec![(0,125), (150,125)], &BLACK))?;
    root.draw(&PathElement::new(vec![(0,475), (150,475)], &BLACK))?;
    root.draw(&PathElement::new(vec![(150,125), (150,475)], &BLACK))?;
    // small left box
    root.draw(&PathElement::new(vec![(0,225),(60,225)],&BLACK))?;
    root.draw(&PathElement::new(vec![(0,375), (60,375)],&BLACK))?;
    root.draw(&PathElement::new(vec![(60,375), (60,225)],&BLACK))?;
    // arc
    root.draw(&Circle::new((150, 300),50,&BLACK,))?;
    root.draw(&Rectangle::new([(70,225),(150,375)],ShapeStyle::from(&WHITE).filled()))?;
    root.draw(&Circle::new((105, 300),2,ShapeStyle::from(&BLACK).filled()))?;

    // right box
    root.draw(&PathElement::new(vec![(800,125), (650,125)], &BLACK))?;
    root.draw(&PathElement::new(vec![(800,475), (650,475)], &BLACK))?;
    root.draw(&PathElement::new(vec![(650,125), (650,475)], &BLACK))?;
    // small right
    root.draw(&PathElement::new(vec![(800,225), (740,225)], &BLACK))?;
    root.draw(&PathElement::new(vec![(800,375), (740,375)], &BLACK))?;
    root.draw(&PathElement::new(vec![(740,225), (740,375)], &BLACK))?;
    // arc
    root.draw(&Circle::new((650,300),50, &BLACK))?;
    root.draw(&Rectangle::new([(651,225),(730,375)], ShapeStyle::from(&WHITE).filled()))?;
    root.draw(&Circle::new((695, 300), 2, ShapeStyle::from(&BLACK).filled()))?;
    
    Ok(root)
}



//functions: returns a png that maps out a player's individual passing map, with red arrows indicating failed passes and green indicating successful
//inputs: player name and passing data
//logic: first i call the draw field function to initialize a field, then i need to fix a x_scale and y_scale because my given data isnt in 800x600 dimensions its in 120x100 this means i need to multiply
        // all points by this scale. I iterate over data only letting it through if name matches, i adjust the x and y and then draw the lines. I then draw the arrow head as well with a bit of trig. I make 
        // it green or red depending on outcome of pass
pub fn player_passing_map (name: &str, data: &Vec<PassingData>)-> Result<(), Box<dyn std::error::Error>>{
    let filename= format!("output/{}_passing_map.png", name);
    let root= draw_field(&filename)?;
    let x_scale=800.0/120.0;
    let y_scale=600.0/80.0;
    for pass in data{
        if name== pass.player_name{
            let x_start= (pass.x * x_scale) as i32;
            let y_start= (pass.y * y_scale) as i32;
            let x_end =(pass.end_x * x_scale) as i32 ;
            let y_end= (pass.end_y * y_scale) as i32 ;
            if pass.outcome_name.is_empty(){
                root.draw(&PathElement::new(vec![ (x_start,y_start), (x_end,y_end)], &GREEN))?;
            }
            else{
                root.draw(&PathElement::new(vec![ (x_start,y_start), (x_end,y_end)], &RED))?;
            }
            
            // drawing arrow
            let arrow_size = 10.0;
            let angle = ((y_end - y_start) as f64).atan2((x_end - x_start) as f64);
            let left = (x_end as f64 - arrow_size * (angle + PI / 6.0).cos(),y_end as f64 - arrow_size * (angle + PI / 6.0).sin(),);
            let right = (x_end as f64 - arrow_size * (angle - PI / 6.0).cos(),y_end as f64 - arrow_size * (angle - PI / 6.0).sin(),);
            if pass.outcome_name.is_empty(){
                root.draw(&Polygon::new(vec![(x_end, y_end),(left.0 as i32, left.1 as i32),(right.0 as i32, right.1 as i32),],ShapeStyle::from(&GREEN).filled(),))?;
            }
            else{
                root.draw(&Polygon::new(vec![(x_end, y_end),(left.0 as i32, left.1 as i32),(right.0 as i32, right.1 as i32),],ShapeStyle::from(&RED).filled(),))?;
            }
            
        }
    }
    println!("{} passing map can be found in output folder", name);
    Ok(())    
    }


//function: returns a png that maps out average team passing map, linking players together if they have more than 5 passes and increasing arrow size as # of passes grow
//inputs: team name and passing data
//logic: first i call the draw field function, fixing x y scale like i did for previous function, I then first initialize team starters because i only want to map them. for every starter i run
        // them through average_pass_start_location to find their average passing position. I also call top_passes to find who they pass to the most. i then make sure theyre a starter by using an if
        // loop of starters.contains_key(&pair.1) as subs wont be mapped. I then set count to be above 5 in order to be mapped so that the map doesnt get too busy. I then need to make a size scale
        // scaling up as the count increases to better visualize the passes. To find where the arrow ends I basically just find where the recipient's average position is (line 123). Everything is
        // scaled and I draw the line. The arrow implementaion is a bit different because i first need to shift the end down a bit because it gets covered by the player bubbles. However, the rest of it
        // is still the same as before, simple trig claulcations.
pub fn team_passing (team:&str, passing_data: &Vec<PassingData>, player_data: &Vec<PlayerData> )->Result<(), Box<dyn std::error::Error>>{
    let filename= format!("output/{}_passing_map.png", team);
    let root= draw_field(&filename)?;
    let x_scale=800.0/120.0;
    let y_scale=600.0/80.0;
    let starters= starting_xi(player_data, team);
    for passer in starters.keys() {
        let (x,y)= average_pass_start_location(passer, passing_data);
        let pairs= top_passes(passer, passing_data, false);
        for (pair, count) in pairs{
            if starters.contains_key(&pair.1) {
                if count >= 5{
                    let scale= (count-5)/(2);
                    let (end_x,end_y)= average_pass_start_location(&pair.1, passing_data);
                    let x_end= (end_x * x_scale) as i32;
                    let y_end= (end_y * y_scale) as i32;
                    root.draw(&PathElement::new(vec![ ((x*x_scale) as i32,(y* y_scale) as i32 ), (x_end,y_end)], ShapeStyle::from(&RED).stroke_width(1+scale as u32)))?;

                    // draw arrow
                    let arrow_size = 10.0 + scale as f64;
                    let angle = ((y_end - (y * y_scale) as i32) as f64).atan2((x_end - (x * x_scale) as i32) as f64);
                    let shifted_x_end = x_end as f64 - 20.0 * angle.cos();
                    let shifted_y_end = y_end as f64 - 20.0 * angle.sin();

                    let left = (shifted_x_end as f64 - arrow_size * (angle + PI / 6.0).cos(),shifted_y_end as f64 - arrow_size * (angle + PI / 6.0).sin(),);
                    let right = (shifted_x_end as f64 - arrow_size * (angle - PI / 6.0).cos(),shifted_y_end as f64 - arrow_size * (angle - PI / 6.0).sin(),);
                    root.draw(&Polygon::new(vec![(shifted_x_end as i32,shifted_y_end as i32),(left.0 as i32, left.1 as i32),(right.0 as i32, right.1 as i32),],ShapeStyle::from(&RED).filled(),))?;
            }
            }
        }
    for (passer,number) in starters.clone(){
        let (x,y)= average_pass_start_location(&passer, passing_data);
        root.draw(&Circle::new(((x*x_scale) as i32,(y* y_scale) as i32),20,ShapeStyle::from(&WHITE).filled()))?;
        root.draw(&Circle::new(((x*x_scale) as i32,(y* y_scale) as i32),20,ShapeStyle::from(&BLACK).stroke_width(1)))?;
        root.draw(&Text::new(number.to_string(),((x*x_scale) as i32,(y* y_scale) as i32),("Arial", 20).into_font().color(&BLACK).pos(Pos::new(HPos::Center, VPos::Center))))?;
    }
    }
    println!("{} passing map can be found in output folder", team);
    Ok(())
    }
