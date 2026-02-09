#[path = "../main_confirm.rs"]
mod main_confirm;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = main_confirm::main(args.len() as i32, args);
    std::process::exit(code);
}
