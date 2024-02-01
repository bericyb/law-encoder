use std::fs::File;
use std::io::{Read, Write};

use law_encoder::{InputFormat, LawEncoder, OutputFormat};

fn main() {
    let mut file = File::open("audio.raw").unwrap();
    let mut audio = Vec::new();
    file.read_to_end(&mut audio).unwrap();

    let encoder = LawEncoder {};

    let mut output = [0; 150000];

    let num_bytes_encoded = encoder
        .encode(
            InputFormat::BigEndian,
            &audio,
            OutputFormat::Mulaw,
            &mut output,
        )
        .unwrap();

    println!("Encoded {} bytes!", num_bytes_encoded);

    let final_out = &output[0..num_bytes_encoded];

    let mut output_file = File::create("encoded.ulaw").unwrap();
    output_file.write_all(&final_out).unwrap();
}
