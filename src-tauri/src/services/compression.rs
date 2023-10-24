extern crate flate2;

use super::read_dir::CurrFile;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[tauri::command]
pub fn compress_file(curr_file: CurrFile) -> u128 {
    let start = Instant::now();
    let mut file = File::open(curr_file.path.clone()).expect("Failed to open file");
    let mut buffer = BufReader::new(&mut file);
    let mut output_filename = curr_file.path.clone();
    output_filename.push_str(".pcompressed");
    let output = File::create(output_filename).expect("Failed to create output file");
    let mut encoder = GzEncoder::new(output, Compression::default());
    copy(&mut buffer, &mut encoder).expect("Failed to compress file");
    let output = encoder
        .finish()
        .expect("Failed to create compressed output file");
    start.elapsed().as_millis()
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("compression")
        .invoke_handler(tauri::generate_handler![compress_file])
        .build()
}
