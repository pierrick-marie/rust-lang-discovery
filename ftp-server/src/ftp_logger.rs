use log::{Level, Record, Metadata, SetLoggerError, LevelFilter};

struct SimpleLogger;

const MAX_LEVEL: Level = Level::Trace;
const MAX_FILTER_LEVEL:LevelFilter = LevelFilter::Trace;

impl log::Log for SimpleLogger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= MAX_LEVEL
	}
	
	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			println!("#{}: {}", record.level(), record.args());
		}
	}
	
	fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
	log::set_logger(&LOGGER)
		.map(|()| log::set_max_level(MAX_FILTER_LEVEL))
}