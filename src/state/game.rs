// <esc> will activate hidden menu for theme, exit, etc...
//____________________________________________________
//|<LEAD del> open menu  <LEAD T> open themes etc    |
//
//
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
//

use cursive::{ menu, theme::Style, views::{Dialog, LinearLayout, NamedView, ResizedView, ScrollView, TextArea, TextView, Panel}, Cursive
};
use cursive_multiplex::Mux;
use crate::state::exit::show_exit_menu;
use crate::utils::user::User;
use crate::games::meta::Game;
use crate::utils::theme::{set_theme_dark, set_theme_light};

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
            .title(user.name + " " + &game.name)
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

    let output = mux
        .add_right_of(
            Panel::new(ResizedView::with_full_screen(TextView::new("Output"))).title("Output"),
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
        "Help",
        menu::Tree::new()
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
                game.clone().teardown();
            }),
    );

    s.add_global_callback(cursive::event::Key::Esc, |s| s.select_menubar());
    cursive::logger::init();
}



