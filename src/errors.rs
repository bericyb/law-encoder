use core::fmt;

#[derive(Debug)]
pub enum EncodeError {
    OutputBufferTooSmall,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodeError::OutputBufferTooSmall => write!(
                f,
                "Output buffer is too small. Must be at least 1/2 of input size."
            ),
        }
    }
}
