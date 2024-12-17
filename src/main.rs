#![allow(dead_code)]
mod app;
mod calender;
mod utils;
fn main() {
    let res = calender::app();
    match res {
        Ok(_) => println!("Hello, world!"),
        Err(e) => println!("error: {}", e),
    }
}
