use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::time::Instant;

fn main() {
    let file_path = "./speed_test.txt";
    let file_size = 1024 * 1024 * 1024; // 1 GB
    let block_size = 1024 * 1024; // 1 MB
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();
    // Write test data to the file
    let start_time = Instant::now();
    for i in 0..file_size / block_size {
        let data = vec![i as u8; block_size];
        file.write_all(&data).unwrap();
    }
    let write_time = start_time.elapsed();
    // Read test data from the file
    let start_time = Instant::now();
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut buffer = vec![0; block_size];
    for _ in 0..file_size / block_size {
        file.read_exact(&mut buffer).unwrap();
    }
    let read_time = start_time.elapsed();
    // Print the results
    println!(
        "Write speed: {:.2} MB/s",
        (file_size as f64 / write_time.as_secs_f64()) / (1024.0 * 1024.0)
    );
    println!(
        "Read speed: {:.2} MB/s",
        (file_size as f64 / read_time.as_secs_f64()) / (1024.0 * 1024.0)
    );
    // Delete the test file
    std::fs::remove_file(file_path).unwrap();
}

// TODO add benchmark
