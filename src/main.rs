extern crate pancurses;
extern crate clap;
use pancurses::{initscr, endwin, Input, noecho, Window};
use std::collections::HashMap;
use clap::{Arg, App, SubCommand};
use std::path::Path;




fn main() {
    let mut h_counters: HashMap<char, Counter> = HashMap::new();
    let matches = App::new("Countly")
        .version("0.1")
        .author("Lukas C.")
        .about("A simple counter using keboard input")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(false)
            .index(1))
        .get_matches();
    let config = matches.value_of("INPUT").unwrap_or("default");
    match config{
        "default" => {

        }
        conf => {

        }
    }



    let window = initscr();
    window.refresh();
    refresh_screen(&window, &h_counters);
    window.keypad(true);
    noecho();
    loop {
        match window.getch() {
            Some(Input::Character(x)) => {
                let tmp = x.clone();
                match h_counters.get_mut(&tmp.to_ascii_lowercase()){
                    Some(mut c) => {
                        if x.is_lowercase() {
                            c.increment();
                        } else { c.decrement(); }
                    }
                    _ => ()
                }
            }
            Some(Input::KeyClose) => break,
            Some(_) => (),
            None => ()

        }
        refresh_screen(&window, &h_counters);
    }
    endwin();
}

fn populate_default_counters(mut h_counters: HashMap<char, Counter>) {
    h_counters.insert('l', Counter::new('l', "Lymph Node Count", 0));
    h_counters.insert('m', Counter::new('m', "Mitotic Figure Count", 0));
    h_counters.insert('b', Counter::new('b', "Boys", 0));
    h_counters.insert('g', Counter::new('g', "Girls", 0));
}

fn refresh_screen(window: &Window, h_counters: &HashMap<char, Counter>) {
    window.clear();
    // print the header
    window.attron(pancurses::A_BOLD); window.attron(pancurses::A_UNDERLINE);
    window.printw("Countly:");
    window.attroff(pancurses::A_BOLD);
    window.printw(" counting made simple.\n");
    window.attroff(pancurses::A_UNDERLINE);
    window.printw("Lowercase keyboard increments. Uppercase keyboard decrements\n");
    // print each of the counters
    for (_, c) in h_counters {
        print_count(window, &c)
    }
}

fn print_count(window: &Window, c: &Counter) {
    window.printw(&format!("\n{} [{}]: ", c.label,c.kb_lower));
    window.attron(pancurses::A_BOLD);
    window.printw(&format!("{}\n", c.count));
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

    fn increment(&mut self) {
        self.count += 1
    }

    fn decrement(&mut self) {
        if self.count > 0 {
            self.count -= 1
        }
    }
}
