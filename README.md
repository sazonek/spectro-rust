## Info

Project built for audio visualization on raspberry pi with WS28xx RGB LED matrices. Matrix has to be connected on SPI bus. Audio is read on default from I2S microphone through custom alsa config as specified [here](https://cdn-learn.adafruit.com/downloads/pdf/adafruit-i2s-mems-microphone-breakout.pdf). Connecting the matrix is specified [here](https://github.com/phip1611/ws2818-rgb-led-spi-driver). There are some plans for more compatibility (maybe). Built as part of class at NCU in Toruń.

## Installation

```
git pull https://github.com/sazonek/spectro-rust.git
cd spectro-rust
cargo install spectro-rust --path .
```

## Uninstallation

```
cargo uninstall spectro-rust
```

## Credits

- me (lol)
- [bwisniewski26](https://github.com/bwisniewski26)
- [krasnykid](https://github.com/krasnykid)
- [monolopoly](https://github.com/monolopoly)
