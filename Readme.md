# Rust on an ATTiny85

### Can it be?

## Requirements

The exact Rust nightly toolchain described in [rust-toolchain.toml](rust-toolchain.toml)

It should automatically download when running the build command for the first time.

## Build command

```shell
$ build -Z build-std=core --target ./avr-unknown-gnu-attiny85.json --release
```

- Release is required!
- target is described in the target file
- the core library has to be built for the MCU as well


## Current progress

I'm pretty sure that it sets up the Watchdog Interrupt and jumps to it, but it doesn't switch back on.

My current theory is that it segfaults or has some other issue returning from the interrupt.

I don't have a multimeter at the moment to check the voltages on the pins.
