// <esc> will activate hidden menu for theme, exit, etc...
//____________________________________________________
//|<LEAD del> open menu  <LEAD T> open themes etc    |
//
//____________________________________________________
//| USER_NAME  GAME_NAME  TIME  HI_SCORE  ATTEMPTS   |
//|--------------------------------------------------|
//| Instructions blah blah blah                      |
//|             Example here...                      |
//|                                                  |
//|--------------------------------------------------|
//| pseudo test editor       |                       |
//|   (Bash panel)           |  paste STD_OUT here   |
//|                          |                       |
//|                          |                       |
//|                          |                       |
//|                          |                       |
//|                          |                       |
//|                          |                       |
//----------------------------------------------------

use cursive::{ menu, views::{Dialog, LinearLayout, NamedView, ResizedView, ScrollView, TextArea, TextView, Panel}, Cursive};
use cursive_multiplex::Mux;
use crate::{state::exit::show_exit_menu, utils::file::return_dummy_user};
use crate::utils::user::User;
use crate::games::meta::Game;
use crate::utils::theme::{set_theme_dark, set_theme_light};
use crate::scripter::run;
use crate::state::menuselect::menu_select;

pub fn engine(s: &mut Cursive, game : Game, user : User){
    game.clone().setup();


    //initialize the multiplexing window...
    let mut mux = Mux::new().with_default_split_ratio(0.7);
    
    let instructions = Panel::new(ResizedView::with_full_screen(ScrollView::new(TextView::new(
                    "Objective: ".to_owned() + &game.objective +
                    "\n\nDescription: " + &game.description
                    ))));
   

    let information = mux
        .add_right_of(
            instructions
            .title(user.clone().name + " " + &game.name)
            .title_position(cursive::align::HAlign::Left),
            mux.root().build().unwrap(),
        )
        .expect("INFORMATION FAILED");


    let input = mux
        .add_below(
            LinearLayout::vertical().child(
            Panel::new(ResizedView::with_full_screen(NamedView::new("input", TextArea::new())))
            .title("Input")),
            information,
        )
        .expect("INPUT FAILED");

    let _output = mux
        .add_right_of(
            Panel::new(ResizedView::with_full_screen(
                    NamedView::new("output", TextView::new("Output")))).title("Output"),
            input,
        )
        .expect("OUTPUT FAILED");

    let idlayer = cursive::views::NamedView::new("inout", mux);

    let boxes = cursive::views::ResizedView::new(
        cursive::view::SizeConstraint::Full,
        cursive::view::SizeConstraint::Full,
        idlayer,
    );
s.add_fullscreen_layer(boxes);

    s.menubar()
        .add_subtree(
            "Menu",
            menu::Tree::new()
                .leaf(
                    "Run", move
                        |s| { // Specify the type of `s` as `&mut Cursive`
                        let content = s.find_name::<TextArea>("input").unwrap().get_content().to_string();
                        let out : String = run::execute(content.clone());
                        s.find_name::<TextView>("output").unwrap().set_content(out.clone());
                        if game.clone().check(out) == true{
                            show_correct(s, user.clone());
                        };
                    }
                )
                .subtree(
                    "Themes",
                    menu::Tree::new()
                        .leaf("Dark", |s| {
                            set_theme_dark(s);
                            s.add_layer(Dialog::info("Switched to Dark"))
                        })
                        .leaf("Light", |s| {
                            set_theme_light(s);
                            s.add_layer(Dialog::info("Switched to Light"))
                        }),
                )
                .leaf("Exit", move |s| {
                    show_exit_menu(s);
            }),
        );

    s.add_global_callback(cursive::event::Key::Esc, |s| s.select_menubar());
    cursive::logger::init();
}

fn show_correct(s: &mut Cursive, user : User){
   //write to data here
   s.add_layer(Dialog::new().content(TextView::new("Correct!\nWould you like to continue?"))
       .button("Yes", |s| {
               s.pop_layer();
           menu_select(s, return_dummy_user());})
       .button("No", |s| {s.pop_layer();}));
}




