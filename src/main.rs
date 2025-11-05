fn main() {
    if let Err(e) = salinity_teos_10::adapters::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
