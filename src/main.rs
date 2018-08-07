extern crate pancurses;
use pancurses::{initscr, endwin, Input, noecho, Window};
use std::collections::HashMap;

fn main() {
    let mut hCounters: HashMap<char, Counter> = HashMap::new();
    hCounters.insert('l', Counter::new('l', "Lymph Node Count", 0));
    hCounters.insert('m', Counter::new('m', "Mitotic Figure Count", 0));

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
                match hCounters.get(&x){
                    Some(c) => {
                        if x.is_lowercase() {
                            c.increment();
                        } else { c.decrement(); }
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

pub struct Counter{
    pub label: String,
    count: u32,
    pub kb_lower: char
}



impl Counter {
    pub fn new(kb_lower: char, label: &str, count: u32) -> Counter{
        Counter {
            kb_lower,
            label: String::from(label),
            count
        }
    }

    fn increment(self) {
        self.count += 1;
    }

    fn decrement(self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }
}
