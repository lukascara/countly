extern crate bear_lib_terminal;

use bear_lib_terminal::Color;
use bear_lib_terminal::geometry::{Point, Size};
use bear_lib_terminal::geometry;
use bear_lib_terminal::terminal::{self, config, Event, KeyCode};
use std::collections::HashMap;
use std::fmt::Debug;


fn main() {
    let mut h_counters = get_default_counters();
    get_default_counters();
    terminal::open("Simple example", 80, 30);
    terminal::set(config::Window::empty().resizeable(true));

    loop {
        refresh_screen(&h_counters);
        terminal::refresh();
        match terminal::wait_event() {
            Some(x) => {
                let key_press = terminal::state::char();
                match h_counters.get_mut(&key_press.to_ascii_lowercase()){
                    Some(mut c) => {
                        if key_press.is_lowercase() {
                            c.increment();
                        } else { c.decrement(); }
                    }
                    _ => ()
                }
            }
            None => {
            }
        }
    }

    terminal::with_colors(Color::from_rgb(0xFA, 0xAF, 0x29), Color::from_rgb(0x05, 0x50, 0xD6), || terminal::print_xy(0, 1, "Colerd"));


    terminal::set_foreground(Color::from_rgb(0xFF, 0xFF, 0xFF));
    if let Some(string) = terminal::read_str(Point::new(0, 5), 30) {
        terminal::print_xy(0, 5, &*&string);
    }
    terminal::refresh();

    terminal::close();
}




fn get_default_counters() ->  HashMap<char, Counter> {
    let mut h_counters: HashMap<char, Counter> = HashMap::new();
    h_counters.insert('l', Counter::new('l', "Lymph Node Count", 0));
    h_counters.insert('m', Counter::new('m', "Mitotic Figure Count", 0));
    h_counters.insert('b', Counter::new('b', "Boys", 0));
    h_counters.insert('g', Counter::new('g', "Girls", 0));
    h_counters
}

fn refresh_screen(h_counters: &HashMap<char, Counter>) {
    // print each of the counters
    let Size{ width: x, height: y } = terminal::state::size();
    terminal::clear(None);
    terminal::print_xy(0, 0, "Countly: counting made simple");
    terminal::print_xy(1,1,&format!("Screen size: ({}, {})",x,y));
    terminal::set_foreground(Color::from_rgb(255, 255, 255));
    for ( i,(_, c)) in h_counters.iter().enumerate() {

        terminal::print_xy(1,(i + 1) as i32,
                           &format!("\n{} [[{}]]: [color=orange][font=bold]{}[/font][/color]\n", c.label,c.kb_lower, c.count.to_string()));
    }
    terminal::set_foreground(Color::from_rgb(0xFF, 0xFF, 0xFF));
}

fn print_count(c: &Counter) {

    // window.attron(pancurses::A_BOLD);
    //window.printw(&format!("{}\n", c.count));
    //window.attroff(pancurses::A_BOLD);
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