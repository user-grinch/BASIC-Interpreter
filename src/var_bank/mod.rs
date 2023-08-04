use std::{any::{Any, TypeId}, error::Error};
use std::fmt;

#[derive(Debug)]
struct VarInfo {
    name: String,
    value: Box<dyn Any>
}

#[derive(Debug)]
struct InvalidMemoryError(String);

impl fmt::Display for InvalidMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid memory error: {}", self.0)
    }
}

impl Error for InvalidMemoryError {}

struct VarBank {
    store : Vec<VarInfo>
}

impl VarBank {
    /*
        Returns existing value
        NULL is retured for non existent
    */
    pub fn get(&mut self, name: String, val: String) -> Result<&dyn Any, Box<dyn Error>> {

        // Look for existing vars
        for var in self.store.iter_mut().enumerate() {
            if var.1.name == name {
                var.1.value = Box::new(val);
                println!("Found existing var with var: {:?}", var);
                return Ok(var.1.value.as_ref());
            }
        }

        return Err(Box::new(InvalidMemoryError("Var not found ".into())));
    }

    /*
        Pushes the var into the buffer
        Existing value gets overwritten
    */
    pub fn set(&mut self, name: String, val: String) {

        // Look for existing vars
        for var in self.store.iter_mut().enumerate() {
            if var.1.name == name {
                var.1.value = Box::new(val);
                println!("Set existing var with var: {:?}", var);
                return;
            }
        }
        self.store.push(VarInfo { 
            name : name, value: Box::new(val),
        });
        println!("Setup new var name: {} value: {}", name, val);
    }}

pub static BANK: VarBank = VarBank::new();