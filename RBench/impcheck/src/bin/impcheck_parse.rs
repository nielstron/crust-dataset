#[path = "../main_parse.rs"]
mod main_parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = main_parse::main(args.len() as i32, args);
    std::process::exit(code);
}
