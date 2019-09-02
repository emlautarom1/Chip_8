![Logo](logo.jpg)

# Chip 8 Virtual Machine implemented in Rust.

This repo contains a WIP Chip 8 emulator implemented in Rust. It's my first glance at the emulation world and my first serious Rust project.

Thanks to **[Austin Morlan](https://austinmorlan.com/posts/chip8_emulator/)** for sharing an amazing post of the Chip 8, which I used as reference for this project.

## How to build & run this project:

### Building:
For building, just run:
```shell script
   >> cargo build [--release]
``` 
*Note:* Use the optional `release` flag for maximum performance!. 


### Running:
For running the project you will need a valid Chip 8 rom, or you can use the ones in this repo, which are provided by [**zophar**](https://www.zophar.net/pdroms/chip8.html).

```shell script
    >> cargo run -- (path-to-your-rom)
```

For example, if you want to run the classic **Pong**:
```shell script
    >> cargo run -- ./roms/PONG
```