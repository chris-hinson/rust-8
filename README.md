# Rust-8

chip8 emulator written in rust, using rust-sdl2 for graphics.

### Features

Fully implemented all 35 original chip8 opcodes.
This emulator does NOT support chip8 variants such as super-chip8, chip48 or xo-chip (may be implemented in a future release)

### Building
pre-requisites: ```libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-ttf-dev ```
in root dir: ```cargo build --release```
the executable will be at ./target/release/rust-8

### Running 
binary expects the rom's .ch8 file to be in the same directory as it
invoke using: ```./rust-8 romname.ch8```

### Binaries
sdl throws a million errors when trying to build a portable executable, so pre-built binaries will be included in a future release
