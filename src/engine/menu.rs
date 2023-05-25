use std::{io::{self, Read}, process};

pub struct Menu;

impl Menu {
    pub fn print_menu() {
        Self::clear_terminal();
        println!("Langton's Ant");
        println!("1. Display task list");
        println!("2. Add new task");
        println!("3. Mark task as completed");
        println!("4. Exit");
        Self::handle_choice();
    }

    fn handle_choice() {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Error reading input data.");

        match choice.trim().parse::<u32>() {
            Ok(1) => {
                Self::exit_program_from_input();
            }
            Ok(2) => {
                Self::exit_program_from_input();
            }
            Ok(3) => {
                Self::exit_program_from_input();
            }
            Ok(4) => {
                Self::exit_program_from_input();
            }
            _ => {
                Self::clear_terminal();
                println!("Error reading input data.\n");
            }
        }
    }

    fn clear_terminal() {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn exit_program_from_input() {
        Self::clear_terminal();
        println!("Goodbye.");
        process::exit(0);
    }
}