use std::collections::HashMap;
use crate::utils::file::{self, fs_create, fs_delete};

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

    pub fn check(script : String){
       //Call scripter with {script, and correct_answer}
       //The scripter will return {STDOUT : String && answer_check : bool}
       //
    }

    pub fn teardown(self){
        let _ = fs_delete(self.file_system);
    }
}

// I need to split each game into 3 phases...
//
// 1) ---> | setup | the game and create any nessesary files and directories
//
// 2) ---> | check | the output of the scripter against the 1st public answer and the 2nd private answer
// (Display the output of STDOUT in the terminal section of the ui.
//
// 3) ---> | teardown | delete any files created by the program
//
// All games should implment these phases, so include them in the meta.rs

// Hidden answer & public answer will be defined in the game file
