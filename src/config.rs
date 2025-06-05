pub const MATRIX_WIDTH: usize = 32;
pub const MATRIX_HEIGHT: usize = 8;

pub const SPI_BUS: &str = "/dev/spidev0.0";

pub const CONFIG_FILEPATH: &str = "spectro-rust-config";

pub const PATTERN: [(u8, u8, u8); 8] = [
    (0, 10, 0),
    (0, 10, 0),
    (5, 10, 0),
    (5, 10, 0),
    (10, 5, 0),
    (10, 5, 0),
    (10, 0, 0),
    (10, 0, 0),
];
