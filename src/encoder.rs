use crate::InputFormat;
use crate::OutputFormat;

const CLIP: i16 = 0x1FFF;
const BIAS: i16 = 33;

pub fn encode(
    input_format: InputFormat,
    input: &[u8],
    output_format: OutputFormat,
    output: &mut [u8],
) -> usize {
    let mut encoded_bytes = 0;

    for chunk in input.chunks(2) {
        let sample = match input_format {
            InputFormat::LittleEndian => {
                let bytes = if chunk.len() == 2 {
                    chunk.try_into().unwrap()
                } else {
                    break;
                };
                i16::from_le_bytes(bytes)
            }
            InputFormat::BigEndian => {
                let bytes = if chunk.len() == 2 {
                    chunk.try_into().unwrap()
                } else {
                    break;
                };
                i16::from_be_bytes(bytes)
            }
        };
        output[encoded_bytes] = match output_format {
            OutputFormat::Mulaw => mulaw_encode_sample(sample),
            OutputFormat::Alaw => alaw_encode_sample(sample),
        };
        encoded_bytes += 1;
    }
    encoded_bytes
}

fn alaw_encode_sample(input: i16) -> u8 {
    // Find absolute magnitude
    let mut sample = if input < 0 { !input >> 4 } else { input >> 4 };

    // If large enough amplitude find exponent
    if sample > 15 {
        let mut exp_pos = 1;
        while sample > 31 {
            sample >>= 1;
            exp_pos += 1;
        }

        // Remove leading 1
        sample -= 16;

        // Compute encoded value
        sample += exp_pos << 4;
    }

    // Add back in sign
    if input >= 0 {
        sample |= 0x0080;
    }

    // Toggle even bits
    (sample ^ 0x0055) as u8
}

fn mulaw_encode_sample(input: i16) -> u8 {
    // Find absolute magnitude and add bias
    let mut sample = if input < 0 {
        (!input >> 2) + BIAS
    } else {
        (input >> 2) + BIAS
    };

    // Add clipping
    if sample > CLIP {
        sample = CLIP;
    }

    // Find exponent
    let mut i = sample >> 6;
    let mut segno = 1;
    while i != 0 {
        segno += 1;
        i >>= 1;
    }

    // high-nibble
    let high_nibble = (0x0008) - segno;

    // low-nibble
    let low_nibble = (0x000F) - ((sample >> segno) & (0x000F));

    // Join nibbles together
    if input >= 0 {
        ((high_nibble << 4) | low_nibble | (0x0080)) as u8
    } else {
        ((high_nibble << 4) | low_nibble) as u8
    }
}
