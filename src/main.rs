use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fmt::Debug;
mod basic_functions;

const FILE_NAME : &str = "foo.txt";

#[derive(Debug)]
struct CommanadDefination {
    line: i32,
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

    fn parse_code(&mut self, file_name: &str) -> bool {
        let file = File::open(file_name);
        let reader = BufReader::new(file.unwrap());
        let mut rtn :bool = false;
    
        for t in reader.lines() {
            let line: String = t.unwrap();
            let mut words = line.split_whitespace();
            let mut collection: Vec<String> = vec![];
            let line = words.next()
                           .unwrap()
                           .parse()
                           .unwrap();
                
            let command = String::from(words.next()
                            .unwrap()); 

            for word in words {
                collection.push(word.to_string());
            }

            let cmd: CommanadDefination = CommanadDefination {
                line: line,
                command: command,
                inputs: collection,
            };
            self.commands.push(cmd);
            rtn = true;
        }
    
        return rtn;
    }
    
    fn execute_code(&self) -> bool {
        for cmd in &self.commands {
            if basic_functions::Bank.contains_key(&cmd.command) {
                basic_functions::Bank[&cmd.command](&cmd.inputs);
            } else {
                println!("Unknown command: {}", cmd.command);
            }
        }
        true
    }

    fn run(&mut self) -> bool {
        if !self.parse_code(FILE_NAME) {
            return false;
        }
        
        if !self.execute_code() {
            return false;
        }

        return true;
    }
}


fn main() -> io::Result<()> {
    let mut mgr: CodeManager = CodeManager::new();
    mgr.run();

    Ok(())
}