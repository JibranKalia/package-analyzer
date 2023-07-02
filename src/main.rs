use clap::{Parser, command};

#[derive(Parser)]
#[command(version)]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    /// The pattern to look for
    #[arg(long)]
    pattern: String,

    /// The path to the file to read
    #[arg(long)]
    path: std::path::PathBuf,
} 


fn main() {
  let args = Cli::parse();
  print!("Pattern: {}, Path: {}", args.pattern, args.path.display());

}