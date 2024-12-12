mod cli;
mod fast;

pub fn main() -> Result<(), anyhow::Error> {
    // cli::Cli::run()
    if std::env::args().any(|a| &a == "--version") {
        println!("0.12.2");
        Ok(())
    } else {
        fast::run()
    }
}
