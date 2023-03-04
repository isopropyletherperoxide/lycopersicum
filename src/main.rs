use std::io::stdin;
use std::process::exit;
use std::sync::mpsc::{Receiver, channel};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

const VER: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("Welcome to lycopersicum! {VER}");
    let (tx, rx) = channel();
    let args = vec![15, 5, 0];
    let bingo = Mutex::new(Duration::from_secs(0));
    let bingo = Arc::new(bingo);
    let rx = Arc::new(Mutex::new(rx));
    std::thread::spawn(move || loop {
        for i in &args {
            count_to(bingo.clone(), *i, rx.clone());
        }
    });
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "start" => tx.send("start").unwrap(),
            "pause" => tx.send("pause").unwrap(),
            "quit" => {
                exit(0);
            }
            _ => (),
        }
    }
}

fn count_to(penis: Arc<Mutex<Duration>>, end: u64, rx: Arc<Mutex<Receiver<&str>>>) {
    let mut iter = 1;
    let mut penis2 = penis.lock().unwrap();
    println!("{end}");
    let rx = rx.try_lock().unwrap();
    while penis2.as_secs() < end {
        let state = rx.try_recv().unwrap_or(""); 
        match state {
            "start" => {
                iter = 1;
            }
            "pause" => {
                iter = 0;
            }
            "quit" => {
                return;
            }
            &_ => (),
        }
        *penis2 += Duration::from_secs(iter);
        std::thread::sleep(Duration::from_secs(1));
        //        println!("{}", penis2.as_secs());
    }
    println!("Period Elapsed!");
    *penis2 = Duration::from_micros(0);
}
