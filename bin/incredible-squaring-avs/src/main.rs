#![allow(missing_docs)]

fn main() {
    // TODO: replace this with your own API key and user ID
    let _ = eigen_telemetry::telemetry::Telemetry::set_config("@@@@@", "@@@@@");
    eigen_telemetry::telemetry::Telemetry::capture_event("started").unwrap();

    use incredible_squaring_avs::cli::Cli;

    if let Err(err) = Cli::parse_args().start() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
