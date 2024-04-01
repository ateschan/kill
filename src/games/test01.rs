use crate::games::meta::Game;
use std::collections::HashMap;

pub fn new() -> Game {
    let name : String = "Test01".to_string();
    let current_time : f64 = 0.00;
    let description : String = "This is a test game".to_string();
    let hint : String = "Use echo".to_string();
    let objective : String = "Print the contents of ./file.txt to console...".to_string();
    let attempts : i32 = 0;
    
    let correct_answer : String = "Hello World!".to_string();

    let mut file_system : HashMap<String, String> = HashMap::new();
    file_system.insert(
        "file.txt".to_string(),
        "Hello World!".to_string()
    );

    Game{name, current_time, description, hint, objective, attempts, correct_answer, file_system} 
}
