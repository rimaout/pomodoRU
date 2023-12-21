#![allow(unused_imports)]

use std::{fs,env::args, thread, time::Duration, };
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, TryRecvError};
use std::io::{stdin, stdout, Write};

fn main() {
    let focus_time: u64;
    let break_time: u64;

    //Get arguments from command line
    let args: Vec<String> = args().collect();
    (focus_time, break_time) = collect_args(args);

    //Print Timers
    let mut focus_counter = 0; //Count how many focus cycles have been completed 
    println!("----------------- Work -----------------");
    print_focus_timer("Focus for:", focus_time);
    
    focus_counter += 1;
    println!("focus count = {}", focus_counter);
    
    println!("----------------- Break ----------------");
    print_break_timer("Relax for:", break_time);
    
} 

///Funcition used to check if comand line arguments are integers 
fn is_string_numeric(sting: &str)-> bool{
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
    } else
    
    //Check if two arguments are provided
    if args.len() != 3 {
        panic!("Error: only zero or two arguments are accepted (use pomodoru -h for help)");
    }

    //Check if arguments are numeric
    let focus_time = &args[1];
    let break_time = &args[2];
    if !(is_string_numeric(focus_time) && is_string_numeric(break_time)){
        panic!("Error: only integers numbers are accepted");
    }
    
    //Convert to usigned integers
    let focus_time: u64 = focus_time.parse().unwrap();
    let break_time: u64 = break_time.parse().unwrap();

    (focus_time, break_time)
}

fn print_break_timer(message: &str, time_in_minutes: u64,){
    let mut time_in_seconds = time_in_minutes*60; //converting minutes in to seconds
    
    let mut minutes;
    let mut seconds_first_digit;
    let mut secods_scond_digit;

    let mut stdout = stdout();

    while time_in_seconds != 0{
        time_in_seconds-=1; 
        
        minutes = time_in_seconds / 60;
        seconds_first_digit = (time_in_seconds % 60)/10;
        secods_scond_digit = (time_in_seconds % 60) - (seconds_first_digit*10); 
       
        print!("\r{} {}:{}{} minutes\r",message, minutes, seconds_first_digit, secods_scond_digit);
        stdout.flush().unwrap();
    }
}

fn print_focus_timer(message: &str, time_in_minutes: u64) {
    let mut time_in_seconds = time_in_minutes*60; //converting minutes in to seconds
    
    let mut minutes;
    let mut seconds_first_digit;
    let mut secods_scond_digit;

    let mut stdout = stdout();

    while time_in_seconds != 0{
        time_in_seconds-=1; 
        
        minutes = time_in_seconds / 60;
        seconds_first_digit = (time_in_seconds % 60)/10;
        secods_scond_digit = (time_in_seconds % 60) - (seconds_first_digit*10); 

        thread::sleep(Duration::from_secs(1));
        print!("\r{} {}:{}{} minutes\r",message, minutes, seconds_first_digit, secods_scond_digit);
        stdout.flush().unwrap();
    } 
}

