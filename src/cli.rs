use clap::Parser;

#[derive(Parser)]
struct Cli {
    n: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let cli = Cli::parse();
    let _ = ld_::interface(cli.n);
    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}