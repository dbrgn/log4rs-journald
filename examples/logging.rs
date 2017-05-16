#[macro_use] extern crate log;
extern crate log4rs;
extern crate log4rs_journald;

use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs_journald::JournalAppender;

fn main() {
    let stdout = ConsoleAppender::builder().build();
    let journal = JournalAppender::new();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("journal", Box::new(journal)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("journal")
                .build(LogLevelFilter::Debug)
        ).unwrap();
    log4rs::init_config(config).unwrap();

    info!("An information.");
    warn!("A warning!");
    error!("Oh shitshit...");
}
