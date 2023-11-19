use clap::{Arg, Command};
use notify_rust::Notification;

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
        .expect("Unable to parse the `work` argument");

    let break_duration: u64 = matches
        .get_one::<String>("break")
        .unwrap()
        .parse()
        .expect("Unable to parse the `break` argument");

    let work_duration = Duration::from_secs(work_duration) * 60;
    let break_duration = Duration::from_secs(break_duration) * 60;

    loop {
        println!("Start work session!");
        start_timer(work_duration).await;
        println!("Take a break!");
        start_timer(break_duration).await;
    }
}

async fn start_timer(target_time: Duration) {
    println!("Target time: {}", format_duration(target_time));

    println!("Press Enter to continue...");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading from stdin.");

    count_down(target_time);

    Notification::new()
        .summary("Timer is finished.")
        .sound_name("message-new-instant")
        .show()
        .expect("Error during sending the notification.");
}

fn count_down(target_time: Duration) {
    let start_time = Instant::now();
    while start_time.elapsed() < target_time {
        print!(
            "\rStart time elapsed: {}",
            format_duration(start_time.elapsed())
        );
        io::stdout()
            .flush()
            .expect("Error flushing the output stream.");
        sleep(Duration::from_secs(1));
    }
    println!();
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Duration::from_secs(0), "00:00:00")]
    #[test_case(Duration::from_secs(1500), "00:25:00")]
    #[test_case(Duration::from_secs(3600), "01:00:00")]
    #[test_case(Duration::from_secs(98765), "27:26:05")]
    fn test_format_duration(duration: Duration, formatted_duration: &str) {
        assert_eq!(format_duration(duration), formatted_duration);
    }
}
