use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

struct Logger;

static LOGGER: Logger = Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color: u8 = match record.level() {
                Level::Info => 94,
                Level::Warn => 93,
                Level::Error => 91,
                _ => 97,
            };
            let content = record.args().to_string();
            let args: Vec<&str> = content.split("\n").collect();

            let f_args = args
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, s)| {
                    if i == 0 {
                        if args.len() > 1 {
                            format!("┬ {s}\n")
                        } else {
                            format!("- {s}\n")
                        }
                    } else if i == args.len() - 1 {
                        format!("     ╰ {s}\n")
                    } else {
                        format!("     | {s}\n")
                    }
                })
                .collect::<Vec<String>>()
                .join("");

            let level = record.level().to_string();

            println!("\x1b[{};1m{}\x1b[0m {}", color, &level[0..4], f_args);
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
