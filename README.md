# fujinet-hal-rust

A rust project for esp32 devices.

## pre-requisites

See the template page, as this project started from that: https://github.com/esp-rs/esp-idf-template

You should install the following cargo packages:
```shell
cargo install cargo-generate
cargo install ldproxy
cargo install espup
cargo install espflash
cargo install cargo-espflash
```

## building

There's a build.sh script to deal with cargo and copying files around (bootloader.bin):

```shell
# Do this when you first clone the project:
git submodule update --init --recursive

# See help in build script
./build.sh -h

# Clean/Build/Flash/Monitor.
./build.sh -cbfm

# Normally you can ignore the clean, and just incrementally build.
./build.sh -b
```

## Other notes of less interest

### git submodule

I add the littlefs submodule with:

```shell
git submodule add -- https://github.com/joltwallet/esp_littlefs.git components/esp_littlefs
git add -A
git commit -m 'Add esp_littlefs submodule'
```
