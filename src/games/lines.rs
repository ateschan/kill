use crate::games::meta::Game;
use std::collections::HashMap;

pub fn new() -> Game {
    let name : String = "Lines".to_string();
    let current_time : f64 = 0.00;
    let description : String = "Count the number of lines in file.txt".to_string();
    let hint : String = "Use 'cat file.txt file2.txt'".to_string();
    let objective : String = "Learning wc".to_string();
    let attempts : i32 = 0;
    
    let correct_answer : String = "44".to_string();

    let mut file_system : HashMap<String, String> = HashMap::new();
    file_system.insert(
        "file.txt".to_string(),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n".to_string()
    );

    Game{name, current_time, description, hint, objective, attempts, correct_answer, file_system} 
}
