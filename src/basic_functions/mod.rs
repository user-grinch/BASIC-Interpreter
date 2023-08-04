fn print_fn(vec: &Vec<String>) {
    let str: &String = &vec.join(" ");
    print!("{}", &str);
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