use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use tokio::time::Instant;

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;

    return Ok(io::BufReader::new(file).lines());
}

async fn line_count(file_name: String) -> anyhow::Result<usize> {
    use tokio::fs::File;
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;
    let mut line_count = 0;
    let file = File::open(file_name).await?;

    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            line_count += 1;
        }
    }
    
    return Ok(line_count);
}
fn main() {
    println!("Hello, world!");
}
