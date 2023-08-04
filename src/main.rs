use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fmt::Debug;
mod basic_functions;
mod var_bank;

// Temp code for now, replace this with cmd argument later
const FILE_NAME : &str = "foo.txt";

/*
    Defination Structure
    Contains info about command specifications
*/
#[derive(Debug)]
struct CommanadDefination {
    command: String,
    inputs: Vec<String>,
}

struct CodeManager {
    commands: Vec<CommanadDefination>,
}

impl CodeManager {
    fn new () -> CodeManager {
        return CodeManager { 
            commands: vec![],
        };
    }

    /*
        Pasing the BASIC code line by line
        Our version of the language structure is this
        
        COMMAND <space> ARG1 <space> <ARG2> ... 
    */
    fn parse_code(&mut self, file_name: &str) {
        let file = File::open(file_name);
        let reader = BufReader::new(file.unwrap());
        
        for t in reader.lines() {
            let line: String = t.unwrap();
            let mut words = line.split_whitespace();
            let mut collection: Vec<String> = vec![];
                
            let command = String::from(words.next().unwrap()); 

            for word in words {
                collection.push(word.to_string());
            }

            let cmd: CommanadDefination = CommanadDefination {
                command: command,
                inputs: collection,
            };
            self.commands.push(cmd);
        }
    }
    
    /*
        Goes through the lookup table and executes 
        user commands

        Error is returned on invalid input
    */
    fn execute_code(&self) {
        for cmd in &self.commands {
            if basic_functions::BANK.contains_key(&cmd.command) {
                basic_functions::BANK[&cmd.command](&cmd.inputs);
            } else {
                println!("Unknown command: {}", cmd.command);
            }
        }
    }

    fn run(&mut self) -> bool {
        self.parse_code(FILE_NAME);
        self.execute_code();

        return true;
    }
}


fn main() -> io::Result<()> {
    let mut mgr: CodeManager = CodeManager::new();
    mgr.run();

    Ok(())
}