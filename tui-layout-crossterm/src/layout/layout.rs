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

use crate::dimens::*;
use r3bl_rs_utils::Builder;

/// Direction of the layout of the box.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
  Horizontal,
  Vertical,
}

impl Default for Direction {
  fn default() -> Direction {
    Direction::Horizontal
  }
}

/// A box is a rectangle with a position and size. The direction of the box determines how
/// it's contained elements are positioned.
#[derive(Clone, Default, Builder)]
pub struct Layout {
  pub id: String,
  pub dir: Direction,
  pub origin_pos: Option<Position>,
  pub bounds_size: Option<Size>,
  pub req_size_percent: Option<RequestedSizePercent>,
  pub layout_cursor_pos: Option<Position>,
  pub content_cursor_pos: Option<Position>,
}

impl Layout {
  /// Explicitly set the position & size of our box.
  pub fn make_root_layout(
    id: String,
    canvas_size: Size,
    origin_pos: Position,
    width_pc: Percent,
    height_pc: Percent,
    dir: Direction,
  ) -> Layout {
    LayoutBuilder::new()
      .set_id(id)
      .set_dir(dir)
      .set_origin_pos(origin_pos.as_some())
      .set_bounds_size(
        Size::new(
          calc_percentage(width_pc, canvas_size.width),
          calc_percentage(height_pc, canvas_size.height),
        )
        .as_some(),
      )
      .set_req_size_percent(RequestedSizePercent::new(width_pc, height_pc).as_some())
      .set_layout_cursor_pos(origin_pos.as_some())
      .build()
  }

  /// Actual position and size for our box will be calculated based on provided hints.
  pub fn make_layout(
    id: String,
    dir: Direction,
    container_bounds: Size,
    origin_pos: Position,
    width_pc: Percent,
    height_pc: Percent,
  ) -> Self {
    LayoutBuilder::new()
      .set_id(id)
      .set_dir(dir)
      .set_origin_pos(origin_pos.as_some())
      .set_bounds_size(
        Size::new(
          calc_percentage(width_pc, container_bounds.width),
          calc_percentage(height_pc, container_bounds.height),
        )
        .as_some(),
      )
      .set_req_size_percent(RequestedSizePercent::new(width_pc, height_pc).as_some())
      .build()
  }
}

/// Pretty print `Layout`.
#[derive(Clone, Copy, Debug)]
enum FormatMsg {
  None,
}

/// Pretty print `Layout`.
macro_rules! format_option {
  ($opt:expr) => {
    match ($opt) {
      Some(v) => v,
      None => &FormatMsg::None,
    }
  };
}

/// Pretty print `Layout`.
impl std::fmt::Debug for Layout {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("Layout")
      .field("id", &self.id)
      .field("dir", &self.dir)
      .field(
        "origin_pos",
        format_option!(&self.origin_pos),
      )
      .field(
        "bounds_size",
        format_option!(&self.bounds_size),
      )
      .field(
        "req_size_percent",
        format_option!(&self.req_size_percent),
      )
      .field(
        "layout_cursor_pos",
        format_option!(&self.layout_cursor_pos),
      )
      .field(
        "content_cursor_pos",
        format_option!(&self.content_cursor_pos),
      )
      .finish()
  }
}
