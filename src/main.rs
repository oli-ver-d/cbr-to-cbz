use cbr_to_cbz::{extract, compress};

fn main() {
    println!("CBR to CBZ.");
    // set up a clap program that takes a string for in path and a string for out path
    // call lib to extract and then compress
    // return the out file path + success msg

    // e
    let cbr_path = "placeholder";
    let out_path = "placeholder";

    let extracted_path = extract::extract_cbr(cbr_path).unwrap();
    compress::create_cbz(&extracted_path, out_path).unwrap();
}
