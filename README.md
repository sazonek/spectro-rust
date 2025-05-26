## overview

audio visualizer for raspberry pi using ws28xx rgb led matrices over spi.
input from an i2s mems mic via a custom alsa [config](https://cdn-learn.adafruit.com/downloads/pdf/adafruit-i2s-mems-microphone-breakout.pdf). matrix wiring [info](https://github.com/phip1611/ws2818-rgb-led-spi-driver).
built during coursework at ncu toruń. future support for other setups is a maybe.

## install

directly on pi:

```sh
git clone https://github.com/sazonek/spectro-rust.git
cd spectro-rust
cargo install spectro-rust --path .
```

although recommended to compile through [cross](https://github.com/cross-rs/cross) with supplied docker image. you'll figure it out.

## uninstall

```sh
cargo uninstall spectro-rust
```

## credits

- [sazonek](https://github.com/sazonek)
- [bwisniewski26](https://github.com/bwisniewski26)
- [krasnykid](https://github.com/krasnykid)
- [monolopoly](https://github.com/monolopoly)
