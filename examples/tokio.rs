use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::env;
use std::io::Write;
use tempfile::NamedTempFile;
use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_multi_thread()
        .thread_name("tokio-worker")
        .max_blocking_threads(24)
        .enable_all()
        .build()
        .unwrap();
    // Get the number of files from command line args
    let args: Vec<String> = env::args().collect();
    let num_files = args
        .get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .expect("Please provide the number of files as an argument");

    // Create temporary files
    let temp_files: Vec<NamedTempFile> = (0..num_files)
        .map(|i| {
            let mut file =
                NamedTempFile::new().unwrap_or_else(|_| panic!("Failed to create temp file {}", i));
            let random_string = "Hello, world!".repeat(1000);
            file.write(random_string.as_bytes()).unwrap();
            file
        })
        .collect();
    // Store paths because we'll close the initial handles
    let paths: Vec<String> = temp_files
        .iter()
        .map(|file| file.path().to_string_lossy().to_string())
        .collect();

    // Create a barrier for synchronization

    let start_time = std::time::Instant::now();
    let futures: FuturesUnordered<_> = paths
        .iter()
        .map(|path| tokio::fs::read_to_string(path))
        .collect();
    let _: Vec<_> = rt.block_on(futures.collect());
    let elapsed = start_time.elapsed();
    // dbg!(result);
    println!("Elapsed time: {:?}", elapsed);
    // Cleanup happens automatically when NamedTempFile is dropped
}
