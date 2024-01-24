#![allow(unused_imports)]

use std::io::{stdin, stdout, Read, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Mutex};
use std::{env::args, fs, thread, time::Duration};

extern crate termion;
use termion::async_stdin;
use termion::raw::IntoRawMode;

fn main() {
    //Get arguments from command line
    let focus_time: u64;
    let break_time: u64;
    let args: Vec<String> = args().collect();
    (focus_time, break_time) = collect_args(args);

    //Print Timers
    let mut focus_counter: u64 = 0; //Count how many focus cycles have been completed
    print_focus_timer(focus_time, focus_counter);
    focus_counter += 1;
    print_break_timer(break_time, focus_counter);
}

///Funcition used to check if comand line arguments are integers
fn is_string_numeric(sting: &str) -> bool {
    for c in sting.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn collect_args(args: Vec<String>) -> (u64, u64) {
    // is help flag present?
    if args.len() == 2 && (args[1] == "testhelp" || args[1] == "-h" || args[1] == "--help") {
        panic!("Usage: pomodoru [focus time] [break time]\nExample: pomodoru 25 5\nIf no arguments are provided, default values are used (25 and 5)");
    }

    //Check if no arguments are provided (use default values)
    if args.len() == 1 {
        return (25, 5);
    }

    //Check if two arguments are provided
    if args.len() != 3 {
        panic!("Error: only zero or two arguments are accepted (use pomodoru -h for help)");
    }

    //Check if arguments are numeric
    let focus_time = &args[1];
    let break_time = &args[2];
    if !(is_string_numeric(focus_time) && is_string_numeric(break_time)) {
        panic!("Error: only integers numbers are accepted");
    }

    //Convert to usigned integers
    let focus_time: u64 = focus_time.parse().unwrap();
    let break_time: u64 = break_time.parse().unwrap();

    (focus_time, break_time)
}

fn print_focus_timer(time_in_minutes: u64, focus_counter: u64) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut time_in_seconds = time_in_minutes * 60; //converting minutes in to seconds

    let mut minutes;
    let mut seconds_first_digit;
    let mut secods_scond_digit;

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    write!(stdout, "----------------- Focus -----------------").unwrap();

    while time_in_seconds != 0 {
        time_in_seconds -= 1;

        // Function to read input char and break if 'q' is pressed
        for _ in 0..10 {
            let b = stdin.next();
            if let Some(Ok(b'q')) = b {
                panic!(
                    "You worked hard for {} minutes, now go relax!",
                    focus_counter * time_in_minutes
                );
            }
            thread::sleep(Duration::from_millis(100));
        }

        minutes = time_in_seconds / 60;
        seconds_first_digit = (time_in_seconds % 60) / 10;
        secods_scond_digit = (time_in_seconds % 60) - (seconds_first_digit * 10);

        write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
        write!(
            stdout,
            "\rFocus for: {}:{}{} minutes\r",
            minutes, seconds_first_digit, secods_scond_digit
        )
        .unwrap();
        stdout.flush().unwrap();
    }
}

fn print_break_timer(time_in_minutes: u64, focus_counter: u64) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut time_in_seconds = time_in_minutes * 60; //converting minutes in to seconds

    let mut minutes;
    let mut seconds_first_digit;
    let mut secods_scond_digit;

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    write!(stdout, "----------------- Break -----------------").unwrap();

    while time_in_seconds != 0 {
        time_in_seconds -= 1;

        // Function to read input char and break if 'q' is pressed
        for _ in 0..10 {
            let b = stdin.next();
            if let Some(Ok(b'q')) = b {
                panic!(
                    "You worked hard for {} now go relax!",
                    focus_counter * time_in_minutes
                );
            }
            thread::sleep(Duration::from_millis(100));
        }

        minutes = time_in_seconds / 60;
        seconds_first_digit = (time_in_seconds % 60) / 10;
        secods_scond_digit = (time_in_seconds % 60) - (seconds_first_digit * 10);

        write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
        write!(
            stdout,
            "\rRelax for: {}:{}{} minutes\r",
            minutes, seconds_first_digit, secods_scond_digit
        )
        .unwrap();
        stdout.flush().unwrap();
    }
}
