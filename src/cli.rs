use clap::Parser;

#[derive(Parser)]
struct Cli {
    value: Option<usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let cli = Cli::parse();

    let _ = match cli.value {
        Some(n) => ld_::interface(n),
        None => ld_::get_pool(),
    };

    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}
