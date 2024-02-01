# law-encoder

⚖️`️‍️law-encoder`👨‍⚖ is a Rust library for A-law and μ-law (mu-law) audio encoding. These encoding schemes are defined in ITU-T standards G.711 and are widely used in digital audio processing, particularly in telephony and audio compression.

## Features

- **A-law & μ-law Encoding**: Compand 16bit audio data using the A-law and μ-law algorithm.
- **High Performance**: No dynamic memory allocations. Optimized for real-time and embedded applications.
- **Zero Dependencies**: Designed to work standalone, with no external dependencies. No standard library. Just good ole bit shifting.

### Installation

Add `law-encoder` to your `Cargo.toml`:

```toml
[dependencies]
law_encoding = "0.1.0"
```

### Usage

Here's a quick example to get you started with `law-encoder`:

```rust
use law_encoder::{LawEncoder, InputFormat, OutputFormat};

let encoder = LawEncoder;

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
```

More detailed examples in the examples directory.

## Contributing

Contributions are welcome! Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the [MIT License](LICENSE) - see the LICENSE file for details.

## Acknowledgments

- This project was developed with the specifications in ITU-T G.711 standards for audio compression.

## Support

For support and inquiries, please open an issue on the GitHub repository issue tracker.
