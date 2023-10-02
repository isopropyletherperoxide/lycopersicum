use clap::Parser;
use lycopersicum::lib::count_to;
use notify_rust::Notification;
use notify_rust::Timeout;
use std::io::stdin;
use std::println;
use std::process::exit;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Parser)]
struct Args {
    /// A list of timers (specified in minutes) to loop through
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub timers: Vec<u64>,

    /// Disable Notifications
    #[clap(short, long)]
    pub no_notifications: bool,
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
        let mut n: i32 = 1;
        for i in &timers {
            if args.no_notifications == false {
                Notification::new()
                    .summary("Pomodoro:")
                    .body(format!("Timer {n} for {i} minute(s) started").as_str())
                    .timeout(Timeout::Milliseconds(6000))
                    .show()
                    .expect("Error while creating notification!");
            }
            println!("Starting timer {n} for {i} minute(s)!");
            count_to(clock.clone(), *i, rx.clone());
            n = n + 1;
        }
    });
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input!");
        match input.trim() {
            "start" => {
                tx.send("start").expect("Error sending signal!");
            }
            "show" => tx.send("show").expect("Error sending signal!"),
            "pause" => {
                tx.send("pause").expect("Error sending signal!");
            }
            "quit" => {
                println!("Quitting Lycopersicum, Goodbye!");
                exit(0);
            }
            _ => (),
        }
    }
}
