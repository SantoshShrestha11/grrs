use clap::Parser;

//search for a pattern in a file and display the line that contain it.
#[derive(Parser)]
struct Cli {
    //the pattern to look for 
    pattern: String,
    //the path to the file to read
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, paath: {:?}", args.pattern, args.path);
}
