fn print_fn(vec: &Vec<String>) {
    let str: &String = &vec.join(" ");
    print!("{}", &str[1..str.len()-1]);
}

fn tab_fn(_: &Vec<String>) {
    print!("\t");
}

fn newline_fn(_: &Vec<String>) {
    print!("\n");
}

fn null_fn(_: &Vec<String>) {
}

fn let_fn(vec: &Vec<String>) {
    let words: Vec<&str> = vec[0].split("=").collect();
    
}

pub static BANK: phf::Map<&'static str, fn(&Vec<String>)> = phf::phf_map! {
    // io stream
    "PRINT" => print_fn,
    "NEWLINE" => newline_fn,
    "TAB" => tab_fn,
    "REM" => null_fn,

    // variables
    "LET" => let_fn,
    
    "END" => null_fn,
};