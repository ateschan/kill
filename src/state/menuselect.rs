use cursive::view::Margins;
use cursive::views::{Dialog, ScrollView, SelectView};
use cursive::Cursive;

use crate::games::citty_cat::new;
use crate::games::{meta::Game, list::game_list};
use crate::state::{tutorial::tutorial_engine, game::engine};
use crate::utils::file::return_dummy_user;
use crate::utils::user::User;
use crate::utils::theme::set_theme_sub_menu;

pub fn menu_select(s: &mut Cursive, user : User){

    s.dump();
    set_theme_sub_menu(s);
    let mut select = SelectView::<String>::new().on_submit(on_submit_closure);

    for i in &mut game_list().iter(){
        select.add_item_str(&i.name);
    }

    //Render games out in a list view, allow user to select
    s.add_layer(ScrollView::new(Dialog::new()
        .padding(Margins::lrtb(3, 3, 2, 2))
        .title("Game Select")
        .content(select)
        ));
    }

    pub fn on_submit_closure (s: &mut Cursive, gamestr: &str){
        let mut game : Game = new();
        for i in &mut game_list().iter(){
            if gamestr.eq(&i.name){
               game = i.clone();
            }
        }
        if game.name != "Tutorial"{
            engine(s, game.clone(), return_dummy_user().clone());
        }
        else {
            tutorial_engine(s, game.clone(), return_dummy_user().clone());
        }
    }

