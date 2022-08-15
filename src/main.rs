mod game;
mod menu;

use ncurses as nc;
use std::env;

#[derive(Debug, Clone, Copy)]
pub struct GOptions {
    border: bool,

    head_color: i16,
    body_color: i16,
    apple_color: i16,
    bg_color: i16,
    border_color: i16,

    speed: i16, // millis

    auto: bool,
}

impl GOptions {
    fn default() -> GOptions {
        GOptions {
            border: false,
            head_color: 2,
            body_color: 15,
            apple_color: 1,
            bg_color: 0,
            border_color: 3,
            speed: 100,
            auto: false,
        }
    }
}

fn main() {
    // setting
    nc::initscr();
    nc::start_color();
    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    nc::noecho();
    nc::keypad(nc::stdscr(), true);
    nc::refresh();

    // colors
    nc::init_pair(1, 15 as i16, 0 as i16); // white on black
    nc::init_pair(2, 0 as i16, 15 as i16); // black on white
    nc::refresh();
    nc::color_set(1);
    
    // menu
    let go = GOptions {
        auto: if env::args().len() > 1 && env::args().collect::<Vec<String>>()[1] == "--auto" {
            true
        } else {
            false
        },
        ..menu::run()
    };
    // game
    while game::run(&go) {}

    nc::endwin();
}
