use lightning::util::logger::{Logger, Record};

#[derive(Clone)]
pub struct YourLogger;

impl YourLogger {
    pub fn new() -> Self {
        YourLogger
    }
}

impl Logger for YourLogger {
    fn log(&self, record: Record) {
        // let _raw_log = record.args.to_string();
        println!("log: {}", record.args.to_string());
        // let log = format!(
        //     "{} {:<5} [{}:{}] {}\n",
        //     OffsetDateTime::now_utc().format("%F %T"),
        //     record.level.to_string(),
        //     record.module_path,
        //     record.line,
        //     raw_log
        // );
        // <insert code to print this log and/or write this log to disk>
    }
}
