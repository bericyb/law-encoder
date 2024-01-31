use errors::EncodeError;
use input_format::InputFormat;

mod encoder;
pub mod errors;
pub mod input_format;
mod utils;

pub struct MulawEncoder {}

impl MulawEncoder {
    pub fn new() -> MulawEncoder {
        MulawEncoder {}
    }

    pub fn encode(
        &self,
        input_format: InputFormat,
        input_data: &[u8],
        output_buffer: &mut [u8],
    ) -> Result<usize, EncodeError> {
        if output_buffer.len() < (input_data.len() / 2) {
            return Err(EncodeError::OutputBufferTooSmall);
        }

        let num_bytes = encoder::encode_mulaw(input_format, input_data, output_buffer);

        Ok(num_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
