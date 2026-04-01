use clap::*;

#[derive(Parser, Debug)]
#[command(version, about = "Digital Posture RegEx2", long_about = None)]
pub struct Args {
    /// Config k-v file path 
    #[arg(short, long)]
    pub config: String,

    /// Input dir path 
    #[arg(short, long)]
    pub input_dir: String,

    /// Max length to read (input files)
    #[arg(short = 'l', long, default_value = "5000")]
    pub max_len: String,

    /// Output file format [json|ndjson|csv] 
    #[arg(short = 'f', long, default_value = "json", value_parser = ["json", "ndjson", "csv"])]
    pub output_format: String,

    /// Output dir path
    #[arg(short = 'o', long, default_value = "./")]
    pub output_dir: String,
}