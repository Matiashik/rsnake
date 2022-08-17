use std::{f32::consts::E, ops::Index};

use crate::GOptions;
use nc::mv;
use ncurses as nc;
use rand;
use std::{sync::mpsc, thread, time};

pub fn run(go: &GOptions) -> bool {
    nc::init_pair(1, go.bg_color, go.head_color);
    nc::init_pair(2, go.bg_color, go.body_color);
    nc::init_pair(3, go.bg_color, go.apple_color);
    nc::init_pair(4, if go.bg_color != 0 { 0 } else { 15 }, go.bg_color);
    nc::init_pair(5, go.bg_color, go.border_color);
    nc::init_pair(
        6,
        if go.border_color != 0 { 0 } else { 15 },
        go.border_color,
    );
    nc::nodelay(nc::stdscr(), true);
    nc::refresh();
    
    let LINS = nc::getmaxy(nc::stdscr());
    let COLS = nc::getmaxx(nc::stdscr());

    /*0
    3   1
      2*/
    let mut snake = (
        (rand::random::<f64>() * (LINS - 1) as f64) as i32,
        (rand::random::<f64>() * (COLS - 1) as f64) as i32,
        -1,
    );
    let mut apple = (
        (rand::random::<f64>() * (LINS - 1) as f64) as i32,
        (rand::random::<f64>() * (COLS - 1) as f64) as i32,
    );

    return play(snake, apple, &go);
}

fn draw(brd: bool, apple: (i32, i32), hist: &Vec<(i32, i32)>) {
    let LINS = nc::getmaxy(nc::stdscr());
    let COLS = nc::getmaxx(nc::stdscr());
    let SCORE = format!(
        "{}/{}",
        hist.len() - 1,
        if brd {
            (LINS - 2) * (COLS - 2)
        } else {
            LINS * COLS
        }
    );

    nc::clear();

    nc::color_set(4);
    for l in 0..LINS {
        for c in 0..COLS {
            nc::mv(l, c);
            nc::addch(' ' as u32);
        }
    }

    if brd {
        nc::color_set(5);
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
        nc::mv(0, 0);
        nc::color_set(6);
        nc::addstr(SCORE.as_str());
        nc::color_set(0);
    } else {
        nc::mv(0, 0);
        nc::color_set(4);
        nc::addstr(SCORE.as_str());
        nc::color_set(0);
    }

    nc::color_set(3);
    nc::mv(apple.0, apple.1);
    nc::addch(' ' as u32);
    nc::color_set(0);

    nc::color_set(1);
    nc::mv(hist.last().unwrap().0, hist.last().unwrap().1);
    nc::addch(' ' as u32);
    nc::color_set(0);

    nc::color_set(2);
    for i in 1..hist.len() - 1 {
        nc::mv(hist[i].0, hist[i].1);
        nc::addch(' ' as u32);
    }
    nc::color_set(0);
    nc::refresh();
}

fn tick(brd: bool, dir: i32, hist: &Vec<(i32, i32)>) -> (i32, i32) {
    let LINS = nc::getmaxy(nc::stdscr());
    let COLS = nc::getmaxx(nc::stdscr());

    let mut res = hist.last().unwrap().clone();

    match dir {
        0 => res.0 -= 1,
        1 => res.1 += 1,
        2 => res.0 += 1,
        3 | _ => res.1 -= 1,
    }

    if brd {
        match res {
            (0, _) => return (-1, -1),
            (_, 0) => return (-1, -1),
            (y, _) if y == LINS - 1 => return (-1, -1),
            (_, x) if x == COLS - 1 => return (-1, -1),
            _ => (),
        }
    } else {
        match res {
            (-1, x) => return (LINS - 1, x),
            (y, -1) => return (y, COLS - 1),
            (y, x) if y == LINS => return (0, x),
            (y, x) if x == COLS => return (y, 0),
            _ => (),
        }
    }

    return res;
}

fn gen_apple(hist: &Vec<(i32, i32)>, brd: bool) -> (i32, i32) {
    let LINS = nc::getmaxy(nc::stdscr());
    let COLS = nc::getmaxx(nc::stdscr());
    let mut places: Vec<(i32, i32)> = Vec::new();
    for y in (if brd { 1 } else { 0 })..LINS - (if brd { 1 } else { 0 }) {
        for x in (if brd { 1 } else { 0 })..COLS - (if brd { 1 } else { 0 }) {
            if !hist.contains(&(y, x)) {
                places.push((y, x));
            }
        }
    }
    if places.len() == 0 {
        return (-1, -1);
    }
    return places[(rand::random::<f64>() * places.len() as f64) as usize];
}

fn autopilot(hist: &Vec<(i32, i32)>, brd: bool, cur: i32) -> i32 {
    let LINS = nc::getmaxy(nc::stdscr());
    let COLS = nc::getmaxx(nc::stdscr());

    let snk = hist.last().unwrap().clone();
    /*0
    3   1
      2*/
    if COLS % 2 == 1 {
        nc::resize_term(LINS, COLS - 1);
    }

    if cur == -1 {
        if snk.1 % 2 == if brd { 1 } else { 0 } {
            if snk.1 > 0 + if brd { 1 } else { 0 } {
                if snk.0 < LINS - if { brd } { 2 } else { 1 } - 1 {
                    2
                } else {
                    3
                }
            } else {
                if snk.0 < LINS - if { brd } { 2 } else { 1 } {
                    2
                } else {
                    1
                }
            }
        } else {
            if snk.0 > 0 + if brd { 1 } else { 0 } {
                0
            } else {
                3
            }
        }
    } else if cur == 0 {
        if snk.0 > 0 + if brd { 1 } else { 0 } {
            0
        } else {
            3
        }
    } else if cur == 3 {
        if snk.0 == 0 + if brd { 1 } else { 0 } {
            2
        } else {
            0
        }
    } else if cur == 1 {
        if snk.1 == COLS - if brd { 2 } else { 1 } {
            0
        } else {
            1
        }
    } else {
        // if cur == 2
        if snk.0 < LINS - if { brd } { 2 } else { 1 } - 1 {
            2
        } else {
            if snk.1 > 0 + if brd { 1 } else { 0 } {
                3
            } else {
                if snk.0 == LINS - if { brd } { 2 } else { 1 } {
                    1
                } else {
                    2
                }
            }
        }
    }
}

fn play(mut snake: (i32, i32, i32), mut apple: (i32, i32), go: &GOptions) -> bool {
    let LINS = nc::getmaxy(nc::stdscr());
    let COLS = nc::getmaxx(nc::stdscr());
    let mut hist = vec![(0, 0), (snake.0, snake.1)];

    draw(go.border, apple, &hist);
    loop {
        snake.2 = if go.auto {
            autopilot(&hist, go.border, -1)
        } else {
            match nc::getch() {
                nc::KEY_UP => 0,
                nc::KEY_RIGHT => 1,
                nc::KEY_DOWN => 2,
                nc::KEY_LEFT => 3,
                _ => -1,
            }
        };
        if snake.2 != -1 {
            break;
        }
    }

    loop {
        let head = tick(go.border, snake.2, &hist);
        if head == (-1, -1) || (hist.contains(&head) && hist[0] != head) {
            draw(go.border, apple, &hist);
            break;
        }
        hist.push(head);
        if head == apple {
            apple = gen_apple(&hist, go.border);
        } else if apple == (-1, -1) {
            break;
        } else {
            hist.remove(0);
        }
        draw(go.border, apple, &hist);
        std::thread::sleep(time::Duration::from_millis(go.speed as u64));
        /*0
        3   1
          2*/
        snake.2 = if go.auto {
            autopilot(&hist, go.border, snake.2)
        } else {
            match match nc::getch() {
                nc::KEY_UP => 0,
                nc::KEY_RIGHT => 1,
                nc::KEY_DOWN => 2,
                nc::KEY_LEFT => 3,
                _ => -1,
            } {
                0 if snake.2 != 2 => 0,
                1 if snake.2 != 3 => 1,
                2 if snake.2 != 0 => 2,
                3 if snake.2 != 1 => 3,
                _ => snake.2,
            }
        };

        loop {
            let ch = nc::getch();
            if match ch {
                nc::KEY_UP => 0,
                nc::KEY_RIGHT => 1,
                nc::KEY_DOWN => 2,
                nc::KEY_LEFT => 3,
                _ => -1,
            } != snake.2
            {
                nc::ungetch(ch);
                break;
            }
        }
    }
    return gameover(&hist, go);
}

fn gameover(hist: &Vec<(i32, i32)>, go: &GOptions) -> bool {
    let LINS = nc::getmaxy(nc::stdscr());
    let COLS = nc::getmaxx(nc::stdscr());
    let TILES = if go.border {
        (LINS - 2) * (COLS - 2)
    } else {
        LINS * COLS
    };
    let SCORE = hist.len() as i32 - 1;
    let mut res: bool = true;

    nc::init_pair(1, 15, 0);
    nc::init_pair(2, 0, 15);
    nc::nodelay(nc::stdscr(), false);
    nc::refresh();
    nc::color_set(1);
    loop {
        nc::mv(LINS / 2 - 1, COLS / 2 - 4);
        nc::addstr(format!("You {}", if SCORE == TILES { "win" } else { "lost" }).as_str());
        nc::mv(LINS / 2, COLS / 2 - 12);
        nc::addstr(format!("Score {} out of {}", SCORE, TILES).as_str());
        nc::mv(LINS / 2 + 1, COLS / 2 - 10);
        nc::addstr("Continue?");
        nc::color_set(1);

        nc::addstr(format!(" {}", if res { ">" } else { " " }).as_str());
        if res {
            nc::attr_on(nc::A_UNDERLINE());
        }
        nc::addstr("Yes");
        nc::attr_off(nc::A_UNDERLINE());

        nc::addstr(format!(" {}", if !res { ">" } else { " " }).as_str());
        if !res {
            nc::attr_on(nc::A_UNDERLINE());
        }
        nc::addstr("No");
        nc::attr_off(nc::A_UNDERLINE());

        nc::refresh();

        match nc::getch() {
            nc::KEY_LEFT | nc::KEY_RIGHT => res = !res,
            nc::KEY_BACKSPACE => return res,
            _ => (),
        }
    }
}
