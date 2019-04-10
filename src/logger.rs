use std::convert::{From, Into};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::string::ToString;
use std::sync::mpsc::{self, SendError, Sender};
use std::thread;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct LoggerError(SendError<MessageType>);

impl Display for LoggerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "sending to logging thread failed: {}", self.0)
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

impl From<SendError<MessageType>> for LoggerError {
    fn from(err: SendError<MessageType>) -> Self {
        Self(err)
    }
}

#[derive(Copy, Clone)]
enum LogLevel {
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

#[derive(Clone)]
struct LogMessage {
    timestamp: Instant,
    level: LogLevel,
    event: fn() -> String,
}

impl LogMessage {
    pub fn new(timestamp: Instant, level: LogLevel, event: fn() -> String) -> Self {
        LogMessage {
            timestamp,
            level,
            event,
        }
    }
}

impl Display for LogMessage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}, {}, {}",
            self.timestamp,
            self.level,
            (self.event)()
        )
    }
}

type MessageType = LogMessage;

#[derive(Clone)]
pub struct Logger(Sender<MessageType>);

impl Logger {
    pub fn start(logger_name: &str) -> Self {
        let mut file = File::create(logger_name).unwrap();
        let (send_chan, recv_chan) = mpsc::channel::<MessageType>();

        thread::spawn(move || {
            while let Ok(log_message) = recv_chan.recv() {
                file.write_all(log_message.to_string().as_bytes()).unwrap();
            }
        });

        Logger(send_chan)
    }

    pub fn info(&self, event: fn() -> String) -> Result<(), LoggerError> {
        self.0
            .send(LogMessage::new(Instant::now(), LogLevel::Info, event))
            .map_err(Into::into)
    }

    pub fn warning(&self, event: fn() -> String) -> Result<(), LoggerError> {
        self.0
            .send(LogMessage::new(Instant::now(), LogLevel::Warning, event))
            .map_err(Into::into)
    }

    pub fn error(&self, event: fn() -> String) -> Result<(), LoggerError> {
        self.0
            .send(LogMessage::new(Instant::now(), LogLevel::Error, event))
            .map_err(Into::into)
    }
}
