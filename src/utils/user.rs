//A user has a name, int it, and a vec of times with the coresponding index to the game
use std::vec::Vec;
use std::default::Default;
use crate::games::meta::Game;
//Adding a set of derive values to customize defaults

#[derive(Default)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct User {
   pub name : String,
   pub id : i32,
 //  times : Vec<f64>,
 //  preferred_theme : String,
 //  total_attempts : i32,
  // game_meta : Vec<Game>,
}
