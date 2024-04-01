use cursive::view::Margins;
use cursive::views::{Dialog, ScrollView, SelectView};
use cursive::Cursive;

use crate::games::{meta::Game, list::game_list, joined_together::new};
use crate::state::game::engine;
use crate::utils::user::User;

pub fn menu_select(s: &mut Cursive, user : User){

    let on_submit_closure = move |s: &mut Cursive, gamestr: &str| {
        //Already "found" a user object
        let cpuser = user.clone();
        //Implement finding the proper game object based on &str
        let mut game : Game = new();
            for i in &mut game_list().iter(){
                if gamestr.eq(&i.name){
                    game = i.clone();
                }
    }
        
        engine(s, game, cpuser);
    };

    let prod_games = game_list();

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
