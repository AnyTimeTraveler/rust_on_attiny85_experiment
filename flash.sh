avr-objcopy -O ihex target/avr-unknown-gnu-attiny85/release/blink.elf blink.hex

sudo tools/launcher -cdigispark --timeout 60 -Uflash:w:blink.hex:h
