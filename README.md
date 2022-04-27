# trimble - some self contained esp32s3 examples

## examples

Each directory is an entirely independent project showing how to use some feature of rust on esp32s3

### apple

Basic "hello world", but entirely in rust by using the esp-idf-sys crate and
existing support in `std` for `println` and `main`.

### blueberry

 - Use `embedded-hal` and `esp-idf-hal` to make the RGB LED on the esp32-s3-devkitC light flash.
 - Use the `log` crate to send `info!` logs to the espmonitor (instead of plain prints)

### carrot

 - Use `embedded-svc` and `esp-idf-svc` to store data to NVS and retrieve data
   from NVS (at startup)

## Building & Running these

1. Install the esp rust toolchain

```
git clone https://github.com/esp-rs/rust-build.git
./rust-build/install-rust-toolchain.sh
```

2. Install some tools:

```
cargo install cargo-espflash
cargo install ldproxy
```

3. Install `ccache` and `sccache` for caching (highly recommended), and configure them

```
# Note: package manager dependent
nix-env -i ccache sccache
```

Add this to your `~/.cargo/config.toml`

```
[build]
rustc-wrapper = "sccache"
```

4. Run `../cargo-esp espflash --monitor --release` from any of the example directories

## Crates that you may need to use to work with ESP32s3

 - `esp-idf-hal` includes rust types and functions that allow access to
   basic/typical peripherals on the esp32s3, like GPIO, SPI, etc. It impliments
   traits in `embedded-hal`. Generic code (code that doesn't want to be
   platform specific) can use `embedded-hal` traits and accept objects
   implimenting those traits as inputs.
 - `esp-idf-svc` provides
 - `esp-idf-sys`: raw bindings to functions and types in the esp-idf. This
   should be a last resort, instead use `esp-idf-svc` and `esp-idf-hal`
