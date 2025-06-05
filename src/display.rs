use crate::config::{MATRIX_HEIGHT, MATRIX_WIDTH, PATTERN, SPI_BUS};
use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;

const NUM_OF_LEDS: usize = MATRIX_WIDTH * MATRIX_HEIGHT;

pub struct DisplayController {
    adapter: WS28xxSpiAdapter,
    rgb_values: [(u8, u8, u8); NUM_OF_LEDS],
}

impl DisplayController {
    pub fn new() -> Self {
        let adapter = WS28xxSpiAdapter::new(SPI_BUS).unwrap();
        Self {
            adapter,
            rgb_values: [(0u8, 0u8, 0u8); NUM_OF_LEDS],
        }
    }

    pub fn clear(&mut self) {
        self.adapter.clear(NUM_OF_LEDS)
    }
    pub fn set_bars(&mut self, bars: &[u8]) {
        for rgb in &mut self.rgb_values {
            *rgb = (0, 0, 0);
        }

        for (i, bar) in bars.iter().rev().enumerate() {
            let bar = *bar as usize;
            let value = (bar * 8 + 127) / 255;

            if i % 2 == 0 {
                for j in 0..value {
                    self.rgb_values[i * 8 + j] = PATTERN[j];
                }
            } else {
                for j in (0..value).rev() {
                    self.rgb_values[i * 8 + (7 - j)] = PATTERN[j];
                }
            }
        }
    }
    pub fn display_bars(&mut self) {
        self.adapter.write_rgb(&self.rgb_values).unwrap();
    }
}
