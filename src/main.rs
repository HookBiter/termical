#![allow(dead_code)]
mod app;
mod calender;
mod input;
mod tui;
mod utils;

use crate::calender::calender::Calender;
fn main() {
    let mut calender = Calender::new("test");
    //let res = calender::app();
    match calender.load() {
        Ok(_) => println!("Hello, world!"),
        Err(e) => println!("error: {}", e),
    }
}
