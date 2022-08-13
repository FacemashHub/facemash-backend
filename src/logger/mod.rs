use log::{Level, LevelFilter, Metadata, Record};

struct Logger;

pub fn init() {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();

    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        // _ => LevelFilter::Off,
        _ => LevelFilter::Trace,
    });

    error!(target:"main", "this is a error");
    warn!(target:"main", "this is a warning");
    info!("this is a information");
    debug!("this is a debug");
    trace!("this is a trace");
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };

        println!(
            "\u{1B}[{}m[{:>5}]:{} - {}\u{1B}[0m",
            color,
            record.level(),
            record.target(),
            record.args(),
        );
    }

    fn flush(&self) {}
}
