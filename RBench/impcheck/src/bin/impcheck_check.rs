fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = main_check::main(args.len() as i32, args);
    std::process::exit(code);
}

#[path = "../main_check.rs"]

mod main_check;

