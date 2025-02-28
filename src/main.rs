use std::env;
use futures::future::join_all;
use tempfile::NamedTempFile;

#[tokio::main]
async fn main() {
    // Get the number of files from command line args
    let args: Vec<String> = env::args().collect();
    let num_files = args.get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .expect("Please provide the number of files as an argument");

    // Create temporary files
    let temp_files: Vec<NamedTempFile> = (0..num_files)
        .map(|i| {
            NamedTempFile::new()
                .unwrap_or_else(|_| panic!("Failed to create temp file {}", i))
        })
        .collect();

    // Store paths because we'll close the initial handles
    let paths: Vec<String> = temp_files
        .iter()
        .map(|file| file.path().to_string_lossy().to_string())
        .collect();

    // Create a barrier for synchronization
    
    let start_time = std::time::Instant::now();
    let mut handles = vec![];

    // Spawn tasks to open files
    for path in paths {
        let handle = tokio::fs::File::open(path);
        handles.push(handle);
    }
    join_all(handles).await;

    let elapsed = start_time.elapsed();
    println!("Total execution time: {:?}", elapsed);

    // Cleanup happens automatically when NamedTempFile is dropped
}
