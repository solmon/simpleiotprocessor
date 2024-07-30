use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Logger {
  names: Vec<String>,
}

#[derive(Debug)]
pub enum LogRecord {
  Controller {
    stream: LogStream,
    record: ControllerLogRecord,
  },
  Process {
    stream: LogStream,
    record: ProcessLogRecord,
  },
}

#[derive(Debug)]
pub enum LogStream {
  Stdout,
  Stderr,
}

#[derive(Debug, Serialize)]
pub struct ControllerLogRecord {
  pub time: DateTime<Utc>,
  pub message: String,
}

#[derive(Debug)]
pub struct ProcessLogRecord {
  pub name: String,
  pub line: String,
}

impl Logger {
  /// Creates a new instance of `Logger`.
  pub fn new() -> Self {
    Self {
      names: vec![
        String::from("controller"),
      ],
    }
  }

  /// Registers a new name in the logger.
  ///
  /// # Arguments
  ///
  /// * `name` - The name to register.
  pub fn register_name(&mut self, name: &str) {
    let name = format!("proc.{}", name);
    let pos = self.search_name(&name).unwrap_or_else(|pos| pos);
    self.names.insert(pos, name);
  }

  /// Searches for a name in the logger.
  ///
  /// # Arguments
  ///
  /// * `name` - The name to search for.
  ///
  /// # Returns
  ///
  /// * `Result<usize, usize>` - The index of the found name, or the index where the name should be inserted.
  fn search_name(&self, name: &str) -> Result<usize, usize> {
    self.names.binary_search_by(|s| s.len().cmp(&name.len()).reverse())
  }

  /// Returns the length of the longest name in the logger.
  ///
  /// # Returns
  ///
  /// * `usize` - The length of the longest name.
  fn longest_name(&self) -> usize {
    self.names[0].len()
  }

  /// Logs a record to the appropriate stream.
  ///
  /// # Arguments
  ///
  /// * `record` - The log record to be logged.
  pub fn log(&self, record: LogRecord) {
    let (name, stream, line) = match record {
      LogRecord::Controller { stream, record } => {
        let name = String::from("controller");
        let line = alogfmt::to_string(&record).unwrap_or_default();
        (name, stream, line)
      },
      LogRecord::Process { stream, record } => {
        let name = format!("proc.{}", record.name);
        self.search_name(&name).expect("process name not found");
        (name, stream, record.line)
      }
    };

    let width = self.longest_name();
    match stream {
      LogStream::Stdout => {
        let name = format!("{}[stdout]", name);
        println!("{:width$} | {}", name, line, width = width + 8);
      },
      LogStream::Stderr => {
        let name = format!("{}[stderr]", name);
        println!("{:width$} | {}", name, line, width = width + 8);
      },
    }
  }
}

impl ControllerLogRecord {
  pub fn new(message: String) -> Self {
    Self {
      time: Utc::now(),
      message,
    }
  }
}