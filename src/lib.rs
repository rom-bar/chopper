use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::RollingFileAppender;
use log::SetLoggerError;

pub fn init_log(dev_mode: bool, file_path: &str, archive_pattern: &str) -> Result<(), SetLoggerError> {
    if let Err(e) = log4rs::init_config(get_config(dev_mode, file_path, archive_pattern)) {
        return Err(e);
    }
    Ok(())
}

fn get_config(dev_mode: bool, file_path: &str, archive_pattern: &str) -> Config {
    let level = if dev_mode {
        log::LevelFilter::Trace
    } else { log::LevelFilter::Info };

    // Policy for 5MB file size trigger roller
    // start with number 1
    // Max count 50 File
    let policy = Box::new(
        CompoundPolicy::new(
            Box::new(SizeTrigger::new(5_000_000)),
            Box::new(FixedWindowRoller::builder()
                .base(1)
                .build(archive_pattern, 50)
                .unwrap()
            ),
        ));
    // Logging to log file.
    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::default()))
        .build(file_path, policy)
        .unwrap();

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    let builder = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        );
    //Combine logger
    let root_builder = Root::builder()
        .appender("logfile")
        .appender("stderr");

    builder.build(
        root_builder
            .build(level),
    )
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::init_log;

    #[test]
    fn it_works() {
        let result = init_log(true,"log/file.log","archive/file.{}.log");
        assert!(result.is_ok());
    }
}