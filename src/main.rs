//TODO: implement state saving and parsing for saving user game data in a local toml file, then
//getting that file to maintain persistence across docker sessions.
//
//TODO: Implement answer and progress tracking
//
//TODO: Write more games
//
//TODO: Make sure this shit works with docker

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
use crate::utils::theme::set_theme_menu;

fn main() {
    let mut s = cursive::default();
    set_theme_menu(&mut s);
    
    //Intro point...
    s.add_layer(Dialog::new()
        .content(TextView::new("
  G:              ,;        ,;                                                       L.             :                                         t#,           .        ,;           
  E#,    :      f#i       f#i t                t                             i   t   EW:        ,ft Ef                                   i   ;##W.         ;W      f#i j.         
  E#t  .GE    .E#t      .E#t  ED.              Ej GEEEEEEEL                 LE   Ej  E##;       t#E E#t                                 LE  :#L:WE        f#E    .E#t  EW,        
  E#t j#K;   i#W,      i#W,   E#K:             E#,,;;L#K;;.                L#E   E#, E###t      t#E E#t     :KW,      L                L#E .KG  ,#D     .E#f    i#W,   E##j       
  E#GK#f    L#D.      L#D.    E##W;            E#t   t#E                  G#W.   E#t E#fE#f     t#E E#t      ,#W:   ,KG               G#W. EE    ;#f   iWW;    L#D.    E###D.     
  E##D.   :K#Wfff;  :K#Wfff;  E#E##t           E#t   t#E                 D#K.    E#t E#t D#G    t#E E#t fi    ;#W. jWi               D#K. f#.     t#i L##Lffi:K#Wfff;  E#jG#W;    
  E##Wi   i##WLLLLt i##WLLLLt E#ti##f          E#t   t#E                E#K.     E#t E#t  f#E.  t#E E#t L#j    i#KED.               E#K.  :#G     GK tLLG##L i##WLLLLt E#t t##f   
  E#jL#D:  .E#L      .E#L     E#t ;##D.        E#t   t#E              .E#E.      E#t E#t   t#K: t#E E#t L#L     L#W.              .E#E.    ;#L   LW.   ,W#i   .E#L     E#t  :K#E: 
  E#t ,K#j   f#E:      f#E:   E#ELLE##K:       E#t   t#E             .K#E        E#t E#t    ;#W,t#E E#tf#E:   .GKj#K.            .K#E       t#f f#:   j#E.      f#E:   E#KDDDD###i
  E#t   jD    ,WW;      ,WW;  E#L;;;;;;,       E#t   t#E            .K#D         E#t E#t     :K#D#E E###f    iWf  i#K.          .K#D         f#D#;  .D#j         ,WW;  E#f,t#Wi,,,
  j#t          .D#;      .D#; E#t              E#t   t#E           .W#G          E#t E#t      .E##E E#K,    LK:    t#E         .W#G           G#t  ,WK,           .D#; E#t  ;#W:  
   ,;            tt        tt E#t              E#t    fE          :W##########Wt E#t ..         G#E EL      i       tDj       :W##########Wt   t   EG.              tt DWi   ,KK: 
                                               ,;.     :          :,,,,,,,,,,,,,.,;.             fE :                         :,,,,,,,,,,,,,.      ,                              
\n                                         By: Addison Kane Teschan
                                                                                                      

").style(Effect::Strikethrough))
        .padding(Margins::lrtb(10, 10, 5, 5))
        .title("MAIN MENU")
        .button("EXIT", |s| show_exit_menu(s))
        .button("ENTER", |s| show_intro_menu(s)));

    s.run();
}



