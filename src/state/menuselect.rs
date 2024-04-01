use cursive::view::{Margins, Nameable, Resizable};
use cursive::views::{Dialog, ScrollView, SelectView};
use cursive::Cursive;

use std::vec::Vec;
use crate::games::{test01, test02};
use crate::games::meta::Game;
use crate::state::game::engine;
use crate::utils::user::User;



pub fn menu_select(s: &mut Cursive, user : User){

    let on_submit_closure = move |s: &mut Cursive, gamestr: &str| {
        //Already "found" a user object
        let cpuser = user.clone();
        //Implement finding the proper game object based on &str
        let game = test01::new();
        
        engine(s, game, cpuser);
    };

    //pop into games.rs screen...
    //Declare list of objects
    let mut prod_games : Vec<Game> = Vec::new();

    //Push completed games to list
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test01::new());  
    prod_games.push(test02::new());  

    s.pop_layer();
    let mut select = SelectView::<String>::new().on_submit(on_submit_closure);

    for i in &mut prod_games.iter(){
        select.add_item_str(&i.name);
    }

    s.pop_layer();
    //Render games out in a list view, allow user to select
    s.add_layer(ScrollView::new(Dialog::new()
        .padding(Margins::lrtb(20, 20, 5, 5))
        .title("Game Select")
        .content(select)
        ));

}


