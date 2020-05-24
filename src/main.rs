fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("引数が{}個あります。引数は2個にしてください", args.len());
        return;
    }
    if let Ok(i) = args[1].parse::<i32>() {
        println!("define i32 @main() local_unnamed_addr #0 {{");
        println!("entry:");
        println!("\tret i32 {}", i);
        println!("}}");
    } else {
        eprintln!("{}は数値ではありません", args[1]);
    }
}
