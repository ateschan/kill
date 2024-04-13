use std::collections::HashMap;
use crate::utils::file::{fs_create, fs_delete};
use crate::scripter::check::compare;

#[derive(Clone)]
pub struct Game {
    pub name : String,
    pub current_time : f64,
    pub description : String,
    pub hint : String,
    pub objective : String,
    pub attempts : i32,

    pub correct_answer : String,
    pub file_system : HashMap<String,String>
}

impl Game{
    pub fn setup(self){
        let _ = fs_create(self.file_system);
    }

    pub fn check(self, output : String) -> bool{
       //Call scripter with {script, and correct_answer}
       //The scripter will return {STDOUT : String && answer_check : bool}
        compare(self.correct_answer, output)
    }

    pub fn teardown(self){
        let _ = fs_delete(self.file_system);
    }
}

