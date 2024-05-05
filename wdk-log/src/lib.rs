#![no_std]

extern crate alloc;

use alloc::{ffi::CString, format, string::ToString};
use log::{LevelFilter, SetLoggerError};
use wdk_sys::ntddk::DbgPrint;

static LOGGER: WdkLogger = WdkLogger;

pub struct WdkLogger;

impl WdkLogger {
    pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
    }
}

impl log::Log for WdkLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let message = format!(
                "[XX] {:<5} [{}:{}] {}\n\0",
                record.level().to_string(),
                record.target(),
                record.line().unwrap_or(0),
                record.args()
            );
            for line in message.lines() {
                let message = widestring::U16CString::from_str(line).unwrap();
                let format = CString::new("%ws\n").unwrap();

                unsafe {
                    DbgPrint(format.as_ptr() as _, message.as_ptr());
                }
            }
        }
    }

    fn flush(&self) {}
}
