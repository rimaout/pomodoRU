use std::{io::{stdout, Write},env, thread::sleep, time::Duration};
//use std::io::{stdin, stdout, Read, Write};

fn is_string_numeric(sting: &str)-> bool{
    for c in sting.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn print_time(message: &str, time_in_minutes: u64){
    let mut time_in_seconds = time_in_minutes*60;
    
    let mut minutes;
    let mut seconds_first_digit;
    let mut secods_scond_digit;

    while time_in_seconds != 0{
        time_in_seconds-=1;
        
        let mut stdout = stdout();
        minutes = time_in_seconds / 60;
        seconds_first_digit = (time_in_seconds % 60)/10;
        secods_scond_digit = (time_in_seconds % 60) - (seconds_first_digit*10); 

        sleep(Duration::from_secs(1));
        print!("\r{} {}:{}{} minutes\r",message, minutes, seconds_first_digit, secods_scond_digit);
        stdout.flush().unwrap();
    }
}

/*
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn restart(){
}
*/

fn main() {
    let args: Vec <String> = env::args().collect();
    let work_time = &args[1];
    let break_time = &args[2];

    // Check if string is numeric and a integer
    if !(is_string_numeric(work_time) && is_string_numeric(break_time)){
        panic!("Error: only integers numbers are accepted");
    }

    let work_time: u64 = work_time.parse().unwrap();
    let break_time: u64 = break_time.parse().unwrap();

    let pause = false;

    while !pause{
        println!("----------------- Work -----------------");
        print_time("Focus for:", work_time);
        println!();
        println!();
        println!("----------------- Break -----------------");
        print_time("Relax for:", break_time);
        println!();
        println!();

    }
   
    println!("ciao");
}