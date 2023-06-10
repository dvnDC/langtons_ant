use std::{io::{self, Read}, process};

pub struct Menu;

impl Menu {
    pub fn print_menu() {
        Self::clear_terminal();
        println!("Langton's Ant");
        println!("Resolution: XxX, Hexagons: 40, Ants: 1, Rotation: L1L1R1R2NU");
        println!("1. Add/remove Ants");
        println!("2. Change rotation");
        println!("4. Set map size");
        println!("5. Exit\n");
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
            Ok(5) => {
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