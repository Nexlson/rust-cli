use clap::Parser;
use std::process::Command;
use std::env;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// script to run
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let filename_path = format!("/src/scripts/{}.sh", args.file);
    println!("Running scripts from {}", filename_path);


    // get current DIR
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let dir_string = current_dir.clone().into_os_string().into_string().expect("Failed to convert PathBuf to String");

    // run bash file
    let output = Command::new("bash")
        .arg(dir_string + &filename_path)
        .output()
        .expect("Failed to run Bash scripts");
    if output.status.success(){
        println!("Done executing your script!")
    }else {
        println!("Failed to execute your script!");
    }
}