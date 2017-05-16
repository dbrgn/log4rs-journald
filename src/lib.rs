//! A systemd journal appender.
 
extern crate log;
extern crate log4rs;
extern crate systemd;

use std::error::Error;

use log::LogRecord;
use log4rs::append::Append;
use systemd::journal;

/// An appender which logs to the systemd journal.
#[derive(Debug)]
pub struct JournalAppender;


impl JournalAppender {
    pub fn new() -> Self {
        JournalAppender {}
    }
}

impl Append for JournalAppender {
    fn append(&self, record: &LogRecord) -> Result<(), Box<Error + Sync + Send>> {
        journal::log_record(record);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
