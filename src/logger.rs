use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn init() -> Result<(), ()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}\n")))
        .build("/var/log/rpm.log")
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();
    log4rs::init_config(config).unwrap();
    Ok(())
}
