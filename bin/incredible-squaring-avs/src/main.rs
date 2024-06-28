#![allow(missing_docs)]

fn main() {
    use incredible_squaring_avs::cli::Cli;

    if let Err(err) = Cli::parse_args().start() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
