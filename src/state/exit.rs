// Are you sure you want to quit?
//
// --> yes? no?
//
use crate::{Dialog, Cursive};

pub fn show_exit_menu(s: &mut Cursive) {
    s.add_layer(Dialog::text("Exit KILL?")
        .title("...")
        .button("Yes!", |s| s.quit())
        .button("No!", |s| {s.pop_layer();}));
}

