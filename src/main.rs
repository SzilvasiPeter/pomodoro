use chrono::{Duration, NaiveTime};
use clap::{Arg, Command};
use notify_rust::Notification;
use tokio::time::sleep;

use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let matches = Command::new("Pomodoro Timer")
        .version("1.0")
        .author("Peter Szilvasi")
        .about("A simple Pomodoro timer CLI app.")
        .arg(
            Arg::new("work")
                .short('w')
                .long("work")
                .value_name("WORK_DURATION")
                .help("Sets the duration of a work session in minutes")
                .default_value("00:25:00"),
        )
        .arg(
            Arg::new("break")
                .short('b')
                .long("break")
                .value_name("BREAK_DURATION")
                .help("Sets the duration of a short break in minutes")
                .default_value("00:05:00"),
        )
        .get_matches();

    let work_duration = matches.get_one::<String>("work").unwrap();
    let break_duration = matches.get_one::<String>("break").unwrap();

    let work_duration = NaiveTime::parse_from_str(work_duration, "%H:%M:%S")
        .expect("Unable to parse the `work` argument.");
    let break_duration = NaiveTime::parse_from_str(break_duration, "%H:%M:%S")
        .expect("Unable to parse the `break` argument.");

    loop {
        println!("Start work session!");
        start_timer(work_duration).await;
        println!("Take a break!");
        start_timer(break_duration).await;
    }
}

async fn start_timer(target_time: NaiveTime) {
    println!("Target time: {}", target_time);

    println!("Press Enter to continue...");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading from stdin.");

    count_down(target_time).await;

    Notification::new()
        .summary("Timer is finished.")
        .sound_name("message-new-instant")
        .show()
        .expect("Error during sending the notification.");
}

async fn count_down(mut target_time: NaiveTime) {
    let zero = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    while target_time > zero {
        target_time -= Duration::seconds(1);
        sleep(std::time::Duration::from_secs(1)).await;
        
        print!("\rStart time elapsed: {}", target_time);
        io::stdout()
            .flush()
            .expect("Error flushing the output stream.");

    }
    println!();
}
