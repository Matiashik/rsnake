use crate::GOptions;
use ncurses as nc;

pub fn run() -> GOptions {
    let mut res = GOptions::default();

    res.border = border_menu();
    (
        res.head_color,
        res.body_color,
        res.apple_color,
        res.bg_color,
        res.border_color,
    ) = color_menu(&res);

    res.speed = speed_menu(res.speed);

    return res;
}

fn border_menu() -> bool {
    let mut pos: i8 = 0;
    let opts = [("borderLESS", false), ("borderYES", true)];
    let LINS = nc::getmaxy(nc::stdscr());
    let COLS = nc::getmaxx(nc::stdscr());

    loop {
        nc::clear();
        nc::mv(0, 0);
        nc::addstr("BACKSPACE to enter");

        nc::color_set(2);
        nc::mv(LINS / 2 - opts.len() as i32 / 2 - 1, COLS / 2 - 4);
        nc::addstr("borders?");
        nc::color_set(1);

        for i in 0..opts.len() as i8 {
            nc::mv(LINS / 2 - opts.len() as i32 / 2 + i as i32, COLS / 2 - 5);
            if pos == i {
                nc::addch('>' as u32);
                nc::attr_on(nc::A_UNDERLINE());
            }
            nc::addstr(opts[i as usize].0);
            nc::attr_off(nc::A_UNDERLINE());
        }

        match nc::getch() {
            nc::KEY_DOWN => pos += 1,
            nc::KEY_UP => pos -= 1,
            nc::KEY_BACKSPACE => return opts[pos as usize].1,
            _ => (),
        }

        if pos < 0 {
            pos = opts.len() as i8 - 1
        } else if pos > opts.len() as i8 - 1 {
            pos = 0
        }
    }
}

fn color_menu(o: &GOptions) -> (i16, i16, i16, i16, i16) {
    let mut res = [
        o.head_color,
        o.body_color,
        o.apple_color,
        o.bg_color,
        o.border_color,
    ];
    let mut pos: i8 = 0;
    let LINS = nc::getmaxy(nc::stdscr());
    let COLS = nc::getmaxx(nc::stdscr());

    loop {
        nc::clear();

        nc::init_pair(3, res[3], res[0]);
        nc::init_pair(4, res[3], res[1]);
        nc::init_pair(5, res[3], res[2]);
        nc::init_pair(6, if res[3] != 0 { 0 } else { 15 }, res[3]);
        nc::init_pair(7, res[3], res[4]);
        nc::refresh();

        // bg and help
        nc::color_set(6);
        for l in 0..LINS {
            for c in 0..COLS {
                nc::mv(l, c);
                nc::addch(' ' as u32);
            }
        }
        nc::mv(if o.border { 1 } else { 0 }, if o.border { 1 } else { 0 });
        nc::addstr("BACKSPACE to enter");
        nc::color_set(1);

        // head
        nc::color_set(3);
        nc::mv(LINS / 2 - res.len() as i32 / 2 - 3, COLS / 2 - 3);
        nc::addch(' ' as u32);
        nc::color_set(1);

        // body
        for i in 1..=5 {
            nc::color_set(4);
            nc::mv(LINS / 2 - res.len() as i32 / 2 - 3, COLS / 2 - 3 + i);
            nc::addch(' ' as u32);
            nc::color_set(1);
        }

        // apple
        nc::color_set(5);
        nc::mv(LINS / 2 - res.len() as i32 / 2 - 3, COLS / 2 - 6);
        nc::addch(' ' as u32);
        nc::color_set(1);

        if o.border {
            // border
            nc::color_set(7);
            for l in 0..LINS {
                nc::mv(l, 0);
                nc::addch(' ' as u32);
                nc::mv(l, COLS - 1);
                nc::addch(' ' as u32);
            }
            for c in 0..COLS {
                nc::mv(0, c);
                nc::addch(' ' as u32);
                nc::mv(LINS - 1, c);
                nc::addch(' ' as u32);
            }
            nc::color_set(1);
        }

        // header
        nc::color_set(2);
        nc::mv(LINS / 2 - res.len() as i32 / 2 - 1, COLS / 2 - 4);
        nc::addstr("colors");
        nc::color_set(1);

        // opts
        for i in 0..res.len() - if o.border { 0 } else { 1 } {
            nc::color_set(6);
            let h = ["head", "body", "apple", "bg", "border"];
            nc::mv(LINS / 2 - res.len() as i32 / 2 + i as i32, COLS / 2 - 4);
            if i as i8 == pos {
                nc::addch('>' as u32);
                nc::attr_on(nc::A_UNDERLINE());
            }
            nc::addstr(h[i]);
            nc::attr_off(nc::A_UNDERLINE());
            nc::addstr(" <");
            nc::color_set(3 + i as i16);
            nc::addch(' ' as u32);
            nc::color_set(6);
            nc::addch('>' as u32);
        }

        match nc::getch() {
            nc::KEY_DOWN => pos += 1,
            nc::KEY_UP => pos -= 1,
            nc::KEY_RIGHT => res[pos as usize] += 1,
            nc::KEY_LEFT => res[pos as usize] -= 1,
            nc::KEY_BACKSPACE => return (res[0], res[1], res[2], res[3], res[4]),
            _ => (),
        }

        for i in 0..res.len() - if o.border { 0 } else { 1 } {
            if res[i] < 0 {
                res[i] = 15;
            } else if res[i] > 15 {
                res[i] = 0;
            }
        }

        if pos < 0 {
            pos = res.len() as i8 - 1 - if o.border { 0 } else { 1 }
        } else if pos > res.len() as i8 - 1 - if o.border { 0 } else { 1 } {
            pos = 0
        }
    }
}

fn speed_menu(s: i16) -> i16 {
    let mut opts = [25, 50, 100, 150, 200, 250];
    let mut pos: i8 = 2;
    let LINS = nc::getmaxy(nc::stdscr());
    let COLS = nc::getmaxx(nc::stdscr());

    nc::color_set(1);
    loop {
        nc::clear();
        nc::mv(0, 0);
        nc::addstr("BACKSPACE to enter");

        nc::mv(LINS / 2 - 1, COLS / 2 - 5);
        nc::color_set(2);
        nc::addstr(format!("speed: {}", opts[pos as usize]).as_str());

        nc::mv(LINS / 2, COLS / 2 - 4);
        nc::color_set(1);
        nc::addstr("[");
        for i in 0..pos + 1 {
            nc::addstr("#");
        }
        for i in pos + 1..opts.len() as i8 {
            nc::addstr(".");
        }
        nc::addstr("]");

        match nc::getch() {
            nc::KEY_RIGHT => pos += 1,
            nc::KEY_LEFT => pos -= 1,
            nc::KEY_BACKSPACE => return opts[pos as usize],
            _ => (),
        }

        if pos < 0 {
            pos = 0
        } else if pos > opts.len() as i8 - 1 {
            pos = opts.len() as i8 - 1
        }
    }
}
