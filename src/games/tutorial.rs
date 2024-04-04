use crate::games::meta::Game;
use std::collections::HashMap;

pub fn new() -> Game {
    let name : String = "Tutorial".to_string();
    let current_time : f64 = 0.00;
    let description : String = "".to_string();
    let hint : String = "".to_string();
    let objective : String = "".to_string();
    let attempts : i32 = 0;
    
    let correct_answer : String = "".to_string();

    let mut file_system : HashMap<String, String> = HashMap::new();
    file_system.insert(
        "".to_string(),
        "".to_string()
    );

    Game{name, current_time, description, hint, objective, attempts, correct_answer, file_system} 
}

