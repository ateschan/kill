// Tutorial 1
//
// use cursive::views::{Dialog, TextView};
//
// fn main(){
//     let mut siv = cursive::default();
//     
//     //State is defined by layers to an application using a stackview
//     siv.add_layer(Dialog::around(TextView::new("HELLO WORLD"))
//         .title("Cursive")
//         .button("Quit", |s| s.quit()));
//
//     siv.run();
// }
//
//
//

pub mod state;
pub mod utils;
pub mod games;


use state::exit::show_exit_menu;
use state::intro::show_intro_menu;
use cursive::{Cursive, View};
use cursive::views::{Dialog, TextView};
use cursive::view::{Margins, Resizable};
use cursive::theme::Effect;

fn main() {
    let mut siv = cursive::default();
    
    //Intro point...
    siv.add_layer(Dialog::new()
        .content(TextView::new("KEEP. IT. LINUX. LOSER.").style(Effect::Strikethrough))
        .padding(Margins::lrtb(10, 10, 5, 5))
        .title("MAIN MENU")
        .button("EXIT", |s| show_exit_menu(s))
        .button("ENTER", |s| show_intro_menu(s)));

    siv.run();
}



