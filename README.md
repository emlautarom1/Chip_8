# Chip-8

![Logo](logo.jpg)

Simple but extensively documented Chip-8 emulator implemented in Rust. It's my first glance at the emulation world and my first serious Rust project.

Thanks to [Austin Morlan](https://austinmorlan.com/posts/chip8_emulator/) for sharing an amazing post of the Chip-8, which I used as reference for this project.

## Requirements
- Rust toolchain (rustc, cargo)

Tested on Fedora 36, rustc and cargo 1.68.2

## Building
```shell
$ cargo build [--relase]
```
*Note:* Use the optional `release` flag for maximum performance!

## Running

A valid Chip-8 ROM is required. You can use the ones in this repo, which were provided by [zophar](https://www.zophar.net/pdroms/chip8.html).

```shell script
$ cargo run -- (path-to-your-rom)
```

For example, if you want to run the classic **Pong**:

```shell script
$ cargo run -- ./roms/PONG
```
