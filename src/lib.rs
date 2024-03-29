#![no_std]

pub use crate::errors::EncodeError;
pub use crate::formats::{InputFormat, OutputFormat};

mod encoder;
pub mod errors;
pub mod formats;

pub struct LawEncoder;

impl LawEncoder {
    ///Encodes audio data from one format to another, writing the encoded data into the provided output buffer.
    ///
    ///#### Parameters:
    ///
    ///- `input_format`: An `InputFormat` enum specifying the format of the input data.
    ///- `input_data`: A slice of `u8` representing the audio data to be encoded. This data should conform to the format specified by `input_format`.
    ///- `output_format`: An `OutputFormat` enum specifying the desired format of the output data.
    ///- `output_buffer`: A mutable slice of `u8` where the encoded data will be stored. The buffer must be large enough to hold the encoded data; otherwise, an error is returned.
    ///#### Returns:
    ///- A `Result<usize, EncodeError>` indicating the outcome of the encoding operation. On success, it returns `Ok(num_bytes)`, where `num_bytes` is the number of bytes written to `output_buffer`. On failure, it returns `Err(EncodeError)`, indicating the nature of the error.
    ///#### Errors:
    ///- `EncodeError::OutputBufferTooSmall`: This error indicates that the provided `output_buffer` is not large enough to contain the encoded data. The size of the output buffer must be at least half the size of the input data, reflecting the specific encoding algorithm's requirements.
    ///#### Example Usage:
    ///```rust
    ///use law_encoder::{InputFormat, OutputFormat, LawEncoder};
    ///let input_data = vec![/* input data bytes */];
    ///let mut output_buffer = vec![0u8; /* appropriate size */ 12];
    ///let encoder = LawEncoder;
    ///match encoder.encode(InputFormat::BigEndian, &input_data, OutputFormat::Alaw, &mut output_buffer) {
    ///    Ok(num_bytes) => println!("Encoded {} bytes successfully.", num_bytes),
    ///    Err(e) => println!("Encoding failed: {:?}", e),
    ///}
    ///```
    ///
    ///#### Notes:
    ///
    ///- The exact size requirement for `output_buffer` may vary depending on the input and output formats. It is generally recommended to allocate the output buffer with at least half the size of the input data to accommodate the encoded data.
    pub fn encode(
        &self,
        input_format: InputFormat,
        input_data: &[u8],
        output_format: OutputFormat,
        output_buffer: &mut [u8],
    ) -> Result<usize, EncodeError> {
        if output_buffer.len() < (input_data.len() / 2) {
            return Err(EncodeError::OutputBufferTooSmall);
        }

        let num_bytes = encoder::encode(input_format, input_data, output_format, output_buffer);

        Ok(num_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> LawEncoder {
        LawEncoder {}
    }

    #[test]
    fn num_encoded_bytes() {
        let encoder = setup();

        let bytes: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut output: [u8; 5] = [0; 5];

        let num_encoded = encoder
            .encode(
                InputFormat::BigEndian,
                &bytes,
                OutputFormat::Mulaw,
                &mut output,
            )
            .unwrap();
        assert_eq!(num_encoded, 5);

        let empty = [0; 0];

        let num_encoded = encoder
            .encode(
                InputFormat::BigEndian,
                &empty,
                OutputFormat::Mulaw,
                &mut output,
            )
            .unwrap();
        assert_eq!(num_encoded, 0);
    }

    #[test]
    fn correct_encoded_bytes_big_endian() {
        let encoder = setup();

        // 1000 and -1000 divided across 1 byte segments in big endian
        let bytes: [u8; 4] = [0b00000011, 0b11101000, 0b11111100, 0b00011000];
        let mut output: [u8; 2] = [0; 2];

        let _num_encoded = encoder
            .encode(
                InputFormat::BigEndian,
                &bytes,
                OutputFormat::Mulaw,
                &mut output,
            )
            .unwrap();
        assert_eq!(output, [206, 78]);

        let _num_encoded = encoder
            .encode(
                InputFormat::BigEndian,
                &bytes,
                OutputFormat::Alaw,
                &mut output,
            )
            .unwrap();
        assert_eq!(output, [250, 122])
    }

    #[test]
    fn correct_encoded_bytes_little_endian() {
        let encoder = setup();

        // 1000 and -1000 divided across 1 byte segments in little endian
        let bytes: [u8; 4] = [0b11101000, 0b00000011, 0b00011000, 0b11111100];
        let mut output: [u8; 2] = [0; 2];

        let _num_encoded = encoder
            .encode(
                InputFormat::LittleEndian,
                &bytes,
                OutputFormat::Mulaw,
                &mut output,
            )
            .unwrap();
        assert_eq!(output, [206, 78]);

        let _num_encoded = encoder
            .encode(
                InputFormat::LittleEndian,
                &bytes,
                OutputFormat::Alaw,
                &mut output,
            )
            .unwrap();
        assert_eq!(output, [250, 122]);
    }

    #[test]
    fn output_buffer_error() {
        let encoder = setup();

        let bytes: [u8; 20] = [0; 20];
        let mut output_buffer: [u8; 2] = [0; 2];

        let result = encoder.encode(
            InputFormat::BigEndian,
            &bytes,
            OutputFormat::Mulaw,
            &mut output_buffer,
        );

        assert!(result.is_err(), "Expected an error for ouput buffer size");
    }

    #[test]
    fn odd_length_input() {
        let encoder = setup();

        let bytes: [u8; 3] = [0b00000011, 0b11101000, 0b11111100];
        let mut output: [u8; 2] = [0; 2];

        let num_encoded_bytes = encoder
            .encode(
                InputFormat::BigEndian,
                &bytes,
                OutputFormat::Mulaw,
                &mut output,
            )
            .unwrap();
        assert_eq!(num_encoded_bytes, 1);
        assert_eq!(output[0], 206);
        assert_eq!(output[1], 0);
    }
}
