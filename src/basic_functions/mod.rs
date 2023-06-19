use std::ops::Index;
use phf::Map;

fn print_fn(vec: &Vec<String>) {
    let str: &String = &vec.join(" ");
    println!("{}", &str[1..str.len()-1]);
}

pub static Bank: phf::Map<&'static str, fn(&Vec<String>)> = phf::phf_map! {
    "PRINT" => print_fn,
};