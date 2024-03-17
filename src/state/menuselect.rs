use cursive::{vec, Cursive};

use std::vec::Vec;
use crate::games::{self, test01};
use crate::games::meta::Game;

pub fn menu_select(s: &mut Cursive, uid: &i32){
    
    //Declare list of objects
    let mut prod_games : Vec<Game> = Vec::new();

    //Push completed games to list
    prod_games.push(test01::new());  

    s.pop_layer();



}
