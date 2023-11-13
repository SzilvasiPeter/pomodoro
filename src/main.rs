use clap::{Arg, Command};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

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

    let work_duration: u64 = matches
        .get_one::<String>("work")
        .unwrap()
        .parse()
        .expect("unable to parse the `work` argument");

    let break_duration: u64 = matches
        .get_one::<String>("break")
        .unwrap()
        .parse()
        .expect("unable to parse the `break` argument");

    let work_duration = Duration::from_secs(work_duration);
    let break_duration = Duration::from_secs(break_duration);

    loop {
        println!("Start work session!");
        start_timer(work_duration * 1).await;
        println!("Take a break!");
        start_timer(break_duration * 1).await;
    }
}

async fn start_timer(target_time: Duration) {
    println!("Target time: {:?}", target_time);
    println!("Press Enter to continue...");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    if io::stdin().read_line(&mut buffer).is_ok() {
        count_down(target_time);
    } else {
        eprintln!("Error reading from stdin.");
    }

    // TODO: Add ticking sound at the end of the timer
}

fn count_down(target_time: Duration) {
    let start_time = Instant::now();
    while start_time.elapsed() < target_time {
        print!("\rStart time elapsed: {:?}", start_time.elapsed());
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
    println!();
}
