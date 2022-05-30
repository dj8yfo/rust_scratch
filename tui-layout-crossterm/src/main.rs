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

use r3bl_rs_utils::CommonResult;

// Attach source files.
mod log_mod;
mod crossterm_mod;

// Import everything from attached source files.
use crossterm_mod::*;
use log_mod::*;

#[tokio::main]
async fn main() -> CommonResult<()> {
  emit_log_entries().await?;
  emit_crossterm_commands().await?;
  Ok(())
}
