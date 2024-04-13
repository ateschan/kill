use cursive::{With, Cursive};
use cursive::theme::{BorderStyle, Palette};

pub fn set_theme_dark(s : &mut Cursive){
    s.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;

            {
                // First, override some colors from the base palette.
                use cursive::theme::Color::TerminalDefault;
                use cursive::theme::PaletteColor::*;
                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Red.light();
                palette[Secondary] = Red.dark();
                palette[Tertiary] = White.dark();
                palette[Highlight] = Red.dark();
            }

            {
                // Then override some styles.
                use cursive::theme::Effect::*;
                use cursive::theme::PaletteStyle::*;
                use cursive::theme::Style;
                palette[Highlight] = Style::from(Red.light()).combine(Bold);
            }
        }),
    });
}

pub fn set_theme_light(s : &mut Cursive){
    s.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;

            {
                // First, override some colors from the base palette.
                use cursive::theme::PaletteColor::*;
                palette[Background] = White.dark();
                palette[View] = White.dark();
                palette[Primary] = Black.dark();
                palette[TitlePrimary] = Blue.dark();
                palette[Secondary] = Blue.dark();
                palette[Highlight] = Black.dark();
            }

            {
                // Then override some styles.
                use cursive::theme::Effect::*;
                use cursive::theme::PaletteStyle::*;
                use cursive::theme::Style;
                palette[Highlight] = Style::from(Blue.dark()).combine(Bold);
            }
        }),
    });
}

pub fn set_theme_menu(s : &mut Cursive){
    s.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::None,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;

            {
                // First, override some colors from the base palette.
                use cursive::theme::PaletteColor::*;
                palette[Background] = Black.dark();
                palette[View] =  Black.dark();
                palette[Primary] = Green.dark();
                palette[TitlePrimary] =  Red.dark();
                palette[Secondary] =  Red.dark();
                palette[Highlight] = Green.dark();
            }

            {
                // Then override some styles.
                use cursive::theme::Effect::*;
                use cursive::theme::PaletteStyle::*;
                use cursive::theme::Style;
                palette[Highlight] = Style::from(Red.light()).combine(Bold);
                palette[HighlightInactive] = Style::from(Red.light()).combine(Bold);
                
            }
        }),
    });
}

pub fn set_theme_sub_menu(s : &mut Cursive){
    s.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;

            {
                // First, override some colors from the base palette.
                use cursive::theme::PaletteColor::*;
                palette[Background] = Black.dark();
                palette[View] =  Black.dark();
                palette[Primary] = Green.dark();
                palette[TitlePrimary] =  Green.dark();
                palette[Secondary] =  White.light();
                palette[Highlight] = Green.dark();
            }

            {
                // Then override some styles.
                use cursive::theme::Effect::*;
                use cursive::theme::PaletteStyle::*;
                use cursive::theme::Style;
                palette[Highlight] = Style::from(Red.light()).combine(Bold);
            }
        }),
    });
}



