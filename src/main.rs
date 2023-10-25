use clap::{Command, Arg};
use std::thread::sleep;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let matches = Command::new("Pomodoro Timer")
        .version("1.0")
        .author("Your Name")
        .about("A simple Pomodoro timer CLI app.")
        .arg(
            Arg::new("work")
                .short('w')
                .long("work")
                .value_name("WORK_DURATION")
                .help("Sets the duration of a work session in minutes")
                .default_value("25"),
        )
        .arg(
            Arg::new("break")
                .short('b')
                .long("break")
                .value_name("BREAK_DURATION")
                .help("Sets the duration of a short break in minutes")
                .default_value("5"),
        )
        .get_matches();

    let work_duration: u64 = matches.get_one::<String>("work").unwrap().parse().expect("unable to parse the `work` argument");
    let break_duration: u64 = matches.get_one::<String>("break").unwrap().parse().expect("unable to parse the `break` argument");

    loop {
        println!("Work for {} minutes!", work_duration);
        start_timer(work_duration * 1).await;
        println!("Take a {}-minute break!", break_duration);
        start_timer(break_duration * 1).await;
    }
}

async fn start_timer(duration: u64) {
    let start_time = Instant::now();
    let target_time = Duration::from_secs(duration);

    while start_time.elapsed() < target_time {
        print!("\rTime left: {:?} ", target_time - start_time.elapsed());
        sleep(Duration::from_secs(1));
    }
    println!("\rTime left: 00:00");
}