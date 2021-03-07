use crate::arch;
use crate::println;

use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

static LOGGER: KernelLogger = KernelLogger;

struct KernelLogger;

pub fn init(max_level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(max_level))
}

impl log::Log for KernelLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let uptime = arch::riscv::uptime();
            let sec = uptime.as_secs();
            let ms = uptime.as_micros();
            println!(
                "[{:5}:{:0>6}][{}] {}",
                sec,
                ms,
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
