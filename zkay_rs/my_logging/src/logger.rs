// import datetime
// import json
// import logging.config
// import os
// from logging import addLevelName

// # current time
use zkay_config::config::CFG;
use crate::log_context::FULL_LOG_CONTEXT;

// timestamp = "{:%Y-%m-%d_%H-%M-%S}".format(datetime.datetime.now())

fn timestamp() -> String {
    use chrono::Local;
    Local::now().format("%Y-%m-%d_%H:%M:%S").to_string()
}
// # shutdown current logger (useful for debugging, ...)
// def shutdown(handler_list=None):
//     if handler_list is None:
//         handler_list = []
//     logging.shutdown(handler_list)

// ##########################
// # add log level for DATA #
// ##########################
// # LOG LEVELS
// # existing:
// # CRITICAL = 50
// # ERROR = 40
// # WARNING = 30
// # INFO = 20
// # DEBUG = 10
const DATA: i32 = 5;
// addLevelName(DATA, "DATA")
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
fn set_log() -> Result<(), SetLoggerError> {
    log::set_logger(&CONSOLE_LOGGER)?;
    log::set_max_level(LevelFilter::Info);
    Ok(())
}
use serde_json::json;
pub fn data(key: &str, value: &str) -> String
// """
    // Log (key, value) to log-level DATA
    // """
{
    let d =
        json!({"key": key, "value": value, "context": FULL_LOG_CONTEXT.lock().unwrap().clone()});
    log::debug!("{DATA},{}", d);
    format!("{DATA},{}", d)
}

// def get_log_dir(parent_dir, label):
//     """
//     Convenience function for getting a log directory
//     """
//     d = os.path.join(parent_dir, label)

//     # ensure log directory exists
//     if not os.path.exists(d):
//         os.makedirs(d)

//     return d

// def get_log_file(label="default", parent_dir=None, filename="log", include_timestamp=True):
//     if parent_dir is None:
//         parent_dir = os.path.realpath(cfg.log_dir)
//     if label is None:
//         log_dir = parent_dir
//     else:
//         log_dir = get_log_dir(parent_dir, label)

//     if include_timestamp:
//         filename += "_" + timestamp
//     log_file = os.path.join(log_dir, filename)

//     return log_file

// def prepare_logger(log_file=None, silent=True):
//     # shutdown previous logger (if one was registered)
//     shutdown()

//     # set log dir and console log level
//     if log_file is None:
//         log_file = get_log_file()

//     console_loglevel = "WARNING"

//     if not silent:
//         print(f"Saving logs to {log_file}*...")

//     # set default logging settings
//     default_logging = {
//         "version": 1,
//         "formatters": {
//             "standard": {
//                 "format": "%(asctime)s [%(levelname)s]: %(message)s",
//                 "datefmt": "%Y-%m-%d_%H-%M-%S"
//             },
//             "minimal": {
//                 "format": "%(message)s"
//             },
//         },
//         "filters": {
//             "onlydata": {
//                 "()": OnlyData
//             }
//         },
//         "handlers": {
//             "default": {
//                 "level": console_loglevel,
//                 "formatter": "standard",
//                 "class": "logging.StreamHandler",
//             },
//             "fileinfo": {
//                 "level": "INFO",
//                 "formatter": "standard",
//                 "filename": log_file + "_info.log",
//                 "mode": "w",
//                 "class": "logging.FileHandler",
//             },
//             "filedebug": {
//                 "level": "DEBUG",
//                 "formatter": "standard",
//                 "filename": log_file + "_debug.log",
//                 "mode": "w",
//                 "class": "logging.FileHandler",
//             },
//             "filedata": {
//                 "level": "DATA",
//                 "formatter": "minimal",
//                 "filename": log_file + "_data.log",
//                 "mode": "w",
//                 "class": "logging.FileHandler",
//                 "filters": ["onlydata"]
//             }
//         },
//         "loggers": {
//             "": {
//                 "handlers": ["default", "fileinfo", "filedebug", "filedata"],
//                 "level": 0
//             }
//         }
//     }
//     logging.config.dictConfig(default_logging)

// class OnlyData(logging.Filter):

//     def filter(self, record):
//         # print(record.__dict__)
//         return record.levelno == DATA

// # register a default logger (can be overwritten later)
// prepare_logger()
