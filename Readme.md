# Rust on an ATTiny85

### Can it be?

## Requirements

The exact Rust nightly toolchain described in [rust-toolchain.toml](rust-toolchain.toml)

It should automatically download when running the build command for the first time.

## Build command

```shell
$ cargo build -Z build-std=core --target ./avr-unknown-gnu-attiny85.json --release
```

- Release is required!
- target is described in the target file
- the core library has to be built for the MCU as well

## Flashing

Run this script and plug in the ATTiny85 when it ONLY tells you to.
```shell
$ ./flash.sh
```

## Current progress

Current code is untested, because I forgot the chip in the lab.

The current code should toggle the on-chip LED regularly.
I don't know the interval, as I haven't researched the clock speed.
I do think that it toggle every 3.5 seconds, if the chip clock is 10 MHz.
