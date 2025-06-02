use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;

pub const WIDTH: usize = 32;
pub const HEIGHT: usize = 8;
pub const NUM_OF_LEDS: usize = WIDTH * HEIGHT;

pub struct DisplayController {
    adapter: WS28xxSpiAdapter,
    rgb_values: [(u8, u8, u8); NUM_OF_LEDS],
}

impl DisplayController {
    pub fn new() -> Self {
        let adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();
        Self {
            adapter,
            rgb_values: [(0u8, 0u8, 0u8); NUM_OF_LEDS],
        }
    }

    pub fn clear(&mut self) {
        self.adapter.clear(NUM_OF_LEDS)
    }

    pub fn display_bars(&mut self, bars: &Box<[f64]>) {
        for rgb in &mut self.rgb_values {
            *rgb = (0, 0, 0);
        }
        let max = 0.000001f64;
        let min = 0.00000001f64;

        const PATTERN: [(u8, u8, u8); 8] = [
            (12, 8, 10),
            (24, 16, 20),
            (36, 24, 30),
            (48, 32, 40),
            (60, 40, 50),
            (72, 48, 60),
            (84, 56, 70),
            (96, 64, 80),
        ];

        for (i, bar) in bars.iter().skip(bars.len() / 2).enumerate() {
            let value = (((bar - min) / (max - min)) * 8.0).clamp(0.0, 8.0) as usize;
            if i % 2 == 0 {
                for j in 0..value {
                    self.rgb_values[i * 8 + j] = PATTERN[j];
                }
            } else {
                for j in (0..value).rev() {
                    self.rgb_values[i * 8 + (7 - j)] = PATTERN[7 - j];
                }
            }
        }
        self.adapter.write_rgb(&self.rgb_values).unwrap();
    }
}
