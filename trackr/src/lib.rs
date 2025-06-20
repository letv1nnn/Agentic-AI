use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub desciption: String,
    pub tags: Vec<String>,
    pub completed: bool,
    pub created: DateTime<Utc>,
    pub deadline: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Habit {
    pub id: u32,
    pub description: String,
    pub got_used_to: bool,
    pub created: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub timestamp: DateTime<Utc>,
    pub description: String,
}

pub fn get_data() -> Result<serde_json::Value, &'static str> {
    println!("Task(1)  Habit(2)  Log(3)");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u8 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err("Please enter a number between 1 and 3"),
    };

    match choice {
        1 => {
            println!("Enter task description:");
            let mut description = String::new();
            std::io::stdin().read_line(&mut description).expect("Failed to read line");
            
            println!("Enter tags (comma separated):");
            let mut tags_input = String::new();
            std::io::stdin().read_line(&mut tags_input).expect("Failed to read line");
            let tags: Vec<String> = tags_input.trim().split(',').map(|s| s.trim().to_string()).collect();

            let task = Task::new(
                1,
                description.trim().to_string(),
                tags,
                false,
                Utc::now(),
                Utc::now(),
            );
            Ok(serde_json::to_value(task).unwrap())
        },
        2 => {
            println!("Enter task description:");
            let mut description = String::new();
            std::io::stdin().read_line(&mut description).expect("Failed to read line");

            println!("Did you get used to it:");
            let got_used_to = String::new();
            std::io::stdin().read_line(&mut description).expect("Failed to read line");
            let got_used_to: bool = match got_used_to.trim().parse() {
                Ok(choice) => choice,
                Err(_) => return Err("Please enter true or false"),
            };

            let habit = Habit::new(
                2,
                description.trim().to_string(),
                got_used_to,
                Utc::now(),
            );
            Ok(serde_json::to_value(habit).unwrap())
        },
        3 => {
            println!("Enter task description:");
            let mut description = String::new();
            std::io::stdin().read_line(&mut description).expect("Failed to read line");

            let log = Log::new(
                Utc::now(),
                description.trim().to_string(),
            );
            Ok(serde_json::to_value(log).unwrap())
        },
        _ => Err("Expect 1, 2 or 3")
    }
}

pub fn display() -> std::io::Result<()> {
    if !Path::new("storage.json").exists() {
        println!("No entries found. The storage file doesn't exist yet.");
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open("storage.json")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.trim().is_empty() {
        println!("No entries found.");
        return Ok(());
    }

    println!("\n===== STORED ENTRIES =====\n");

    for (i, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<serde_json::Value>(line) {
            Ok(json_value) => {
                println!("Entry {}:", i + 1);
                println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
                println!("---------------------------");
            }
            Err(e) => {
                println!("Error parsing line {}: {}", i + 1, e);
            }
        }
    }

    Ok(())
}


pub fn add() -> std::io::Result<()> {
    let data = get_data()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let json_data = serde_json::to_string(&data)?;

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("storage.json")?;

    writeln!(file, "{}", json_data)?;
    Ok(())
}

pub fn display_or_add() -> Result<u8, &'static str> {
    let mut action_str = String::new();
    println!("\n___________________________\nDisplay(1)  Add(2)  Exit(3)");
    std::io::stdin()
        .read_line(&mut action_str)
        .map_err(|_| "Failed to read line!")?;

    let action_int: u8 = action_str.trim()
        .parse()
        .map_err(|_| "Please enter a valid number!")?;

    let action_int = match action_int == 1 || action_int == 2 || action_int == 3{
        true => Ok(action_int),
        false => Err("You can pick only 1, 2 or 3!"),
    };
    action_int
}

pub fn task_habit_log() {
    println!("Task(1)  Habit(2)  Log(3)");
}

impl Task {
    pub fn new(id: u32, desciption: String, tags: Vec<String>, 
        completed: bool, created: DateTime<Utc>, deadline: DateTime<Utc>) -> Self {
        Task {
            id,
            desciption,
            tags,
            completed,
            created,
            deadline,
        }
    }
}

impl Habit {
    pub fn new(id: u32, description: String,
        got_used_to: bool, created: DateTime<Utc>) -> Self {
        Habit {
            id,
            description,
            got_used_to,
            created,
        }
    }
}

impl Log {
    pub fn new(timestamp: DateTime<Utc>, description: String) -> Self {
        Log {
            timestamp,
            description,
        }
    }
}