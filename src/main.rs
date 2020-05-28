mod error;
mod parser;
mod lexer;

use error::{ QccError };
use parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("引数が{}個あります。引数は2個にしてください", args.len());
        return;
    }
    if let Err(err) = parse_main(&args[1]) {
        eprintln!("{}\n{}",args[1] , err);
    }
}

fn parse_main(s: &str) -> Result<(), QccError> {
    let mut parser = Parser::new(s)?;

    println!("define i32 @main() local_unnamed_addr #0 {{");
    println!("entry:");
    parser.parse()?;
    println!("\tret i32 %{}", parser.num - 1);
    println!("}}");
    Ok(())
}


