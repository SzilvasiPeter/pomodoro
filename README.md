# Pomodoro Timer

A simple Pomodoro timer CLI app.

```
Usage: pomodoro.exe [OPTIONS]

Options:
  -w, --work <WORK_DURATION>    Sets the duration of a work session. [default: 00:25:00]
  -b, --break <BREAK_DURATION>  Sets the duration of a short break. [default: 00:05:00]
  -h, --help                    Print help
  -V, --version                 Print version
```

Try out with the following command:

```
cargo run -- -w 00:00:10 -b 00:00:02
```