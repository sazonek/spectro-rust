## Overview

An audio visualizer for Raspberry Pi using WS28xx RGB LED matrices.

Features audio input from an I2S MEMS microphone via a custom ALSA [configuration](https://cdn-learn.adafruit.com/downloads/pdf/adafruit-i2s-mems-microphone-breakout.pdf). See matrix wiring [information](https://github.com/phip1611/ws2818-rgb-led-spi-driver) for setup details.
Uses the great [Cava](https://github.com/karlstav/cava) software for audio visualization.

Built during coursework at NCU Toruń.

## Installation
Requires cava to be installed on the system.
### Option 1: Direct installation on Pi

```sh
git clone https://github.com/sazonek/spectro-rust.git
cd spectro-rust
cargo install spectro-rust --path .
```

### Option 2: Cross-compilation (Recommended)

Compile using [cross](https://github.com/cross-rs/cross) with the supplied Docker image:

```sh
cross build --target aarch64-unknown-linux-gnu --release
```

Then copy the binary to your Pi:

```sh
scp target/aarch64-unknown-linux-gnu/release/spectro-rust pi@raspberrypi.local:/home/pi/
```

## Usage

After installation, run the visualizer:

```sh
./spectro-rust
```

Config file should be placed in the same directory as the binary.

## Uninstallation

If installed via cargo:

```sh
cargo uninstall spectro-rust
```

## Credits

- [sazonek](https://github.com/sazonek)
- [bwisniewski26](https://github.com/bwisniewski26)
- [krasnykid](https://github.com/krasnykid)
- [monolopoly](https://github.com/monolopoly)
