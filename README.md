# Chip 8 Emulator

## Chip 8 Virtual Machine implemented in Rust.

This repo contains a WIP Chip 8 emulator implemented in Rust. It's my first glance at the emulation world and my first serious Rust project.

## How to build & run

For building, just run:
```shell script
   >> cargo build [--release]
``` 

For running the project you will need a valid Chip 8 rom, or you can use the ones provided in this repo, which are provided by [**zophar**](https://www.zophar.net/pdroms/chip8.html).

```shell script
    >> cargo run -- (path-to-your-rom)
```

For example, if you want to run **Pong**:
```shell script
    >> cargo run -- ./roms/PONG
```