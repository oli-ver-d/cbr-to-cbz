use cbr_to_cbz::{extract, compress};
use clap::Parser;

#[derive(Parser)]
struct Args {
    input: String,
    output: String,
}

fn main() {
    let args = Args::parse();

    println!("Extracting {}", &args.input);
    let extracted_path = extract::extract_cbr(&args.input).unwrap();
    println!("Compressing {}", &args.output);
    compress::create_cbz(&extracted_path, &args.output).unwrap();
    println!("Successfully converted: {} -> {}", &args.input, &args.output);
}
