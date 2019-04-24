use chrono::{DateTime, Utc};
use crossbeam::channel::{self, Sender};
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::string::ToString;
use std::thread;

#[derive(Copy, Clone)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogLevel::Info => "INFO",
                LogLevel::Warning => "WARNING",
                LogLevel::Error => "ERROR",
            }
        )
    }
}

pub trait Context: Send {
    fn context_string(&self) -> String;
}

struct LogMessage<T: Context> {
    timestamp: DateTime<Utc>,
    level: LogLevel,
    context: T,
}

impl<T: Context> LogMessage<T> {
    pub fn new(timestamp: DateTime<Utc>, level: LogLevel, context: T) -> Self {
        LogMessage {
            timestamp,
            level,
            context,
        }
    }
}

impl<T: Context> Display for LogMessage<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}, {}, {}",
            self.timestamp,
            self.level,
            self.context.context_string()
        )
    }
}

#[derive(Clone)]
pub struct Logger<T: Context>(Sender<LogMessage<T>>);

impl<T: 'static + Context> Logger<T> {
    pub fn start(logger_name: &str, message_queue_size: usize) -> Self {
        let mut file = File::create(logger_name).unwrap();
        let (send_chan, recv_chan) = channel::bounded::<LogMessage<T>>(message_queue_size);

        thread::spawn(move || {
            while let Ok(log_message) = recv_chan.recv() {
                let result = file.write_all(log_message.to_string().as_bytes());
                if result.is_err() {
                    eprintln!(
                        "{}, logger could not write log message successfully: {}",
                        Utc::now(),
                        result.unwrap_err()
                    );
                }
            }
        });

        Logger(send_chan)
    }

    pub fn info(&self, context: T) {
        self.0
            .send(LogMessage::new(Utc::now(), LogLevel::Info, context))
            .unwrap();
    }

    pub fn warning(&self, context: T) {
        self.0
            .send(LogMessage::new(Utc::now(), LogLevel::Warning, context))
            .unwrap();
    }

    pub fn error(&self, context: T) {
        self.0
            .send(LogMessage::new(Utc::now(), LogLevel::Error, context))
            .unwrap();
    }
}
