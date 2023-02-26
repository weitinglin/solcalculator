#![allow(unused)]
use clap::Parser;

//#[derive(Parser)]
//struct Cli {
//    pattern: String,
//    path: std::path::PathBuf,
//}

fn main() {
    let first = std::env::args().nth(1).expect("no pattern given");
    let second = std::env::args().nth(2).expect("no path given");
    let first_num = first.parse::<i32>().expect("not a number");
    let second_num = second.parse::<i32>().expect("not a number");  
    println!("This is your calculation: {} {}", &first, &second);
    println!("The result is: {}", first_num + &second_num);
    //let args = Cli::parse();
}
