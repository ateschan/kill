//TODO: get started on building the ui for the game itself using the menu tree system.
//TODO: look for crates relevent to getting stdout and running bash scripts...
//

//TODO: implement state saving and parsing for saving user game data in a local toml file, then
//getting that file to maintain persistence across docker sessions.

pub mod state;
pub mod utils;
pub mod games;
pub mod scripter;

use state::exit::show_exit_menu;
use state::intro::show_intro_menu;
use cursive::Cursive;
use cursive::views::{Dialog, TextView};
use cursive::view::Margins;
use cursive::theme::Effect;

fn main() {
    let mut s = cursive::default();
    
    //Intro point...
    s.add_layer(Dialog::new()
        .content(TextView::new("KEEP. IT. LINUX. LOSER.").style(Effect::Strikethrough))
        .padding(Margins::lrtb(10, 10, 5, 5))
        .title("MAIN MENU")
        .button("EXIT", |s| show_exit_menu(s))
        .button("ENTER", |s| show_intro_menu(s)));

    s.run();
}



