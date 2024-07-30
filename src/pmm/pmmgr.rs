use tokio::{sync::mpsc::channel, task::JoinSet};
use command_group::Signal;

use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
  config_path: PathBuf  
}

mod cfg;
mod log;
mod proc;
mod sig;

#[tokio::main]
async fn main() {
  let args = Cli::parse();
  let logger = log::Logger::new();
  
  println!("{:?}", args.config_path);

  match run(args, logger).await {
    Ok(success) => {
      if success {
        std::process::exit(0);
      } else {
        std::process::exit(1);
      }
    },
    Err(err) => {
      eprintln!("{}", err);
      std::process::exit(1);
    },
  }
}

/// Runs the main processing logic.
///
/// This function takes the command line arguments (`args`) and a logger (`logger`) as input,
/// and returns a `Result` indicating whether the processing was successful or not.
async fn run(args: Cli, mut logger: log::Logger) -> anyhow::Result<bool> {
  let config = cfg::Config::load(&args.config_path)?;

  for (name, _) in config.processes.iter() {
    logger.register_name(name);
  }

  let mut tasks = JoinSet::new();

  let (exit_tx, mut exit_rx) = channel(config.processes.len());
  let mut signal_senders = vec![];

  for (name, process) in config.processes.iter() {
    let (signal_tx, signal_rx) = channel(1);

    let logger = logger.clone();
    let name = name.clone();
    let proc_cfg = process.clone();

    tasks.spawn(async move {
      let res = proc::run(&name, proc_cfg, signal_rx, &logger).await;

      match res {
        Ok(success) => success,
        Err(err) => {
          logger.log(log::LogRecord::Controller {
            stream: log::LogStream::Stderr,
            record: log::ControllerLogRecord::new(err.to_string()),
          });

          // TODO: Solmon this will not exist on any specific process error
          false
        },
      };

      //exit_tx.send(success).await.unwrap();
    });

    signal_senders.push(signal_tx);
  }

  {
    let logger = logger.clone();
    let signal_senders = signal_senders.clone();
    let (signal_tx, mut signal_rx) = channel(1);

    tokio::spawn(async move {
      sig::listen(signal_tx, &logger).await.unwrap();
    });

    tokio::spawn(async move {
      while let Some(sig) = signal_rx.recv().await {
        for sender in signal_senders.iter() {
          println!("sending signal: {}", sig);
          sender.send(sig).await.expect("signal channel closed");
        }
      }
    });
  }

  let success = exit_rx.recv().await.expect("exit channel closed");

  for sender in signal_senders.iter() {
    if let Err(_) = sender.send(Signal::SIGTERM).await {
      // do nothing, the process was already terminated
    }
  }

  while let Some(res) = tasks.join_next().await {
    res.expect("task panicked");
  }

  Ok(success)
}