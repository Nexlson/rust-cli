use clap::Parser;
use std::process::Command;
use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// script to run
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let filename_path = format!("{}.sh", args.file);
    println!("Running scripts from {}", filename_path);

    // get current DIR
    let home_dir = env::var("HOME").ok().map(PathBuf::from);
    let home_dir_string = home_dir.expect("failed").to_string_lossy().to_string();

    // run bash file
    let output = Command::new("bash")
        .arg(home_dir_string + "/rust-cli/src/scripts/" + &filename_path)
        .output()
        .expect("Failed to run Bash scripts");
    if output.status.success(){
        println!("Done executing your script!")
    }else {
        println!("Failed to execute your script!");
    }
}