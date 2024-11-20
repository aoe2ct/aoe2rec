use clap::Parser;

#[derive(Parser)]
struct Args {
    // Path to the aoe2record file
    #[arg(short, long)]
    file: std::path::PathBuf,
}
fn main() {
    let args = Args::parse();
    if !args.file.is_file() {
        println!("File not found");
        std::process::exit(-1);
    }
    let parsed_header = aoe2rec::Savegame::from_file(&args.file).unwrap();
    println!("{}", serde_json::to_string_pretty(&parsed_header).unwrap());
}
