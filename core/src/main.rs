mod codec;
mod crypto;
mod model;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("  encode <input_json_file> <output_file>");
        eprintln!("  decode <input_file> <output_json_file>");
        return;
    }

    match args[1].as_str() {
        "encode" => {
            let input_path = &args[2];
            let output_path = &args[3];
            codec::encode_file(input_path, output_path).unwrap();
            println!("Encoded successfully.");
        }
        "decode" => {
            let input_path = &args[2];
            let output_path = &args[3];
            codec::decode_file(input_path, output_path).unwrap();
            println!("Decoded successfully.");
        }
        _ => {
            eprintln!("Unknown command.");
        }
    }
}
