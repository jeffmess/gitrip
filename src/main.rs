use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
struct Cli {
    /// Removes branches merged into this branch
    #[clap(short, long, value_parser)]
    branch: String    
}

fn main() {
    // let branch = std::env::args().nth(1).expect("no pattern given");

    let args = Cli::parse();
    println!("{:?}", args);

    let branches = Command::new("git")
                           .arg("branch")
                           .arg(args.branch)
                           .output()
                           .expect("failed to execute process");

    let output = branches.stdout;
    println!("{:?}", output);
}
