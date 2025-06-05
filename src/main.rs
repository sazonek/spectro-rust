mod display;
use display::DisplayController;
use file_handler::init_config;
mod file_handler;
use std::error::Error;
use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
mod config;

fn main() -> Result<(), Box<dyn Error>> {
    // sigint handling
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("Shutting down...");
    })?;

    let mut display_controller = DisplayController::new();

    init_config();

    let mut child = Command::new("cava")
        .args(["-p", "spectro-rust-config"])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdout = child.stdout.take().expect("no stdout");

    println!("Starting audio visualization");
    println!("Press Ctrl+C to exit");

    const CHUNK_SIZE: usize = 32;
    let mut read_buf = [0u8; 4096];
    let mut chunk_buf = [0u8; CHUNK_SIZE * 2];
    let mut chunk_len = 0;

    while running.load(Ordering::SeqCst) {
        display_controller.display_bars();

        let n = stdout.read(&mut read_buf)?;

        chunk_buf[chunk_len..chunk_len + n].copy_from_slice(&read_buf[..n]);
        chunk_len += n;

        let mut offset = 0;
        while chunk_len - offset >= CHUNK_SIZE {
            let chunk = &chunk_buf[offset..offset + CHUNK_SIZE];
            display_controller.set_bars(chunk);
            offset += CHUNK_SIZE;
        }
        chunk_len -= offset;
        chunk_buf.copy_within(offset..offset + chunk_len, 0);
    }

    display_controller.clear();
    println!("Audio visualization ended");

    Ok(())
}
