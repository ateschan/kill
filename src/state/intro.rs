// What is your name?
//
// --> [input field]
//
// *Seek TOML file through file.rs for USER object*
//
// if no user object found...
//        *Create user object*
//        Hello {Name}! Welcome to K.I.L.L...
//          Would you like a tutorial?
//        --> yes? no?
//
//
// if user object found...
//
//        *Unload user object*
//        Hello {Name}! Welcome back to K.I.L.L!
//          Replay tutorial?
//        --> yes? no?

use crate::{utils::file::{check_for_name, return_dummy_user}, Cursive, Dialog, TextView};
use cursive::{view::{Nameable, Resizable}, views::EditView};
use crate::state::menuselect::menu_select;

pub fn show_intro_menu(s: &mut Cursive){
    s.add_layer(Dialog::around(EditView::new()

        .on_submit(submit)
        .with_name("name"))
        .title("What is your name?")
        .button("Ok", |s|{
            let name = 
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            submit(s, &name);
        })
        .button("Cancel", |s|{
            s.pop_layer();
        })

        );
}

fn submit(s: &mut Cursive, name: &str){
   s.pop_layer();
   s.add_layer(Dialog::new().content(TextView::new("Checking name: ".to_owned() + name + "...")));
    if check_for_name(name){
        s.pop_layer();

        s.add_layer(Dialog::new().content(TextView::new("Found name!")).button("Next", |s| {
            menu_select(s, return_dummy_user());
        }));
        
    }

    else{
        s.pop_layer();
        s.add_layer(Dialog::new().content(TextView::new("Did not find name...")).button("Whatever.", |s| {
            //Create a new user here
            //
            
            //Pass that user's id into here
            menu_select(s, return_dummy_user());
        }));
    }

    //Use utility to check for name...
    //
    //Either return T or F
    //
    //Ternary to retrieve user object or create one.
    //
}
