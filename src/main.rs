use std::io;
use terminal::{Clear, Action};
use std::time::Duration;
use std::thread::sleep;
fn main() {
    println!("Welcome to Z7's Rust Countdown Timer'");
    println!("Enter Hours");

    let mut hours = String::new();
    io::stdin()
        .read_line(&mut hours)
        .expect("Failed to read line");

    println!("Enter Minutes");
    let mut minutes = String::new();
    io::stdin()
        .read_line(&mut minutes)
        .expect("Could not read");

    println!("Enter Seaconds");
    let mut seaconds = String::new();
    io::stdin()
        .read_line(&mut seaconds)
        .expect("Could not read");

    let mut seaconds: u32 = seaconds.trim().parse().expect("Seaconds was not a number");
    let mut minutes: u32 = minutes.trim().parse().expect("Minutes was not a number");
    let mut hours: u32 = hours.trim().parse().expect("Hours was not a number");
    let mut seaconds: u32 = seaconds + 60 * {minutes};
    let mut seaconds:u32 = seaconds + 3600 * {hours};
    let mut terminal = terminal::stdout();
   

    while seaconds > 0 {
        seaconds -= 1;
        terminal.act(Action::ClearTerminal(Clear::All));
        if {seaconds} / 3600 >= 1 {
            let mut hours = {seaconds} / 3600;
            let mut shown_seaconds = {seaconds} % 3600;
            if {shown_seaconds} / 60 >= 1{
                let mut minutes = {shown_seaconds} / 60;
                let mut shown_seaconds = {shown_seaconds} % 60;
                println!("Hours Remaining:{hours}");
                println!("Minutes Remaining:{minutes}");
                println!("Seaconds Remaining:{shown_seaconds}");
            }
            else {
            println!("Hours Remaining:{hours}");
            println!("Seaconds Remaining:{shown_seaconds}");
            }
        }
        else if {seaconds} / 60 >= 1{
            let mut minutes = {seaconds} / 60;
            let mut shown_seaconds = {seaconds} % 60;
            println!("Minutes Remaining:{minutes}");
            println!("Seaconds Remaining:{shown_seaconds}");
        }
        else {
            println!("Seaconds Remaining:{seaconds}");
        }
        
        sleep(Duration::from_secs(1));
    }
    terminal.act(Action::ClearTerminal(Clear::All));
    println!("Done");

}


