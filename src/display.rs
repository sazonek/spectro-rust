use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;

pub const WIDTH: usize = 32;
pub const HEIGHT: usize = 8;
pub const NUM_OF_LEDS: usize = WIDTH * HEIGHT;

pub struct DisplayController {
    adapter: WS28xxSpiAdapter,
    rgb_values: Vec<(u8, u8, u8)>,
}

impl DisplayController {
    pub fn new() -> Self {
        let adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();
        Self {
            adapter,
            rgb_values: vec![(0, 0, 0); NUM_OF_LEDS],
        }
    }

    pub fn clear(&mut self) {
        self.adapter.clear(NUM_OF_LEDS)
    }

    pub fn display_bars(&mut self, bars: &Box<[f64]>) {
        self.rgb_values = vec![(0, 0, 0); NUM_OF_LEDS];
        let max = 0.000001f64;
        let min = 0.00000001f64;

        let color = (0, 0, 10);
        for (i, bar) in bars.iter().skip(bars.len() / 2).enumerate() {
            let value = (((bar - min) / (max - min)) * 8.0).clamp(0.0, 8.0) as usize;
            if i % 2 == 0 {
                for j in 0..value {
                    if j == value - 1 {
                        self.rgb_values[i * 8 + j] = (10, 0, 0);
                    } else {
                        self.rgb_values[i * 8 + j] = color;
                    }
                }
            } else {
                for j in (0..value).rev() {
                    if j == value - 1 {
                        self.rgb_values[i * 8 + (7 - j)] = (10, 0, 0);
                    } else {
                        self.rgb_values[i * 8 + (7 - j)] = color;
                    }
                }
            }
        }
        self.adapter.write_rgb(&self.rgb_values).unwrap();
    }
}
