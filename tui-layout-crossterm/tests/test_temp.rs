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

const DEBUG: bool = false;

type ThunkResult<T> = Result<T, Box<ThunkError>>;
type ThunkFunction<T> = fn() -> ThunkResult<T>;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct ThunkError {
  err_type: ThunkErrorType,
}

#[derive(Debug, Clone, Copy)]
pub enum ThunkErrorType {
  ComputeFieldFnError,
}

#[derive(Debug)]
enum ThunkState<T>
where
  T: Clone + Copy,
{
  NotComputedYet,
  ComputedResultingInError(ThunkError),
  ComputedResultingInValue(T),
}

#[derive(Debug)]
struct Thunk<T>
where
  T: Clone + Copy,
{
  pub field: ThunkState<T>,
  pub compute_field_value_fn: ThunkFunction<T>,
}

impl<T> Thunk<T>
where
  T: Clone + Copy,
{
  pub fn new(compute_field_value_fn: ThunkFunction<T>) -> Self {
    Self {
      field: ThunkState::NotComputedYet,
      compute_field_value_fn,
    }
  }

  pub fn access_field(&mut self) -> ThunkResult<T> {
    if let ThunkState::NotComputedYet = self.field {
      let computed_field_value_result = (self.compute_field_value_fn)();
      match computed_field_value_result {
        Ok(computed_field_value) => {
          if DEBUG {
            println!("once - computing value");
          }
          self.field = ThunkState::ComputedResultingInValue(computed_field_value.clone());
          return Ok(computed_field_value);
        }
        Err(e) => {
          if DEBUG {
            println!("once - problem computing value");
          }
          let e_clone = *e.clone();
          self.field = ThunkState::ComputedResultingInError(e_clone);
          return Err(e);
        }
      }
    }

    if let ThunkState::ComputedResultingInValue(value) = self.field {
      if DEBUG {
        println!("returning cached value");
      }
      return Ok(value.clone());
    }

    if let ThunkState::ComputedResultingInError(e) = self.field {
      if DEBUG {
        println!("returning cached error");
      }
      return Err(Box::new(e));
    }

    panic!("unreachable");
  }
}

#[test]
fn test_name() {
  let mut thunk = Thunk::new(|| Ok(1));

  // First access to the field will trigger the computation.
  {
    let result = thunk.access_field();
    if result.is_err() {
      panic!("error");
    } else {
      let field_value = result.unwrap();
      assert_eq!(field_value, 1);
    }
  }

  // Subsequent accesses to the field will return the cached value.
  {
    let result = thunk.access_field();
    if result.is_err() {
      panic!("error");
    } else {
      let field_value = result.unwrap();
      assert_eq!(field_value, 1);
    }
  }
}
