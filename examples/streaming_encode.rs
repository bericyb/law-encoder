use law_encoder::{
    formats::{InputFormat, OutputFormat},
    LawEncoder,
};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("audio.raw").unwrap();
    let mut audio = Vec::new();
    file.read_to_end(&mut audio).unwrap();

    let encoder = LawEncoder;

    let mut output_buffer = [0; 24];

    // Make sure chunk size is an even length for best results
    //
    for chunk in audio.chunks_exact(48) {
        let num_bytes_encoded = encoder
            .encode(
                InputFormat::BigEndian,
                &chunk,
                OutputFormat::Mulaw,
                &mut output_buffer,
            )
            .unwrap();

        // Printing bytes to simulate playing audio
        println!("{:?}", &output_buffer[..num_bytes_encoded])
    }
    return;
}
