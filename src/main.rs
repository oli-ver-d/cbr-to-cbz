use cbr_to_cbz::{extract, compress};
use clap::Parser;
use std::fs::remove_file;

#[derive(Parser)]
struct Args {
    /// Path to CBR file
    input: String,
    /// Path to CBZ file
    output: String,
    /// Flag to delete CBR file after conversion
    #[arg(short, long)]
    delete: bool,
}

fn main() {
    let args = Args::parse();

    println!("Extracting {}", &args.input);
    let extracted_path = extract::extract_cbr(&args.input).unwrap();
    if args.delete {
        remove_file(&args.input).expect("Failed to delete cbr file after extract.");
        println!("Removed {}", &args.input);
    }
    println!("Compressing {}", &args.output);
    compress::create_cbz(&extracted_path, &args.output).unwrap();
    println!("Successfully converted: {} -> {}", &args.input, &args.output);
}
