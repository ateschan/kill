
use crate::games::meta::Game;

pub fn new() -> Game {
    let name : String = "Test01".to_string();
    let current_time : f64 = 0.00;
    let description : String = "This is a test game".to_string();
    let hint : String = "Use echo".to_string();
    let objective : String = "Print hello world to console...".to_string();
    let attempts : i32 = 0;
    
    Game{name, current_time, description, hint, objective, attempts} 
}
