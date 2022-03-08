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
