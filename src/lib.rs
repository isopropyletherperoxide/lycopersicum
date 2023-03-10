pub mod lib {
    
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::sync::mpsc::Receiver;

/// Shared-State timer controlled by a channel
/// 
/// # Arguments 
/// * `clock_pt` - a pointer to a variable containing your time 
/// * `end_minutes` - amount of time to count down (in minutes) 
/// * `rx` - a pointer to a mutex receiver channel
///
/// **TODO:**
/// *Notifications*

pub fn count_to(clock_pt: Arc<Mutex<Duration>>, end_minutes: u64, rx: Arc<Mutex<Receiver<&str>>>) {
    let mut iter = 1;
    let mut unlocked_clock = clock_pt.lock().unwrap();
    let end_seconds = end_minutes * 60;
    match end_minutes {
        1 => println!("starting a timer for {end_minutes} minute"),
        _ => println!("starting a timer for {end_minutes} minutes"),
    }
    let rx = rx.try_lock().unwrap();
    while unlocked_clock.as_secs() < end_seconds {
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
            "show" => {
                let time_output_minutes = (end_seconds - Duration::as_secs(&unlocked_clock)) / 60;
                let time_output =
                    end_seconds - Duration::as_secs(&unlocked_clock) - (time_output_minutes * 60);
                println!("{time_output_minutes}:{time_output} left");
            }
            &_ => (),
        }
        *unlocked_clock += Duration::from_secs(iter);
        std::thread::sleep(Duration::from_secs(1));
        //        println!("{}", penis2.as_secs());
    }
    println!("Period Elapsed!");
    *unlocked_clock = Duration::from_micros(0);
}

} /* Lib */
