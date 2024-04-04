use std::vec::Vec;
use crate::games::{citty_cat, joined_together, lines, meta::Game, tutorial};

pub fn game_list() -> Vec<Game>{
    
    let mut prod_games : Vec<Game> = Vec::new();
    //Push completed games to list
    prod_games.push(tutorial::new());  
    prod_games.push(citty_cat::new());  
    prod_games.push(joined_together::new());  
    prod_games.push(lines::new());  

    return prod_games;
}
