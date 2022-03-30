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

use syn::{punctuated::Punctuated,
          token::Comma,
          GenericArgument,
          Ident,
          PathArguments::AngleBracketed,
          Type};

pub trait TypeExt {
  fn has_ident(&self) -> bool;
  fn get_ident(&self) -> Option<Ident>;
  fn has_angle_bracketed_generic_args(&self) -> bool;
  fn get_angle_bracketed_generic_args_result(
    &self
  ) -> Result<Punctuated<GenericArgument, Comma>, ()>;
  fn get_angle_bracketed_generic_args_idents_result(&self) -> Result<Vec<Ident>, ()>;
}

impl TypeExt for syn::Type {
  fn has_ident(&self) -> bool {
    match self {
      Type::Path(ref type_path) => {
        let path = &type_path.path;
        let ident = &path.segments.first();
        ident.is_some()
      }
      _ => false,
    }
  }

  fn get_ident(&self) -> Option<Ident> {
    match self {
      Type::Path(ref type_path) => {
        let path = &type_path.path;
        let ident = &path
          .segments
          .first()
          .unwrap()
          .ident;
        Some(ident.clone())
      }
      _ => None,
    }
  }

  /// True if self.type_path.path.segments.first().arguments.args.len() to be > 0.
  fn has_angle_bracketed_generic_args(&self) -> bool {
    match self.get_angle_bracketed_generic_args_result() {
      Ok(generic_args) => generic_args.len() > 0,
      Err(_) => false,
    }
  }

  /// Ok if self.type_path.path.segments.first().arguments.args exists.
  fn get_angle_bracketed_generic_args_result(
    &self
  ) -> Result<Punctuated<GenericArgument, Comma>, ()> {
    if let Type::Path(ref type_path) = self {
      let path = &type_path.path;
      let path_arguments = &path
        .segments
        .first()
        .unwrap()
        .arguments;

      if let AngleBracketed(ref angle_bracketed_generic_arguments) = path_arguments {
        return Ok(
          angle_bracketed_generic_arguments
            .args
            .clone(),
        );
      }
    }

    Err(())
  }

  fn get_angle_bracketed_generic_args_idents_result(&self) -> Result<Vec<Ident>, ()> {
    match self.get_angle_bracketed_generic_args_result() {
      Ok(generic_args) => {
        let mut idents = Vec::new();
        for generic_arg in generic_args {
          match generic_arg {
            GenericArgument::Type(ref type_arg) => {
              if let Type::Path(ref type_path) = type_arg {
                let path = &type_path.path;
                let ident = &path
                  .segments
                  .first()
                  .unwrap()
                  .ident;
                idents.push(ident.clone());
              }
            }
            _ => {}
          }
        }
        Ok(idents)
      }
      Err(_) => Err(()),
    }
  }
}
