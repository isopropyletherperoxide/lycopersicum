use clap::Parser;
use lycopersicum::lib::count_to;
use notify_rust::Notification;
use notify_rust::Timeout;
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
            Notification::new()
                .summary("Lycopersicum:")
                .body(format!("Timer for {i} minute(s) elapsed").as_str())
                .timeout(Timeout::Milliseconds(6000))
                .show()
                .expect("Error while creating notification!");
        }
    });
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input!");
        match input.trim() {
            "start" => tx.send("start").expect("Error sending signal!"),
            "show" => tx.send("show").expect("Error sending signal!"),
            "pause" => tx.send("pause").expect("Error sending signal!"),
            "quit" => {
                exit(0);
            }
            _ => (),
        }
    }
}
