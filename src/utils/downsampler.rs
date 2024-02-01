const HZ11025_BUTTER_COEF: ([f64; 2], [f64; 3]) =
    ([-0.15298021, -0.175713], [0.3321733, 0.6643466, 0.3321733]);
const HZ16000_BUTTER_COEF: ([f64; 2], [f64; 3]) =
    ([0.28094574, -0.18556054], [0.2261537, 0.4523074, 0.2261537]);
const HZ22050_BUTTER_COEF: ([f64; 2], [f64; 3]) = (
    [0.63377541, -0.24349338],
    [0.15242949, 0.30485899, 0.15242949],
);
const HZ32000_BUTTER_COEF: ([f64; 2], [f64; 3]) = (
    [0.98947218, -0.35029345],
    [0.09020532, 0.18041063, 0.09020532],
);
const HZ44100_BUTTER_COEF: ([f64; 2], [f64; 3]) = (
    [1.2381281, -0.45696268],
    [0.05470864, 0.10941729, 0.05470864],
);
const HZ48000_BUTTER_COEF: ([f64; 2], [f64; 3]) = (
    [1.29479367, -0.48534111],
    [0.04763686, 0.09527372, 0.04763686],
);
const HZ96000_BUTTER_COEF: ([f64; 2], [f64; 3]) = (
    [1.63503648, -0.69204512],
    [0.01425216, 0.02850432, 0.01425216],
);
const HZ192000_BUTTER_COEF: ([f64; 2], [f64; 3]) = (
    [1.81560286, -0.83122471],
    [0.00390546, 0.00781093, 0.00390546],
);

pub fn resample(input_format: &InputFormat, input_data: &[u8], output_buffer: &mut [u8]) {
    apply_lpf(&input_format, input_data, output_buffer);
    decimate(input_format.get_sample_rate().as_hz(), output_buffer);
}

fn apply_lpf(input_format: &InputFormat, input_data: &[u8], output_buffer: &mut [u8]) {
    let butter_coef = match get_butter_coef(input_format.get_sample_rate()) {
        Some(coef) => coef,
        None => return,
    };

    let a = butter_coef.0;
    let b = butter_coef.1;

    for m in 3..input_data.len() {
        output_buffer[m] = b[0] * input_data[m];
        for i in 1..3 {
            output_buffer[m] += a[i] * output_buffer[m - 1] + b[i] * output_buffer[m - i];
        }
    }
}

fn get_butter_coef(sample_rate: &SampleRate) -> Option<([f64; 2], [f64; 3])> {
    match sample_rate {
        SampleRate::Hz11025 => Some(HZ11025_BUTTER_COEF),
        SampleRate::Hz16000 => Some(HZ16000_BUTTER_COEF),
        SampleRate::Hz22050 => Some(HZ22050_BUTTER_COEF),
        SampleRate::Hz32000 => Some(HZ32000_BUTTER_COEF),
        SampleRate::Hz44100 => Some(HZ44100_BUTTER_COEF),
        SampleRate::Hz48000 => Some(HZ48000_BUTTER_COEF),
        SampleRate::Hz96000 => Some(HZ96000_BUTTER_COEF),
        SampleRate::Hz192000 => Some(HZ192000_BUTTER_COEF),
        SampleRate::Hz8000 => return None,
    }
}

fn get_byte_depth_endianness(input_format: &InputFormat) -> (usize, bool) {
    match input_format {
        InputFormat::Float32BigEndian(_) => (4, false),
        InputFormat::Float32LittleEndian(_) => (4, true),
        InputFormat::Float64BigEndian(_) => (8, false),
        InputFormat::Float64LittleEndian(_) => (8, true),
        InputFormat::Signed16BitBigEndian(_) => (2, false),
        InputFormat::Signed16BitLittleEndian(_) => (2, true),
        InputFormat::Signed24BitBigEndian(_) => (3, false),
        InputFormat::Signed24BitLittleEndian(_) => (3, true),
        InputFormat::Signed32BitBigEndian(_) => (4, false),
        InputFormat::Signed32BitLittleEndian(_) => (4, true),
        InputFormat::Signed8Bit(_) => (1, false),
        InputFormat::Unsigned16BitBigEndian(_) => (2, false),
        InputFormat::Unsigned16BitLittleEndian(_) => (2, true),
        InputFormat::Unsigned24BitBigEndian(_) => (3, false),
        InputFormat::Unsigned24BitLittleEndian(_) => (3, true),
        InputFormat::Unsigned32BitBigEndian(_) => (4, false),
        InputFormat::Unsigned32BitLittleEndian(_) => (4, true),
        InputFormat::Unsigned8Bit(_) => (1, false),
    }
}
