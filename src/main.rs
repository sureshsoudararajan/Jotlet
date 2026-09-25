use jotlet::application::JotletApplication;
use jotlet::config;

fn main() {
    // Initialize logging
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("warn"),
    )
    .init();

    log::info!("Starting {} v{}", config::APP_NAME, config::APP_VERSION);

    // Check if --background flag is passed
    let is_background = std::env::args().any(|arg| arg == "--background");

    // Run the GTK application
    let exit_code = JotletApplication::run(is_background);
    std::process::exit(exit_code);
}
