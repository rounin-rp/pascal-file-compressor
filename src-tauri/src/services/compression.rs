extern crate flate2;

use super::read_dir::CurrFile;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::{
    fs::{self, File},
    io::{copy, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    process::exit,
    thread,
    time::Instant,
};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub const CHUNK_SIZE: usize = 1024;
pub const THREADS: usize = 4;

#[tauri::command]
pub fn compress_file(curr_file: CurrFile) -> u128 {
    /*
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
    */
    let input_file = File::open(curr_file.path.clone()).expect("Failed to open file");
    //let output_file = File::create(curr_file.path.clone() + ".pcompressed")
    //   .expect("Failed to create output file");

    let mut handles = vec![];
    let mut intermediate_files = Vec::new();

    for i in 0..THREADS {
        let root_file_path = curr_file.path.clone();
        let output_file_name = root_file_path + i.to_string().as_str() + ".pcompressed";
        let output_file =
            File::create(output_file_name.clone()).expect("Failed to create output file");
        intermediate_files.push(output_file_name);

        let reader = BufReader::new(input_file.try_clone().expect("Failed to clone reader"));
        //let writer = BufWriter::new(output_file.try_clone().expect("Failed to clone writer"));
        let chunk_start = i * CHUNK_SIZE;
        let chunk_end = (i + 1) * CHUNK_SIZE;

        let handle = thread::spawn(move || {
            let writer = BufWriter::new(output_file);
            let result = compress_chunk(reader, writer, chunk_start, chunk_end);
            if let Err(e) = result {
                eprintln!("Error: {}", e);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Failed to join threads");
    }
    let output_path = curr_file.path.clone() + ".pcompressed";
    combine_intermediate_files(intermediate_files, output_path);
    100
}

#[tauri::command]
pub fn extract_file(curr_file: CurrFile) -> u128 {
    let start = Instant::now();
    let mut file = File::open(curr_file.path.clone()).expect("Failed to open file");
    let mut buffer = BufReader::new(&mut file);
    let mut output_filename = curr_file.path.clone();
    let output_filepath = output_filename
        .strip_suffix(curr_file.name.clone().as_str())
        .unwrap();
    let mut out_filename = output_filepath.to_string();
    out_filename.push_str("extracted_");
    out_filename.push_str(curr_file.name.strip_suffix(".pcompressed").unwrap());
    let mut decoder = GzDecoder::new(buffer);
    println!("Output filename: {}", out_filename);

    copy(
        &mut decoder,
        &mut File::create(out_filename).expect("Failed to create output file"),
    );

    start.elapsed().as_millis()
}

pub fn compress_chunk(
    reader: BufReader<File>,
    writer: BufWriter<File>,
    start: usize,
    end: usize,
) -> Result<(), String> {
    let mut encoder = GzEncoder::new(writer, Compression::default());
    let mut buffer = vec![0 as u8; end - start];
    let mut reader = reader;
    reader
        .seek(SeekFrom::Start(start as u64))
        .map_err(|e| e.to_string())?;

    loop {
        let bytes_read = reader.read(&mut buffer).map_err(|e| e.to_string())?;
        if bytes_read == 0 {
            break;
        }
        encoder
            .write_all(&buffer[0..bytes_read])
            .map_err(|e| e.to_string())?;
    }
    encoder.finish().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn combine_intermediate_files(intermediate_files: Vec<String>, output_path: String) {
    let output_file = File::create(output_path).expect("Failed to create output file");
    let mut writer = BufWriter::new(output_file);
    for file_name in intermediate_files {
        let file = File::open(file_name.clone()).expect("Failed to open file");
        let mut reader = BufReader::new(file);
        let result = copy(&mut reader, &mut writer);
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(1);
            }
        }
        fs::remove_file(file_name);
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("compression")
        .invoke_handler(tauri::generate_handler![compress_file, extract_file])
        .build()
}
