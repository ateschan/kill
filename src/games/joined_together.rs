use crate::games::meta::Game;
use std::collections::HashMap;

pub fn new() -> Game {
    let name : String = "Joined Together".to_string();
    let current_time : f64 = 0.00;
    let description : String = "Concatenate file.txt and file2.txt".to_string();
    let hint : String = "Use 'cat file.txt file2.txt'".to_string();
    let objective : String = "Learning cat".to_string();
    let attempts : i32 = 0;
    
    let correct_answer : String = "Goodbye World!".to_string();

    let mut file_system : HashMap<String, String> = HashMap::new();
    file_system.insert(
        "file.txt".to_string(),
        "Goodbye".to_string()
    );

    file_system.insert(
        "file2.txt".to_string(),
        " World!".to_string()
    );

    Game{name, current_time, description, hint, objective, attempts, correct_answer, file_system} 
}


