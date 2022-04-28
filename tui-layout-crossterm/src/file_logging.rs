/*
 *   Copyright (c) 2022 Nazmul Idris
 *   All rights reserved.

 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at

 *   http://www.apache.org/licenses/LICENSE-2.0

 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
*/

use log::LevelFilter;
use std::{error::Error, sync::Once, io::Error as IoError};
use simplelog::*;
use std::fs::File;

const FILE_PATH: &str = "log.txt";

static mut FILE_LOGGER_INIT_OK: bool = false;
static FILE_LOGGER_INIT_FN: Once = Once::new();

pub type ResultCommon<T> = std::result::Result<T, Box<dyn Error>>;

/// # Docs
/// - [`log::info!`], [`log::warn!`], [`log::error!`]: https://docs.rs/log/latest/log/
///
/// # Example
/// ```ignore
/// fn run() -> ResultCommon<()> {
///   log!(INFO, "This is a info message");
///   log!(WARN, "This is a warning message");
///   log!(ERROR, "This is a error message");
///   Ok(())
/// }
/// ```
#[macro_export]
macro_rules! log {
  (INFO, $($arg:tt)*) => {{
    init_file_logger_once()?;
    log::info!($($arg)*);
  }};
  (WARN, $($arg:tt)*) => {{
    init_file_logger_once()?;
    log::warn!($($arg)*);
  }};
  (ERROR, $($arg:tt)*) => {{
    init_file_logger_once()?;
    log::error!($($arg)*);
  }};
}

/// Simply open the [`FILE_PATH`] file and write the log message to it. This will be
/// opened once per session (i.e. program execution). It is destructively opened, meaning
/// that it will be rewritten when used in the next session.
///
/// # Docs
/// - [`CombinedLogger`], [`WriteLogger`], [`ConfigBuilder`]: https://github.com/drakulix/simplelog.rs
/// - [`format_description!`]: https://time-rs.github.io/book/api/format-description.html
pub fn init_file_logger_once() -> ResultCommon<()> {
  // Run the lambda once & save bool to static `FILE_LOGGER_INIT_OK`.
  FILE_LOGGER_INIT_FN.call_once(|| actually_init_file_logger());

  // Use saved bool in static `FILE_LOGGER_INIT_OK` to throw error if needed.
  unsafe {
    match FILE_LOGGER_INIT_OK {
      true => Ok(()),
      false => Err(Box::new(IoError::new(
        std::io::ErrorKind::Other,
        format!("Failed to initialize file logger {}", FILE_PATH),
      ))),
    }
  }
}

fn actually_init_file_logger() {
  let new_file_result = File::create(FILE_PATH);
  if let Ok(new_file) = new_file_result {
    let logger_init_result = CombinedLogger::init(vec![WriteLogger::new(
      LevelFilter::Info,
      new_config(),
      new_file,
    )]);
    if let Ok(_) = logger_init_result {
      unsafe { FILE_LOGGER_INIT_OK = true }
    }
  }
}

/// Try to make a [`Config`] with local timezone offset. If that fails, return a default.
fn new_config() -> Config {
  match ConfigBuilder::new().set_time_offset_to_local() {
    Ok(builder) => {
      // To use RFC2822 instead of custom: `builder.set_time_format_rfc2822();`
      builder
        .set_time_format_custom(format_description!("[hour repr:12]:[minute] [period]"));
      builder.build()
    }
    Err(_) => Config::default(),
  }
}
