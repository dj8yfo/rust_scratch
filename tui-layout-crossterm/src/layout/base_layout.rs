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

use crate::base_dimens::*;
use bounded_integer::bounded_integer;
use r3bl_rs_utils::ResultCommon;

bounded_integer! {
  /// https://docs.rs/bounded-integer/latest/bounded_integer/index.html#
  pub struct PerCent { 0..100 }
}

/// Direction of the layout of the box.
#[derive(Copy, Clone, Debug)]
pub enum Direction {
  Horiz,
  Vert,
}

impl Default for Direction {
  fn default() -> Direction {
    Direction::Horiz
  }
}

/// A box is a rectangle with a position and size. The direction of the box determines how
/// it's contained elements are positioned.
#[derive(Copy, Clone, Debug, Default)]
pub struct Layout {
  pub direction: Direction,
  pub pos: Position,
  pub size: Size,
  pub width_hint: Option<PerCent>, // TODO: use this to calc box size during layout
  pub height_hint: Option<PerCent>, // TODO: use this to calc box size during layout
}

impl Layout {
  /// Explicitly set the position & size of our box.
  pub fn new_root(
    direction: Direction,
    pos: Position,
    size: Size,
  ) -> Self {
    Self {
      direction,
      pos,
      size,
      width_hint: None,
      height_hint: None,
    }
  }

  /// Actual position and size for our box will be calculated based on provided hints.
  pub fn new(
    direction: Direction,
    width_hint: PerCent,
    height_hint: PerCent,
  ) -> Self {
    Self {
      direction,
      pos: Position::default(),
      size: Size::default(),
      width_hint: Some(width_hint),
      height_hint: Some(height_hint),
    }
  }
}

/// Represents a rectangular area of the terminal screen, and not necessarily the full
/// terminal screen.
#[derive(Clone, Debug, Default)]
pub struct Canvas {
  pub origin_pos: Position,
  pub canvas_size: Size,
  pub layout_stack: Vec<Layout>,
  pub output_commands: Vec<String>, // TODO: String is a placeholder for now, replace w/ enum
}

/// API interface to create nested & responsive layout based UIs.
pub trait LayoutManager {
  // Start and end entire canvas.
  fn start(
    &mut self,
    position: Position,
    size: Size,
  ) -> ResultCommon<()>;
  fn end(&mut self) -> ResultCommon<()>;

  // Start and end a box layout.
  fn start_layout(
    &mut self,
    orientation: Direction,
  ) -> ResultCommon<()>;
  fn end_layout(&mut self) -> ResultCommon<()>;

  // Layout calculations.
  fn get_current_layout(
    &mut self,
    err_msg: &str,
  ) -> ResultCommon<&mut Layout>;
  fn next_position(
    &mut self,
    err_msg: &str,
  ) -> ResultCommon<Position>;

  // Painting operations.
  fn alloc_space_for_paint(
    &mut self,
    size: Size,
  ) -> ResultCommon<()>;
  fn paint(
    &mut self,
    text: String,
  ) -> ResultCommon<()>;
}
