// TRACKR
// A command-line tool that helps users track habits and tasks locally, without needing to sync to the cloud.
// Types of tasks:
//      - Tasks (to-do items with optional due dates, priorities, and tags)
//      - Habits (daily/weekly recurring behaviors that you "mark" as done)
//      - Logs (a time-stamped record of what was completed and when)

extern crate trackr;

use trackr::display_or_add;
#[allow(unused_imports)]
use trackr::{Task, Habit, Log};

fn main() {
    println!("Welcome to TRACKR\nA command-line tool that helps users track habits and tasks locally");
    loop {
        match display_or_add() {
            Ok(1) => {
                if let Err(e) = trackr::display() {
                    eprintln!("Error displaying entries: {}", e);
                }
            }
            Ok(2) => {
                if let Err(e) = trackr::add() {
                    eprintln!("Error adding entry: {}", e);
                }
            }
            Ok(3) => {
                println!("Goodbye!");
                break;
            }
            Err(e) => {
                println!("Error: {}. Please try again.", e);
            }
            _ => {
                println!("Unexpected input. Please try again.");
            }
        }
    }
}

