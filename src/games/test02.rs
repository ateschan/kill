use crate::games::meta::Game;
use std::collections::HashMap;

pub fn new() -> Game {
    let name : String = "Test02".to_string();
    let current_time : f64 = 0.00;
    let description : String = "This is an empty test game...".to_string();
    let hint : String = "N/A".to_string();
    let objective : String = "N/A".to_string();
    let attempts : i32 = 0;
    
    let correct_answer : String = "Complete".to_string();

    let mut file_system : HashMap<String, String> = HashMap::new();

    Game{name, current_time, description, hint, objective, attempts, correct_answer, file_system} 
}


