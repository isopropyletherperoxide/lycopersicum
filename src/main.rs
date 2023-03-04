use std::io::{stdin, stdout};
use std::process::exit;
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use lycopersicum::lib::count_to;



const VER: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // TODO: User IO
    println!("Welcome to lycopersicum! {VER}");
    let (tx, rx) = channel();
    let args = vec![30, 1, 1];
    let clock = Arc::new(Mutex::new(Duration::from_secs(0)));
    let rx = Arc::new(Mutex::new(rx));
    std::thread::spawn(move || loop {
        for i in &args {
            count_to(clock.clone(), *i, rx.clone());
        }
    });
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "start" => tx.send("start").unwrap(),
            "show" => tx.send("show").unwrap(),
            "pause" => tx.send("pause").unwrap(),
            "quit" => {
                exit(0);
            }
            _ => (),
        }
    }
}
