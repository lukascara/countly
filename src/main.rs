extern crate pancurses;
use std::boxed::Box;
use pancurses::{initscr, endwin, Input, noecho, Window};

fn main() {
    let window = initscr();
    let mut ln_count = 0;
    let mut mit_count = 0;
    window.refresh();
    refresh_screen(&window, &ln_count,&mit_count);
    window.keypad(true);
    noecho();
    loop {
        match window.getch() {
            Some(Input::Character(x)) => {
                match x {
                    'l' => {
                        ln_count += 1;
                    }
                    'L' => {
                        ln_count -= 1;
                    }
                    'm' => {
                        mit_count += 1;
                    }
                    'M' => {
                        mit_count -= 1;
                    }
                    _ => ()
                }
            }
            Some(Input::Character(c)) => (),
            Some(Input::KeyClose) => break,
            Some(input) => (),
            None => ()

        }
        refresh_screen(&window, &ln_count, &mit_count);
    }
    endwin();
}


fn refresh_screen(window: &Window, ln_count: &u32, mit_count: &u32) {
    window.clear();
    print_count(&window, "Lymph Node Count[L,l]: ", ln_count);
    print_count(&window, "Mitotic count [M,m]: ", mit_count);
}

fn print_count(window: &Window, label: &str, count: &u32) {
    window.printw(&format!("\n{}", label));
    window.attron(pancurses::A_BOLD);
    window.printw(&format!("{}\n", *count));
    window.attroff(pancurses::A_BOLD);
}
