use crate::InputFormat;

const CLIP: i16 = 0x1FFF;
const BIAS: i16 = 33;

pub fn encode_mulaw(input_format: InputFormat, input: &[u8], output: &mut [u8]) -> usize {
    let mut encoded_bytes = 0;

    for chunk in input.chunks(2) {
        let sample = match input_format {
            InputFormat::LittleEndian => {
                i16::from_le_bytes(chunk.try_into().expect("Array length incorrect"))
            }
            InputFormat::BigEndian => {
                i16::from_be_bytes(chunk.try_into().expect("Array length incorrect"))
            }
        };
        output[encoded_bytes] = encode_16bit_sample(sample);
        encoded_bytes += 1;
    }

    return encoded_bytes;
}

fn encode_16bit_sample(input: i16) -> u8 {
    // Find absolute magnitude and add bias
    let mut sample = if input < 0 {
        (!input >> 2) + BIAS
    } else {
        (input >> 2) +
    };

    // Add clipping
    if sample > CLIP {
        sample = CLIP;
    }

    // Find exponent
    let mut i = sample >> 6;
    let mut segno = 1;
    while i != 0 {
        segno = segno + 1;
        i = i >> 1;
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
