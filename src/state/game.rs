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

use cursive::{ menu, theme::Color, views::{Dialog, LinearLayout, NamedView, ResizedView, ScrollView, TextArea, TextView, Panel}, Cursive};
use cursive::utils::markup::StyledString;
use cursive_multiplex::Mux;
use crate::{state::exit::show_exit_menu, utils::file::return_dummy_user};
use crate::games::meta::Game;
use crate::utils::{theme::{set_theme_dark, set_theme_light, set_theme_sub_menu}, user::User};
use crate::scripter::run;
use crate::state::menuselect::menu_select;

pub fn engine(s: &mut Cursive, game : Game, user : User){
    game.clone().setup();

    //initialize the multiplexing window...
    let mut mux = Mux::new().with_default_split_ratio(0.7);

    let mut rootbuilder = StyledString::plain("");
   
    let obj_name = StyledString::styled("\n   Objective", Color::Dark(cursive::theme::BaseColor::Red));
    let desc_name = StyledString::styled("\n   Description", Color::Dark(cursive::theme::BaseColor::Red));
    let styled_game_desc = StyledString::plain(" - ".to_string() + &game.clone().objective + "\n");
    let styled_game_obj = StyledString::plain(" - ".to_string() + &game.clone().description + "\n");
    
   rootbuilder.append(obj_name.clone());
   rootbuilder.append(styled_game_obj.clone());
   rootbuilder.append(desc_name.clone());
   rootbuilder.append(styled_game_desc.clone());

    let instructions = Panel::new(ResizedView::with_full_screen(ScrollView::new(TextView::new(rootbuilder))));
   
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
            Panel::new(ResizedView::with_full_screen(NamedView::new("in".to_owned() + &game.name, TextArea::new())))
            .title("Input")),
            information,
        )
        .expect("INPUT FAILED");

    let _output = mux
        .add_right_of(
            Panel::new(ResizedView::with_full_screen(ScrollView::new(
                    NamedView::new("out".to_owned() + &game.name, TextView::new("Output"))))).title("Output"),
            input,
        )
        .expect("OUTPUT FAILED");

    let idlayer = cursive::views::NamedView::new("inout".to_owned() + &game.name, mux);

    let boxes = cursive::views::ResizedView::new(
        cursive::view::SizeConstraint::Full,
        cursive::view::SizeConstraint::Full,
        idlayer,
    );


    let on_submit_closure = move |s: &mut Cursive| {
        s.pop_layer();
        menu_select(s, return_dummy_user());
    };



s.add_fullscreen_layer(boxes);

    s.menubar()
        .add_subtree(
            "Menu",
            menu::Tree::new()
                .leaf(
                    "Run",
                        move |s: &mut Cursive| { // Specify the type of `s` as `&mut Cursive`
                        let content = s.find_name::<TextArea>(&("in".to_owned() + &game.name))
                            .unwrap()
                            .get_content()
                            .to_string();
                        let out : String = run::execute(content.clone());
                        s.find_name::<TextView>(&("out".to_owned() + &game.name)).unwrap().set_content(out.clone());
                        if game.clone().check(out) == true{
                            show_correct(s, return_dummy_user());
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
                        })
                        .leaf("Green", |s| {
                            set_theme_sub_menu(s);
                            s.add_layer(Dialog::info("Switched to Green"))
                        })
                )
                 .subtree(
                    "Exit",
                    menu::Tree::new()
                        .leaf("Exit to main menu", move |s| {
                            on_submit_closure(s);
                        })
                        .leaf("Exit Kill", |s| {
                            show_exit_menu(s);
                        })
                )
        );

    s.add_global_callback(cursive::event::Key::Esc, |s| s.select_menubar());
}

pub fn show_correct(s: &mut Cursive, user : User){
   //write to data here
   s.add_layer(Dialog::new().content(TextView::new("Correct!\nWould you like to continue?"))
       .button("Yes", |s| {
        s.pop_layer();
           menu_select(s, return_dummy_user().clone())})
       .button("No", |s| {s.pop_layer();}));
}


