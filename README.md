# ms_to_secs

ms_to_secs is a simple Rust library that converts milliseconds to seconds as an f64 value. This crate is designed for quick, lightweight time conversions, making it useful for time-based calculations in various applications.

## Installation

To use ms_to_secs, add it to your Cargo.toml:

[dependencies]
ms_to_secs = "0.1.0"

use ms_to_secs::ms_to_secs;

fn main() {
    let seconds = ms_to_secs(1500);
    println!("1500 milliseconds is {} seconds", seconds); // Outputs: 1.5 seconds
}

## Features

Converts milliseconds (u64) to seconds as a f64 value
Lightweight and easy to use

## License

This project is licensed under the MIT License.  
