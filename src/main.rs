mod display;
use alsa::{
    Direction, ValueOr,
    pcm::{Access, Format, HwParams, PCM},
};
use cavacore::{CavaBuilder, Channels};
use display::DisplayController;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{error::Error, num::NonZero};
use std::{num::NonZeroU32, sync::Arc};

const SAMPLE_RATE: u32 = 48000;
const CHANNELS: u32 = 2;
const PERIOD_SIZE: usize = 1024;
const PERIODS: u32 = 2;

fn main() -> Result<(), Box<dyn Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("Shutting down...");
    })?;
    let mut display_controller = DisplayController::new();
    // let pcm = PCM::new("hw:0,0", Direction::Capture, false)?;
    let pcm = PCM::new("dmic_sv", Direction::Capture, false)?;
    {
        let hwp = HwParams::any(&pcm)?;
        hwp.set_channels(CHANNELS)?;
        hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest)?;
        hwp.set_format(Format::S32LE)?;
        hwp.set_access(Access::RWInterleaved)?;
        hwp.set_buffer_size(PERIOD_SIZE as i64 * PERIODS as i64)?;
        hwp.set_period_size(PERIOD_SIZE as i64, ValueOr::Nearest)?;
        hwp.set_periods(PERIODS, ValueOr::Nearest)?;
        pcm.hw_params(&hwp)?;
    }
    {
        let swp = pcm.sw_params_current()?;
        swp.set_start_threshold(0)?;
        pcm.sw_params(&swp)?;
    }
    pcm.prepare()?;

    let mut buffer = vec![0i32; PERIOD_SIZE * CHANNELS as usize];

    let mut cava = CavaBuilder::default()
        .audio_channels(Channels::Mono)
        .bars_per_channel(NonZero::new(64).unwrap())
        .enable_autosens(true)
        .frequency_range(NonZeroU32::new(50).unwrap()..NonZeroU32::new(10000).unwrap())
        .noise_reduction(0.20)
        .build()
        .unwrap();

    let mut cava_output = cava.make_output();
    let mut cava_input = vec![0f64; PERIOD_SIZE as usize];

    println!("Starting audio capture loop");
    println!("Press Ctrl+C to exit");

    while running.load(Ordering::SeqCst) {
        match pcm.io_i32()?.readi(&mut buffer) {
            Ok(frames) if frames > 0 => {
                for i in 0..cava_input.len() {
                    cava_input[i] = (buffer[i * 2] as f64) / 2147483648.0;
                }
                cava.execute(&cava_input, &mut cava_output);
                display_controller.display_bars(&cava_output);
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading from PCM: {}", e);
                break;
            }
        }
    }

    display_controller.clear();
    pcm.drain()?;
    println!("Audio capture ended");

    Ok(())
}
