# Raspberry Pi Pico UF2 Autoloader

This is a simple tool to flash a UF2 file to a Raspberry Pi Pico.

![img](./assets/app.png)


## Usage

Click the "Open File..." button to select a UF2 file to flash.
Then click the "Open Folder..." button to select a pico mount point where the UF2 file will be flashed.

The default behaviour is load the uf2 file automatically after a bootloader mode pico board is connected, you can uncheck the "autoload" checkbox to disable it. If the autoload feature is disabled, you should click the "Load" button to load the uf2 file manually.

## Build

### Linux
```shell
cargo run -r
```

### Windows
```shell
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu --release
```
