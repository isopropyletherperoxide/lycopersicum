use clap::Parser;
use lycopersicum::lib::count_to;
use std::io::stdin;
use std::process::exit;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Parser)]
struct Args {
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub timers: Vec<u64>,
}

const VER: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // TODO: User IO
    println!("Welcome to lycopersicum! {VER}");
    let args = Args::parse();
    let (tx, rx) = channel();
    if args.timers.is_empty() {
        println!("Please set timers with the --timers option!");
        exit(1);
    }
    let timers = args.timers;
    let clock = Arc::new(Mutex::new(Duration::from_secs(0)));
    let rx = Arc::new(Mutex::new(rx));
    std::thread::spawn(move || loop {
        for i in &timers {
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
