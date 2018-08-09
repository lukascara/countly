extern crate bear_lib_terminal;

use bear_lib_terminal::Color;
use bear_lib_terminal::geometry::Point;
use bear_lib_terminal::terminal::{self, config, Event, KeyCode};
use std::collections::HashMap;
use std::fmt::Debug;


fn main() {
    let mut h_counters: HashMap<char, Counter> = HashMap::new();
    let mut h_counters: HashMap<char, Counter> = HashMap::new();
    terminal::open("Simple example", 80, 30);
    terminal::set(config::Window::empty().resizeable(true));
    println!("TESTME");

    terminal::print_xy(0, 0, "Countly: counting made simple");
    terminal::refresh();
    for event in terminal::events() {
        match event {
            Event::Resize{width, height} => {
                terminal::print_xy(0, 0, &*&format!("Width: {}\nHeight: {}", width, height));
                terminal::refresh();
            },
            Event::KeyPressed{key: KeyCode::Escape, ctrl: _, shift: _} => {
                println!("BREAK");
                break
            },
            Event::KeyPressed{key: KeyCode, ctrl: _, shift: _} => {
                println!("Print A");
                refresh_screen(&h_counters);
                break
            }

            _                                                                         => (),
        }
    }

    loop {
        match terminal::wait_event() {
            Some(x) => {
                let tmp = terminal::state::char();
                println!("{}", tmp.to_string());
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


fn populate_default_counters(mut h_counters: HashMap<char, Counter>) {
    h_counters.insert('l', Counter::new('l', "Lymph Node Count", 0));
    h_counters.insert('m', Counter::new('m', "Mitotic Figure Count", 0));
    h_counters.insert('b', Counter::new('b', "Boys", 0));
    h_counters.insert('g', Counter::new('g', "Girls", 0));
}

fn refresh_screen(h_counters: &HashMap<char, Counter>) {
    terminal::print_xy(0,2,"Counter #1");
    terminal::print_xy(0,3,"Counter #2");
    terminal::print_xy(0,4,"Counter #3");
    // print each of the counters
    for (_, c) in h_counters {
        print_count( &c)
    }
}

fn print_count(c: &Counter) {
    //window.printw(&format!("\n{} [{}]: ", c.label,c.kb_lower));
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