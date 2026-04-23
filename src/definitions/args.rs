use clap::*;

#[derive(Parser, Debug)]
#[command(version, about = "Digital Posture RegEx2", long_about = None)]
pub struct Args {
    /// Config k-v file path (rules for extracting values from file path and content). If additional cfg.txt file exists in the same directory as the rx.txt file, it will be used.
    #[arg(short, long)]
    pub config: String,

    /// Input dir path. Files in this directory will be processed according to the rules defined in the config file. Subdirectories will be NOT processed.
    #[arg(short, long)]
    pub input_dir: String,

    /// Output file format. The format ndjson is used if the input dir contains multiple files and 1 output file is created, each line is a valid json.
    #[arg(short = 'f', long, default_value = "json", value_parser = ["json", "ndjson"])]
    pub output_format: String,

    /// Output dir path. 
    #[arg(short = 'o', long, default_value = "./")]
    pub output_dir: String,
}