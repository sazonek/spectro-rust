use crate::config::{MATRIX_HEIGHT, MATRIX_WIDTH, PATTERN, SPI_BUS};
use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

const NUM_OF_LEDS: usize = MATRIX_WIDTH * MATRIX_HEIGHT;

pub struct DisplayController {
    adapter: WS28xxSpiAdapter,
    encoded_rgb_values: Vec<u8>,
}

impl DisplayController {
    pub fn new() -> Self {
        let adapter = WS28xxSpiAdapter::new(SPI_BUS).unwrap();
        Self {
            adapter,
            encoded_rgb_values: Vec::with_capacity(encode_rgb(0, 0, 0).len() * NUM_OF_LEDS),
        }
    }

    pub fn clear(&mut self) {
        self.adapter.clear(NUM_OF_LEDS)
    }

    pub fn set_bars(&mut self, bars: &[u8]) {
        self.encoded_rgb_values.clear();

        for (i, bar) in bars.iter().rev().enumerate() {
            let bar = *bar as usize;
            let value = (bar * 8 + 127) / 255;

            if i % 2 == 0 {
                for j in 0..MATRIX_HEIGHT {
                    if j < value {
                        self.encoded_rgb_values.extend_from_slice(&encode_rgb(
                            PATTERN[j].0,
                            PATTERN[j].1,
                            PATTERN[j].2,
                        ));
                    } else {
                        self.encoded_rgb_values
                            .extend_from_slice(&encode_rgb(0, 0, 0));
                    }
                }
            } else {
                for j in (0..MATRIX_HEIGHT).rev() {
                    if j < value {
                        self.encoded_rgb_values.extend_from_slice(&encode_rgb(
                            PATTERN[j].0,
                            PATTERN[j].1,
                            PATTERN[j].2,
                        ));
                    } else {
                        self.encoded_rgb_values
                            .extend_from_slice(&encode_rgb(0, 0, 0));
                    }
                }
            }
        }
    }
    pub fn display_bars(&mut self) {
        self.adapter
            .write_encoded_rgb(&self.encoded_rgb_values)
            .unwrap();
    }
}
