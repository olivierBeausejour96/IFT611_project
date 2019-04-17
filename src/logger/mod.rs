mod log_level;

use std::convert::{From, Into};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::string::ToString;
use std::sync::mpsc::{self, SendError, Sender};
use std::thread;
use std::time::Instant;

use log_level::LogLevel;

pub trait Context: Send {
    fn context_string(&self) -> String;
}

struct LogMessage<T: Context> {
    timestamp: Instant,
    level: LogLevel,
    context: T,
}

impl<T: Context> LogMessage<T> {
    pub fn new(timestamp: Instant, level: LogLevel, context: T) -> Self {
        LogMessage {
            timestamp,
            level,
            context,
        }
    }
}

impl<T: Context> Display for LogMessage<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}, {}, {}",
            self.timestamp,
            self.level,
            self.context.context_string()
        )
    }
}

#[derive(Clone)]
pub struct Logger<T: Context>(Sender<LogMessage<T>>);

impl<T: 'static + Context> Logger<T> {
    pub fn start(logger_name: &str) -> Self {
        let mut file = File::create(logger_name).unwrap();
        let (send_chan, recv_chan) = mpsc::channel::<LogMessage<T>>();

        thread::spawn(move || {
            while let Ok(log_message) = recv_chan.recv() {
                file.write_all(log_message.to_string().as_bytes()).unwrap();
            }
        });

        Logger(send_chan)
    }

    pub fn info(&self, context: T) -> Result<(), LoggerError> {
        self.0
            .send(LogMessage::new(Instant::now(), LogLevel::Info, context))
            .map_err(Into::into)
    }

    pub fn warning(&self, context: T) -> Result<(), LoggerError> {
        self.0
            .send(LogMessage::new(Instant::now(), LogLevel::Warning, context))
            .map_err(Into::into)
    }

    pub fn error(&self, context: T) -> Result<(), LoggerError> {
        self.0
            .send(LogMessage::new(Instant::now(), LogLevel::Error, context))
            .map_err(Into::into)
    }
}

#[derive(Debug, Clone)]
pub struct LoggerError;

impl Display for LoggerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "sending to logging thread failed, receiver is dead")
    }
}

impl Error for LoggerError {
    fn description(&self) -> &str {
        "sending to logging thread failed, received is dead"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl<T: Context> From<SendError<LogMessage<T>>> for LoggerError {
    fn from(_err: SendError<LogMessage<T>>) -> Self {
        Self {}
    }
}
