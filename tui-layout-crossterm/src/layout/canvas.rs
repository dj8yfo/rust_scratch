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

use crate::layout::*;
use crate::*;
use r3bl_rs_utils::{with, CommonResult};

/// Represents a rectangular area of the terminal screen, and not necessarily the full
/// terminal screen.
#[derive(Clone, Debug, Default)]
pub struct Canvas {
  pub origin_pos: Position,
  pub canvas_size: Size,
  pub layout_stack: Vec<Layout>,
  pub stylesheet: Stylesheet,
  pub output_commands: Vec<String>,
}

impl LayoutManager for Canvas {
  fn set_stylesheet(
    &mut self,
    stylesheet: Stylesheet,
  ) {
    self.stylesheet = stylesheet;
  }

  fn get_stylesheet(&self) -> &Stylesheet {
    &self.stylesheet
  }

  fn start(
    &mut self,
    bounds_props: CanvasProps,
  ) -> CommonResult<()> {
    // Expect layout_stack to be empty!
    if !self.is_layout_stack_empty() {
      LayoutError::new_err_with_msg(
        LayoutErrorType::MismatchedStart,
        LayoutError::format_msg_with_stack_len(&self.layout_stack, "Layout stack should be empty"),
      )?
    }
    let CanvasProps { pos, size } = bounds_props;
    self.origin_pos = pos;
    self.canvas_size = size;
    Ok(())
  }

  fn end(&mut self) -> CommonResult<()> {
    // Expect layout_stack to be empty!
    if !self.is_layout_stack_empty() {
      LayoutError::new_err_with_msg(
        LayoutErrorType::MismatchedEnd,
        LayoutError::format_msg_with_stack_len(&self.layout_stack, "Layout stack should be empty"),
      )?
    }
    Ok(())
  }

  fn start_layout(
    &mut self,
    layout_props: LayoutProps,
  ) -> CommonResult<()> {
    {
      match self.is_layout_stack_empty() {
        true => self.add_root_layout(layout_props),
        false => self.add_normal_layout(layout_props),
      }?
    }
    Ok(())
  }

  fn end_layout(&mut self) -> CommonResult<()> {
    // Expect layout_stack not to be empty!
    if self.is_layout_stack_empty() {
      LayoutError::new_err_with_msg(
        LayoutErrorType::MismatchedEndLayout,
        LayoutError::format_msg_with_stack_len(
          &self.layout_stack,
          "Layout stack should not be empty",
        ),
      )?
    }
    self.pop_layout();
    Ok(())
  }

  fn print(
    &mut self,
    text_vec: Vec<&str>,
  ) -> CommonResult<()> {
    with! {
      self.get_current_layout()?,
      as current_layout,
      run {
        let mut pos:Position = match current_layout.content_cursor_pos {
          Some(value) => value,
          None => Position::new(0, 0),
        };
        pos.add_y(text_vec.len());
        current_layout.content_cursor_pos = Some(pos);
      }
    };
    Ok(())
  }
}

impl PerformLayoutAndPositioning for Canvas {
  fn is_layout_stack_empty(&self) -> bool {
    self.layout_stack.is_empty()
  }

  fn push_layout(
    &mut self,
    layout: Layout,
  ) {
    self.layout_stack.push(layout);
  }

  fn pop_layout(&mut self) {
    self.layout_stack.pop();
  }

  /// Calculate and return the position of where the next layout can be added to the
  /// stack. This updates the `layout_cursor_pos` of the current layout.
  fn calc_next_layout_cursor_pos(
    &mut self,
    allocated_size: Size,
  ) -> CommonResult<Position> {
    let current_layout = self.get_current_layout()?;
    let layout_cursor_pos = current_layout.layout_cursor_pos;

    let layout_cursor_pos = unwrap_or_err! {
      layout_cursor_pos,
      LayoutErrorType::ErrorCalculatingNextLayoutPos
    };

    let new_pos: Position = layout_cursor_pos + allocated_size;

    // Adjust `new_pos` using Direction.
    let new_pos: Position = match current_layout.dir {
      Direction::Vertical => new_pos * Pair::new(0, 1),
      Direction::Horizontal => new_pos * Pair::new(1, 0),
    };

    Ok(new_pos)
  }

  fn update_layout_cursor_pos(
    &mut self,
    new_pos: Position,
  ) -> CommonResult<()> {
    self.get_current_layout()?.layout_cursor_pos = new_pos.as_some();
    Ok(())
  }

  /// Get the last layout on the stack (if none found then return Err).
  fn get_current_layout(&mut self) -> CommonResult<&mut Layout> {
    // Expect layout_stack not to be empty!
    if self.layout_stack.is_empty() {
      LayoutError::new_err(LayoutErrorType::LayoutStackShouldNotBeEmpty)?
    }
    Ok(self.layout_stack.last_mut().unwrap())
  }

  /// 🌳 Root: Handle first layout to add to stack, explicitly sized & positioned.
  fn add_root_layout(
    &mut self,
    props: LayoutProps,
  ) -> CommonResult<()> {
    let LayoutProps {
      id,
      dir,
      req_size,
      styles,
    } = props;
    let RequestedSizePercent {
      width: width_pc,
      height: height_pc,
    } = req_size;
    self.push_layout(Layout::make_root_layout(
      id.to_string(),
      self.canvas_size,
      self.origin_pos,
      width_pc,
      height_pc,
      dir,
      styles,
    ));
    Ok(())
  }

  /// 🍀 Non-root: Handle layout to add to stack. Position and Size will be calculated.
  fn add_normal_layout(
    &mut self,
    props: LayoutProps,
  ) -> CommonResult<()> {
    let LayoutProps {
      id,
      dir,
      req_size,
      styles,
    } = props;
    let RequestedSizePercent {
      width: width_pc,
      height: height_pc,
    } = req_size;
    let container_bounds = unwrap_or_err! {
      self.get_current_layout()?.bounds_size,
      LayoutErrorType::ContainerBoundsNotDefined
    };

    let requested_size_allocation = Size::new(
      calc_percentage(width_pc, container_bounds.width),
      calc_percentage(height_pc, container_bounds.height),
    );

    let old_position = unwrap_or_err! {
      self.get_current_layout()?.layout_cursor_pos,
      LayoutErrorType::LayoutCursorPositionNotDefined
    };

    let new_pos = self.calc_next_layout_cursor_pos(requested_size_allocation)?;
    self.update_layout_cursor_pos(new_pos)?;

    self.push_layout(Layout::make_layout(
      id.to_string(),
      dir,
      container_bounds,
      old_position,
      width_pc,
      height_pc,
      styles,
    ));
    Ok(())
  }
}
