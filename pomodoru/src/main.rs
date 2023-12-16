#![allow(unused_imports)]

use std::{fs,env::args, thread, time::Duration, };
use std::sync::{Arc, Mutex};
use std::io::{stdin, stdout, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    //Variable that can be safely accesed by all the treads
    let focus_counter = Arc::new(Mutex::new(0));
    
    //Get arguments from command line
    let args: Vec <String> = args().collect();
    let focus_time = &args[1];
    let break_time = &args[2];

    //Check if string a integer
    if !(is_string_numeric(focus_time) && is_string_numeric(break_time)){
        panic!("Error: only integers numbers are accepted");
    }
    
    //Convert to usigned integers
    let focus_time: u64 = focus_time.parse().unwrap();
    let break_time: u64 = break_time.parse().unwrap();

    let focus_counter_for_tread = Arc::clone(&focus_counter);
    
    thread::spawn(move|| {
        let count = focus_counter_for_tread.lock().unwrap();
        
        
        loop{
            let mut stdout = stdout().into_raw_mode().unwrap();
            let stdin = stdin();
            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char('q') => {
                        println!("count = {}", count);
                        stdout.flush().unwrap();
                        std::process::abort()
                
                }
                _=> {}
                }
        
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    //Print Timers
    let focus_counter_for_tread = Arc::clone(&focus_counter);
    println!("----------------- Work -----------------");
    print_focus_timer("Focus for:", focus_time, focus_counter_for_tread);
    
    let num = focus_counter.lock().unwrap();
    println!("focus count = {:?}", num);

    println!("----------------- Break ----------------");
    print_break_timer("Relax for:", break_time);
}

fn run() {
    
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

fn print_break_timer(message: &str, time_in_minutes: u64){
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

fn print_focus_timer(message: &str, time_in_minutes: u64, cycle_counter: Arc<Mutex<u64>>){
    let mut cycle_counter = cycle_counter.lock().unwrap();
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
    *cycle_counter += 1;
    
}