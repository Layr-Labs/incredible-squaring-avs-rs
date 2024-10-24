#![allow(missing_docs)]

fn main() {
    let _ = eigen_telemetry::telemetry::Telemetry::set_config("@@@@", "@@@@");
    eigen_telemetry::telemetry::Telemetry::capture_event("started").unwrap();

    use incredible_squaring_avs::cli::Cli;

    if let Err(err) = Cli::parse_args().start() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}

/*
    let client = posthog_rs::client("@@@@");

    let _handler = std::thread::spawn(move || {
        let event = posthog_rs::Event::new("app_started_manual", "@@@@");
        client.capture(event).unwrap();
    });
*/
